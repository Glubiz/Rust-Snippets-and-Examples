use rusqlite::{params, Connection, Result};

struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn main() -> Result<()> {
    let conn = Connection::open("my_db.sqlite3")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS people (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)",
        [],
    )?;

    conn.execute("INSERT INTO people (name, age) VALUES (?1, ?2)", params!["Alice", 30])?;
    conn.execute("INSERT INTO people (name, age) VALUES (?1, ?2)", params!["Bob", 28])?;

    let mut stmt = conn.prepare("SELECT id, name, age FROM people")?;
    let people = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            age: row.get(2)?,
        })
    })?;

    for person in people {
        println!("Found person: {:?}", person.unwrap());
    }

    Ok(())
}
