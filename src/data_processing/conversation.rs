use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize, de::Deserializer};
use serde;
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::ISO_8859_1;
use encoding::all::UTF_8;

#[derive(Deserialize)]
pub struct Conversation {
    participants: Vec<Participant>, 
    messages: Vec<Message>, 
    #[serde(deserialize_with = "fix_string_encoding")]
    title: String,
    thread_path: String,
}
impl Conversation {
    pub fn get_title(&self) -> String{
        self.title.clone()
    }
    pub fn get_participants_debug(&self) -> Vec<String>{
        self.participants.clone().into_iter().map(|x| x.name).collect::<Vec<String>>()
    }
    pub fn get_participants(&self) -> Vec<Participant>{
        self.participants.clone()
    }
    pub fn get_messages_debug(&self) -> Vec<String> {
        self.messages.clone().into_iter().map(get_message_contents).collect::<Vec<String>>()
    }
}

impl Conversation {
    pub fn merge_conversations(conversation_one: Conversation, conversation_two: Conversation) -> Self {
        Self {
            participants: conversation_one.participants,
            messages: Message::merge_messages(conversation_one.messages, conversation_two.messages),
            title: conversation_one.title,
            thread_path: conversation_one.thread_path,
        }
    }
}


#[derive(Deserialize)]
pub struct RawMessage {
    // These fields will be in every message
    sender_name: String,
    timestamp_ms: u64,

    #[serde(flatten)]
    message_type: Option<MessageType>,
    reactions: Option<Vec<Reaction>>
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
enum MessageType {
    #[serde(deserialize_with = "fix_string_encoding")]
    Content(String),
    Photos(IgnoredAny),
    Videos(IgnoredAny),
    AudioFiles(IgnoredAny),
    Gifs(IgnoredAny),
    Sticker(IgnoredAny),
    IsUnsent(IgnoredAny),
    BumpedMessageMetadata(IgnoredAny),
}
impl MessageType {
    pub fn message_type_matching(&self) -> String {
        match self {
            MessageType::Photos(..) => "photo".to_string(),
            MessageType::Videos(..) => "video".to_string(),
            MessageType::AudioFiles(..) => "audio".to_string(),
            MessageType::Gifs(..) => "gif".to_string(),
            MessageType::Sticker(..) => "sticker".to_string(),
            MessageType::IsUnsent(..) => "unsent".to_string(),
            MessageType::BumpedMessageMetadata(..) => "bumped".to_string(),
            MessageType::Content(..) => "text".to_string(),
        }
    }
    pub fn get_text_if_exists(&self) -> Option<String> {
        match self {
            MessageType::Content(text) => Some(text.to_string()),
            _ => None,
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
#[serde(from = "RawMessage")]
pub struct Message {
    sender_name: String,
    timestamp_ms: u64,
    message_type: Option<String>,
    text: Option<String>,
    reactions: Option<Vec<Reaction>>
} impl From<RawMessage> for Message {
    fn from(m:RawMessage) -> Self{
        Self { 
            sender_name: m.sender_name,
            timestamp_ms: m.timestamp_ms,
            message_type: {
                match &m.message_type {
                    Some(message) => Some(message.message_type_matching()),
                    None => None,
                }
            },
            text: {
                match &m.message_type {
                    Some(message) => message.get_text_if_exists(),
                    None => None,
                }
            }, 
            reactions: m.reactions 
        }
    }
} 
impl Message {
    // chains the two vectors of messages together. 
    // IMPORTANT - THIS DOES NOT GUARANTEE TEMPORAL CONSISTENCY!!!
    // Sorting by timestamp is CRUCIAL --- MUST BE IMPLEMENTED LATER
    fn merge_messages(messages_one: Vec<Message>, messages_two: Vec<Message>) -> Vec<Message>{
        messages_one.into_iter().chain(
            messages_two.into_iter()
        ).collect::<Vec<Message>>()
    }
}

fn get_message_contents(message: Message) -> String{
    match message.text{
        Some(message_text) => message_text,
        None => {
            match message.message_type{
                Some(message_type) => message_type,
                None => "Empty Message".to_string(),
            }
        },
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Reaction {
    #[serde(deserialize_with = "fix_string_encoding")]
    reaction: String,
    #[serde(deserialize_with = "fix_string_encoding")]
    actor: String,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Participant {
    #[serde(deserialize_with = "fix_string_encoding")]
    name: String,
}

// Facebook download data is incorrectly encoded. This fixes most but not all of the errors. 
// Largely I believe the failure cases are due to facebook having its own in house emojis
//
fn fix_string_encoding<'de, D: Deserializer<'de>>(de: D) -> Result<String, D::Error> {
    let original = String::deserialize(de)?;
    let encoded_string = ISO_8859_1.encode(&original, EncoderTrap::Strict).unwrap();
    Ok(UTF_8.decode(&encoded_string, DecoderTrap::Strict).expect("Didnt properly decode"))
}