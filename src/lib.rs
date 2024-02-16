use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Quotes {
    words: String,
}

pub fn said() -> Result<()> {
    let path = "./db.sqlite";
    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT * FROM quotes")?;
    let quotes_iter = stmt.query_map([], |row| Ok(Quotes { words: row.get(0)? }))?;

    for quote in quotes_iter {
        println!("J.Cole said: {:?}", quote.unwrap().words);
    }
    Ok(())
}
