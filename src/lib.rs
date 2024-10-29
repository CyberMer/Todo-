// lib.rs
use rusqlite::{Connection, Result, params};
use prettytable::{Table, Row, Cell, format};

pub struct Todo {
    pub name: String,
    pub date_added: String,
    pub is_done: bool,
    pub priority: i32,
}

impl Todo {
    pub fn add(conn: &Connection, name: &str, date_added: &str, priority: i32) -> Result<()> {
        conn.execute(
            "INSERT INTO todo (name, date_added, is_done, priority) VALUES (?, ?, 0, ?)",
            params![name, date_added, priority],
        )?;
        Ok(())
    }

    pub fn set_done(conn: &Connection, name: &str, done: bool) -> Result<()> {
        conn.execute(
            "UPDATE todo SET is_done = ? WHERE name = ?",
            params![done as i32, name],
        )?;
        Ok(())
    }

    pub fn set_priority(conn: &Connection, name: &str, priority: i32) -> Result<()> {
        conn.execute(
            "UPDATE todo SET priority = ? WHERE name = ?",
            params![priority, name],
        )?;
        Ok(())
    }

    pub fn print_todos(conn: &Connection) -> Result<()> {
        // Table for todos not done
        let mut undone_table = Table::new();
        undone_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        undone_table.set_titles(Row::new(vec![
            Cell::new("Name"),
            Cell::new("Date Added"),
            Cell::new("Priority"),
        ]));

        let mut stmt = conn.prepare("SELECT name, date_added, priority FROM todo WHERE is_done = 0 ORDER BY priority ASC;")?;
        let todos_iter = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        for todo in todos_iter {
            let (name, date_added, priority): (String, String, i32) = todo?;
            undone_table.add_row(Row::new(vec![
                Cell::new(&name),
                Cell::new(&date_added),
                Cell::new(&priority.to_string()),
            ]));
        }

        // Table for todos done
        let mut done_table = Table::new();
        done_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        done_table.set_titles(Row::new(vec![
            Cell::new("Name"),
            Cell::new("Date Added"),
            Cell::new("Priority"),
        ]));

        let mut stmt = conn.prepare("SELECT name, date_added, priority FROM todo WHERE is_done = 1 ORDER BY priority ASC;")?;
        let todos_iter = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        for todo in todos_iter {
            let (name, date_added, priority): (String, String, i32) = todo?;
            done_table.add_row(Row::new(vec![
                Cell::new(&name),
                Cell::new(&date_added),
                Cell::new(&priority.to_string()),
            ]));
        }

        println!("Todos - Not Done:");
        undone_table.printstd();

        println!("\nTodos - Done:");
        done_table.printstd();

        Ok(())
    }

}
