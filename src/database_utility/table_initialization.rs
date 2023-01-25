use rusqlite::{self, Connection};

pub fn table_initialization (db: String){
    let conn = Connection::open(db).unwrap();
    match conn.execute(
        "create table if not exists conversation (
             id integer primary key,
             title text not null unique
         )",
         []
    ) 
    {
        Ok(..) => println!("Successfully initialized conversation table"),
        Err(error) => println!("{}", error),
    }
}