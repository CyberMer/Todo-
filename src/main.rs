// main.rs
use rusqlite::{Connection, Result};
use std::env;
use todo::Todo; // Ensure your library is named appropriately in your Cargo.toml

fn main() -> Result<()> {
    let conn = Connection::open("todos.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            name TEXT NOT NULL,
            date_added TEXT NOT NULL,
            is_done INTEGER NOT NULL,
            priority INTEGER
        )",
        [],
    )?;

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("add") if args.len() > 4 => {
            let name = &args[2];
            let priority: i32 = args[4].parse().unwrap_or(1);
            let current_date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            Todo::add(&conn, name, &current_date, priority)?;
            println!("Added new todo: '{}'", name);
        },
        Some("done") if args.len() > 2 => {
            let name = &args[2];
            Todo::set_done(&conn, name, true)?;
            println!("Marked '{}' as done", name);
        },
        Some("priority") if args.len() > 3 => {
            let name = &args[2];
            let priority: i32 = match args[3].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Invalid priority: must be an integer.");
                    return Err(rusqlite::Error::InvalidQuery); // Or another appropriate error
                }
            };
            Todo::set_priority(&conn, name, priority)?;
            println!("Set priority for '{}' to {}", name, priority);
        },
        Some("list") => {
            Todo::print_todos(&conn)?;
        },
        _ => println!("Usage: todo <command> [<args>]\nCommands are: add <name> <priority>, done <name>, priority <name> <new priority>, list"),
    }

    Ok(())
}
