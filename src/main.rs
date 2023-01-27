extern crate encoding;
use data_processing::conversation::Conversation;
use dotenv::dotenv;
use std::fs;
mod data_processing;
mod database_utility;


fn main() {
    // Load file paths from .env (you must enter these paths yourself!)
    dotenv().ok();
    // Begins the testing process
    test_program();
}

fn test_program(){
    // Load file paths from environment
    let test_one: String = std::env::var("TEST_PATH_ONE").expect("No value for key 'test_path' found");
    let test_two: String = std::env::var("TEST_PATH_TWO").expect("No value for key 'test_path' found");

    // Serializes the data 
    let test_one_data = load_test_data(test_one);
    println!("First Conversation Size: {:?}", test_one_data.get_messages_debug().len());
    let test_two_data = load_test_data(test_two);
    println!("Second Conversation Size: {:?}", test_two_data.get_messages_debug().len());

    // Currently this checks if the chat logs refer to the same conversation. This will move
    let result = Conversation::merge_conversations(test_one_data, test_two_data);
    println!("{}", result.get_title());
    // println!("{:?}", result.get_participants_debug());
    println!("Summed Conversation Size:{:?}", result.get_messages_debug().len());
}

// Explicit typing is essential for telling serde what we are loading the JSON into 
fn load_test_data(test_path: String) -> data_processing::conversation::Conversation{
    // Read the file to a &str for serde to deserialize
    let data = fs::read_to_string(test_path).expect("Couldnt open file");
    // Deserialization
    serde_json::from_str(&data).unwrap()
}