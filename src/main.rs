extern crate encoding;
use data_processing::{conversation::Conversation, directory_traversal};
mod data_processing;
mod database_utility;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let chat_logs_from_directory = directory_traversal::load_conversations_from_directory();
    let chat_log = chat_logs_from_directory.into_iter().reduce(Conversation::merge_conversations).expect("failed to reduce");
    let database = database_utility::database_creation::create_database_return_name();
    match database_utility::database_population::insertion_batching(chat_log, &database){
        Ok(..) => println!("Sucessfully created database"),
        Err(error) => println!("{}", error),
    };
}