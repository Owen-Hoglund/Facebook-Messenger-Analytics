use rusqlite::{self, Connection};

pub fn table_initialization (db: &str){
    let conn = Connection::open(db).expect("Failed to open database");
    match conn.execute(
        "create table if NOT exists conversations (
             id integer primary key,
             title text NOT NULL unique,
             participants NOT NULL
         )",
         []
    ) 
    {
        Ok(..) => println!("Successfully initialized conversations table"),
        Err(error) => println!("{}", error),
    }
    match conn.execute(
        "CREATE TABLE if NOT EXISTS messages (
             message_id INT primary key,
             sender text NOT NULL,
             date_time text NOT NULL,
             content_type text,
             content text NOT NULL,
             timestamp BIGINT NOT NULL
         )",
         []
    ) 
    {
        Ok(..) => println!("Successfully initialized messages table"),
        Err(error) => println!("{}", error),
    }
    match conn.execute(
        "CREATE TABLE if NOT EXISTS reactions (
            reactor text NOT NULL,
            reaction text NOT NULL,
            message_id BIGINT NOT NULL,
            message_owner text NOT NULL,
            FOREIGN KEY(message_id) REFERENCES messages(message_id) 
         )",
         []
    )
    //FOREIGN KEY(message_id) REFERENCES messages(id) 
    {
        Ok(..) => println!("Successfully initialized reactions table"),
        Err(error) => println!("{}", error),
    }
}

