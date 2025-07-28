use std::vec;

use pretty_sqlite::{print_rows, print_table};
use rusqlite::{types::Value, Connection, ToSql};
use sqlite_db::db_utils::create_schema;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //memory sqlite
    let conn = Connection::open_in_memory()?;
    //create schema
    create_schema(&conn)?;
    //seed users
    let names = &["Jen", "Mike", "Paul", "Pierre"];
    for name in names.iter() {
        let org_id: Option<i64> = None;
        conn.execute(
            "INSERT INTO person(name,org_id,yob) 
        VALUES (?1,?2,?3)",
            (name, &org_id, &2000),
        )?;
    }

    //udpate query dynamic
    let nv_list: Vec<(String, Value)> = vec![
        ("org_id".to_string(), Value::Integer(123)),
        ("name".to_string(), Value::Text("New Name".to_string())),
    ];

    let (cols, vals): (Vec<String>, Vec<Value>) = nv_list.into_iter().unzip();
    let cols = cols
        .iter()
        .map(|col| format!("\"{col}\"=?"))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!("UPDATE person SET {cols}");
    let mut values: Vec<&dyn ToSql> = vals.iter().map(|val| val as &dyn ToSql).collect();
    //build the where clause

    let sql = format!("{sql} WHERE id =?");
    let person_id = Value::Integer(1);
    values.push(&person_id);
    //values are ref to vec :&vec to get the value that point to  we need deref operator *
    //deref value *values to get the value then get the ref of this value &*values
    let num_of_rows = conn.execute(&sql, &*values)?;
    println!("number of rows updated : {num_of_rows}");
    print_table(&conn, "person")?;

    Ok(())
}
