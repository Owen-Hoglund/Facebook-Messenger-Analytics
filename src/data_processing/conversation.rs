use serde::{Deserialize, Serialize, de::Deserializer, de::IgnoredAny};  // for deserializing JSON
use encoding::{Encoding, EncoderTrap, DecoderTrap};                     // for fixing the mojibake in fb JSON
use encoding::all::{ISO_8859_1, UTF_8};

#[derive(Deserialize)]
pub struct Conversation {
    participants: Vec<Participant>, 
    pub messages: Vec<Message>, 
    #[serde(deserialize_with = "fix_string_encoding")]  // deserializes strings with a mojibake fix
    title: String, // Name of groupchat, if it exists
    #[serde(deserialize_with = "fix_string_encoding")]
    thread_path: String, // Path to conversation, probably useless
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
    pub sender_name: String,
    pub timestamp_ms: u64,
    pub message_type: Option<String>,
    pub text: Option<String>,
    pub reactions: Option<Vec<Reaction>>
} impl From<RawMessage> for Message {
    fn from(m:RawMessage) -> Self{
        Self { 
            sender_name: m.sender_name,
            timestamp_ms: m.timestamp_ms,
            message_type: {
                m.message_type.as_ref().map(|message| message.message_type_matching())
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
    fn merge_messages(messages_one: Vec<Message>, messages_two: Vec<Message>) -> Vec<Message>{
        messages_one.into_iter().chain(
            messages_two.into_iter()
        ).collect::<Vec<Message>>()
    }
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Reaction {
    #[serde(deserialize_with = "fix_string_encoding")]
    pub reaction: String,
    #[serde(deserialize_with = "fix_string_encoding")]
    pub actor: String,
}

#[derive(Deserialize, Clone, PartialEq, Eq)]
pub struct Participant {
    #[serde(deserialize_with = "fix_string_encoding")]
    pub name: String,
}

// Facebook download data is incorrectly encoded. This fixes the mojibake.
// Some emojis are not supported by all platforms, so some of the emojis in the 
// text may render as two emojis with a zero width joiner separating them
fn fix_string_encoding<'de, D: Deserializer<'de>>(de: D) -> Result<String, D::Error> {
    let original = String::deserialize(de)?;
    Ok(demojibake(&original))
}

fn demojibake(raw_string: &str) -> String {
    let encoded_string = ISO_8859_1.encode(raw_string, EncoderTrap::Strict).unwrap();
    UTF_8.decode(&encoded_string, DecoderTrap::Strict).expect("Didnt properly decode")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize, de::Deserializer, de::IgnoredAny};  // for deserializing JSON

    const EXAMPLE_DATA: &str = include_str!(r"../test_files/json_example.json");

    #[test]
    fn fix_string_encoding_test() {
        #[derive(Serialize, Deserialize)]
        struct EncodeTest {
            proper_encoding: String,
            improper_encoding: String,
            fixed_encoding: String,
        }
        
        let test_data: EncodeTest = serde_json::from_str(EXAMPLE_DATA).expect("Failed to read from inline JSON");
        let good_string = test_data.proper_encoding;
        let bad_string = test_data.improper_encoding;
        let fixed_string = test_data.fixed_encoding;

        assert_eq!(demojibake(&good_string), good_string);
        assert_eq!(demojibake(&bad_string), fixed_string);
    }
}
