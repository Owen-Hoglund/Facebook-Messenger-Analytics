extern crate encoding;
use std::fs;

use data_processing::{conversation::Conversation, directory_traversal};
mod data_processing;
mod database_utility;
use database_utility::database_api::command_line_control_loop;
use dotenv::dotenv;

fn main() {
    // create_new_db();
    command_line_control_loop();
}

fn create_new_db() {
    dotenv().ok();
    let chat_logs_from_directory = directory_traversal::load_conversations_from_directory();
    let chat_log = chat_logs_from_directory.into_iter().reduce(Conversation::merge_conversations).expect("failed to reduce");
    println!("{:?}", chat_log.participants);
    let database = database_utility::database_creation::create_database_return_name();
    match database_utility::database_population::insertion_batching(chat_log, &database){
        Ok(..) => println!("Sucessfully created database"),
        Err(error) => println!("{}", error),
    };
}

fn current_databases() -> Option<Vec<String>>{

    let entries = match fs::read_dir("") {
        Ok(directory) => directory.filter_map(|x| {
            match x {
                Ok(entry) => Some(entry),
                Err(err) => panic!("Error with particular DirEntry: {}", err),
            }
        }).filter(|x| x.path().extension().is_some())
        .filter(|x| x.path().extension().unwrap().to_str().unwrap() == "db")
        .map(|x| x.file_name().to_str().unwrap().to_owned()).collect::<Vec<String>>(),
        Err(err) => panic!("Error reading directory: {}", err),
    };

    Some(entries)
}