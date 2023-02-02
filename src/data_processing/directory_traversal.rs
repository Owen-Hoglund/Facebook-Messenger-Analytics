use std::fs::{self, DirEntry};
use crate::Conversation;

pub fn load_conversations_from_directory() -> Vec<Conversation>{
    let dir:String = std::env::var("TEST_DIRECTORY").expect("No value for key 'TEST_DIRECTORY' found");
    let paths = fs::read_dir(dir).unwrap();
    paths.into_iter().flatten().filter(|x| {
        match x.path().extension(){
            Some(t) => t.to_str().unwrap() == "json",
            None => false,
        }
    }).map(conversation_from_path).collect::<Vec<_>>()
}

fn conversation_from_path(dir: DirEntry) -> Conversation{
    serde_json::from_str(&fs::read_to_string(dir.path()).expect("Couldnt open file")).unwrap()
}