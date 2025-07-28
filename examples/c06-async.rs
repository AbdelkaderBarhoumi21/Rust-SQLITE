use pretty_sqlite::print_table;
use rusqlite::Connection;
use serde_json::json;
use sqlite_db::db_utils::create_schema;

const DB_PATH: &str = "my_db.db3";
//Additionally, SQLite cannot be connected to increase the connection efficiency with more simultaneous connections.
//In production, it will be preferable to use a pool of connections
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open(DB_PATH)?;
    //create schema 
    create_schema(&conn)?;
    let names = &["Jen", "Mike"];
    for name in names{
        for i in 1..10{
            let name =format!("{name}-{i}");
            let _res=tokio::task::spawn(async move{
          //The conn inside the async task spawns separate connections for concurrent tasks. Each task manages its own connection for thread-safety
        //However, this might lead to a race condition or locking issues if multiple threads access the database at the same time. Ideally
        let conn = Connection::open(DB_PATH).map_err(|err| err.to_string())?;
        		conn.execute(
					"INSERT INTO person (name, yob) 
				               VALUES (?1, ?2)",
					(name, &2000),
				)
				.map_err(|err| err.to_string())

    }).await?;
        }

    } 

    //This is necessary because the connection that was used for inserting data (inside the asynchronous tasks)
    //final print 
    let conn=Connection::open(DB_PATH)?;
    print_table(&conn, "person")?;
    Ok(())
    //For instance, using something like diesel or rusqlite with a connection pool (e.g., r2d2) would manage these connections automatically,
    //allowing you to reuse the same connection for querying after the insertions without needing to open a new connection.
}
