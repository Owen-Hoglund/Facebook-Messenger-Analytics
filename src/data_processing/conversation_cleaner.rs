use core::time;
use std::os::windows::raw;

use crate::data_processing::conversation::{Conversation, RawConversation, RawMessage, Message, ContentType, Reaction};
pub fn raw_conversation_to_cleaned_conversation(raw_conversation: RawConversation) -> Conversation {
    let clean = Conversation {
        title: raw_conversation.get_title(),
        participants: raw_conversation.get_participants(),
        messages: raw_messages_to_cleaned_messages(raw_conversation.get_messages()),
    };
    clean
}

fn raw_messages_to_cleaned_messages(raw_messages: Vec<RawMessage>) -> Vec<Message>{
    raw_messages.iter().map(|raw| message_cleaner(raw)).collect::<Vec<Message>>()
} 

fn message_cleaner(raw: &RawMessage) -> Message {
    match raw {
        RawMessage::TextWithReactions { sender_name, timestamp_ms, content, reactions, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Text,
            content: Some(content.to_string()),
            reactions: Some(reactions.to_vec()),
        },
        RawMessage::TextWithoutReactions {sender_name, timestamp_ms, content, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Text,
            content: Some(content.to_string()),
            reactions: None,
        },

        RawMessage::PhotoWithReactions {sender_name, timestamp_ms, reactions, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: Some(reactions.to_vec()),
        },
        RawMessage::PhotoWithoutReactions {sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: None,
        },

        RawMessage::VideoWithReactions {sender_name, timestamp_ms, reactions, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: Some(reactions.to_vec()),
        },
        RawMessage::VideoWithoutReactions {sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: None,
        },
        
        RawMessage::AudioWithReactions {sender_name, timestamp_ms,reactions, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: Some(reactions.to_vec()),
        },
        RawMessage::AudioWithoutReactions {sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: None,
        },
        
        RawMessage::GifWithReactions {sender_name, timestamp_ms, reactions,.. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: Some(reactions.to_vec()),
        },
        RawMessage::GifWithoutReactions {sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: None,
        },
        
        RawMessage::StickerWithReactions {sender_name, timestamp_ms, reactions, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: Some(reactions.to_vec()),
        },
        RawMessage::StickerWithoutReactions {sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: None,
        },
        
        RawMessage::BumpedMessageWithReactions {sender_name, timestamp_ms,reactions, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: Some(reactions.to_vec()),
        },
        RawMessage::BumpedMessageWithoutReactions {sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: None,
        },
        
        RawMessage::UnsentMessage {sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::Photo,
            content: None,
            reactions: None,
        },
        RawMessage::EmptyMessage{sender_name, timestamp_ms, .. } => Message {
            sender: sender_name.to_string(),
            timestamp: *timestamp_ms,
            content_type: ContentType::EmptyMessage,
            content: None,
            reactions: None,
        },
    }
}


// pub struct Message {
//     sender: String,
//     timestamp: u64,
//     content_type: ContentType,
//     content: Option<String>,
//     reactions: Option<Vec<Reaction>>,
// }
