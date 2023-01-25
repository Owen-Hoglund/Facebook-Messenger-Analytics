use std::fs;
mod data_processing;

use crate::data_processing::conversation_cleaner;
fn main() {
    let test_conversation = test_data();
    let clean_test = conversation_cleaner::raw_conversation_to_cleaned_conversation(test_conversation);
    println!("{:?}", clean_test.participants);
    for m in clean_test.messages.iter().rev() {
        println!("{:?}", m.content);
    }

}

fn test_data() -> data_processing::conversation::RawConversation{
    let data = fs::read_to_string(
        r"C:\Users\owenh\OneDrive\Documents\Coding\Projects\fbm\facebook_chat_analytics\chat_logs\deutschbags\message_1.json"
    ).expect("Couldnt open file");
    serde_json::from_str(&data).unwrap()
}