use rusqlite::Connection;
use std::io::{self, Write};
use crate::database_utility::table_initialization::table_initialization;

pub fn create_database_if_none_exists() {
    let db_name = get_db_name();
    new_database(db_name.clone());
    table_initialization(db_name.clone());

}

fn new_database(db_name: String){
    match Connection::open(&db_name){
        Ok(..) => println!("Successfully created {}", db_name),
        Err(error_name) => {println!("{}", error_name)},
    }
}

fn get_db_name() -> String{
    print!("Enter a name for your database (do not include .db or any special characters): "); 
    io::stdout().flush().expect("Not sure why this wouldnt work"); // Flushes the buffer
    let db_name = get_user_input();
    print!("You entered:  {}  as your database name, is this correct? (y/n)", db_name);
    io::stdout().flush().expect("Not sure why this wouldnt work"); // Flushes the buffer

    match get_user_input().as_str() {
        "y" | "Y" => return [db_name.to_string(), ".db".to_string()].join(""),
        _ => {
            println!("Could not confirm. Please try again.");
             return get_db_name();
        },
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to get input");
    input.trim().to_string()
}