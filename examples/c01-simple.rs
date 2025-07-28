use pretty_sqlite::print_table;
use rusqlite::Connection;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //create a db in memory volatile stocked into RAM
    let conn = Connection::open_in_memory()?;
    //---Create Shce;q
    conn.execute(
        "CREATE TABLE person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            yob INTEGER, --year of birth
            data BLOB
        ) STRICT",
        (),
    )?;
    //--insert
    //Ok us strict mode
    conn.execute(
        "INSERT INTO person (name,yob) VALUES(?1,?2)",
        ("jen", &2020),
    )?;
    //select
    let select_sql = "SELECT person.id,person.name,person.yob
    FROM person 
    WHERE yob >:yob";
    let mut stmt = conn.prepare(select_sql)?; // return statment object verify SQL syntax
    let mut rows = stmt.query(&[(":yob", &1900)])?; //return row object and tuple (name,value) for params
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        let yob: i32 = row.get(2)?;
        println!("Name : {name}");
        println!("yob ; {yob}");
        println!("Row : {row:?}");
    }
    print_table(&conn, "person")?;

    Ok(())
}
