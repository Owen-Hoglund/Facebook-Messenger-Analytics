use chrono::{NaiveDateTime, DateTime, Utc};
use rusqlite::Connection;
use crate::data_processing::conversation::*;

struct  DatabaseMessage {
    message_id: String,
    sender: String,
    date_time: String,
    content_type: String,
    content: String,
    timestamp: String
}
impl DatabaseMessage {
    fn new(message: &Message, id_num: usize) -> Self {
        Self {
            message_id: id_num.to_string(),
            sender: message.sender_name.clone(), 
            date_time: epoch_date_time(message.timestamp_ms), 
            content_type: message.message_type.clone().unwrap_or_else(|| "".to_string()), 
            content: message.text.clone().unwrap_or_else(|| "".to_string()), 
            timestamp: message.timestamp_ms.to_string()
        }
    }
}

struct DatabaseReaction {
    reactor:String,
    reaction: String,
    message_id: String,
    message_owner: String,
}
impl DatabaseReaction {
    fn new(reaction: &Reaction, id: usize, owner: String) -> Self {
        Self { 
            reactor: reaction.actor.clone(), 
            reaction: reaction.reaction.clone(), 
            message_id: id.to_string(),
            message_owner: owner
        }
    }
}

fn message_batch_preparation(messages: &[Message]) -> Vec<DatabaseMessage>{
    messages.iter().enumerate().map(|x|{
        DatabaseMessage::new(x.1, x.0)
    }).collect::<Vec<DatabaseMessage>>()
}


fn reaction_batch_preparation(messages: &[Message]) -> Vec<DatabaseReaction> {
   messages.iter()
    .enumerate()
    .filter(|x| x.1.reactions.is_some())
    .flat_map(|x|{
        x.1.reactions.as_ref().into_iter().flatten().map(move |y| {
            DatabaseReaction::new(y, x.0 + 1, x.1.sender_name.clone())
        })
    })
    .collect::<Vec<DatabaseReaction>>()
}


pub fn insertion_batching(chat_log: Conversation, database: &str) -> Result<(), rusqlite::Error>{
    let mut conn = Connection::open(database).expect("Failed to open database");

    let messages = message_batch_preparation(&chat_log.messages);
    let reactions = reaction_batch_preparation(&chat_log.messages);

    let transaction = conn.transaction()?;
    for message in messages {
        transaction.execute("
        INSERT INTO messages (
            message_id, sender, date_time, content_type, content, timestamp
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6)
     ", [message.message_id, message.sender, message.date_time, message.content_type, message.content, message.timestamp])?;
    }
    transaction.commit()?;

    let transaction = conn.transaction()?;
    for reaction in reactions {
        transaction.execute(
        "INSERT INTO reactions (reactor, reaction, message_id, message_owner)
            VALUES (?1, ?2, ?3, ?4)",
            [reaction.reactor, reaction.reaction, reaction.message_id, reaction.message_owner])?;
    }
    transaction.commit()?;

    // match conn.execute(
    //     "DELETE FROM messages WHERE content LIKE '%to your message'", []){
    //     Ok(..) => todo!(),
    //     Err(error) => println!("{}", error),
    // };

    Ok(())
}

fn epoch_date_time(epoch: u64) -> String {
    let epoch = epoch / 1000;
    let naive = NaiveDateTime::from_timestamp_opt(epoch as i64, 0);
    let date_time:DateTime<Utc> = DateTime::from_utc(naive.unwrap(), Utc);
    date_time.format("%Y-%m-%d %I:%M %p").to_string()
}