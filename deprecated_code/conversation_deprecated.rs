use serde::{Deserialize, Serialize};

use super::re_de_encoding::fix_encoding;


// The raw unedited struct that we deserialize our JSON data from 
// This is a placeholder and should never be used again after cleaning to Conversation type
#[derive(Serialize, Deserialize)]
pub struct RawConversation {
    participants: Vec<Participant>,
    messages: Vec<RawMessage>,
    title: String,
    is_still_participant: bool,
    thread_path: String,
    magic_words: Vec<MagicWord>,
    image: Image,
    joinable_mode: JoinableMode,
}

impl RawConversation {
    // Returns a vector of Strings representing the people in the conversation
    pub fn get_title(&self) -> String{
        // Fixes the mojibake encoding error - more detail at fix_encodings implementation
        fix_encoding(self.title.clone())
    }
    // Returns a vector of Strings representing the people in the conversation
    pub fn get_participants(&self) -> Vec<String>{
        self.participants.iter().map(|x| {
            // Maps each participant: String to its fixed encoding version
            fix_encoding(x.name.clone())
        }).collect::<Vec<String>>()
    }
    // This just sends a complete vector of raw messages back - Encoding fix is invoked done in conversation_cleaner
    pub fn get_messages(&self) -> Vec<RawMessage> {
        self.messages.clone()
    }
}

// The clean format we hold our data in. I believe this will only be used for loading our SQL database
pub struct Conversation {
    pub title: String,
    pub participants: Vec<String>,
    pub messages: Vec<Message>,
}

// Getter functions
impl Conversation {
    pub fn get_title(&self) -> String{
        self.title.clone()
    }
    pub fn get_messages_as_message_type(&self) -> Vec<Message>{
        self.messages.clone()
    }
    pub fn get_participants(&self) -> Vec<String>{
        self.participants.clone()
    }
}

// clean message contains fields for all possible variants of 
#[derive(Clone)]
pub struct Message {
    pub sender: String,
    pub timestamp: u64,
    pub content_type: ContentType,
    pub content: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
}

// Message content variant types
#[derive(Clone)]
pub enum ContentType {
    Text,
    Photo,
    Video,
    Audio,
    Gif,
    Sticker,
    Bumped,
    UnsentMessage,
    EmptyMessage,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RawMessage {
    PhotoWithReactions {
        sender_name: String,
        timestamp_ms: u64,
        photos: Vec<Photo>,
        reactions: Vec<Reaction>
    },
    PhotoWithoutReactions {
        sender_name: String,
        timestamp_ms: u64,
        photos: Vec<Photo>,
    },
    VideoWithReactions {
        sender_name: String,
        timestamp_ms: u64,
        videos: Vec<Video>,
        reactions: Vec<Reaction>
    },
    VideoWithoutReactions {
        sender_name: String,
        timestamp_ms: u64,
        videos: Vec<Video>,
    },
    AudioWithReactions {
        sender_name: String,
        timestamp_ms: u64,
        audio_files: Vec<Audio>,
        reactions: Vec<Reaction>
    },
    AudioWithoutReactions {
        sender_name: String,
        timestamp_ms: u64,
        audio_files: Vec<Audio>,
    },
    GifWithReactions {
        sender_name: String,
        timestamp_ms: u64,
        gifs: Vec<Gif>,
        reactions: Vec<Reaction>
    },
    GifWithoutReactions {
        sender_name: String,
        timestamp_ms: u64,
        gifs: Vec<Gif>,
    },
    StickerWithReactions {
        sender_name: String,
        timestamp_ms: u64,
        sticker: Sticker,
        reactions: Vec<Reaction>
    },
    StickerWithoutReactions {
        sender_name: String,
        timestamp_ms: u64,
        sticker: Sticker,
    },
    TextWithReactions {
        sender_name: String,
        timestamp_ms: u64,
        content: String,
        reactions: Vec<Reaction>
    },
    TextWithoutReactions {
        sender_name: String,
        timestamp_ms: u64,
        content: String,
    },
    UnsentMessage {
        sender_name: String,
        timestamp_ms: u64,
        is_unsent: bool,
    },
    BumpedMessageWithReactions {
        sender_name: String,
        timestamp_ms: u64,
        reactions: Vec<Reaction>,
        bumped_message_metadata: BumpedMessageMetaData,
    },
    BumpedMessageWithoutReactions {
        sender_name: String,
        timestamp_ms: u64,
        bumped_message_metadata: BumpedMessageMetaData,
    },
    EmptyMessage {
        sender_name: String,
        timestamp_ms: u64,
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Reaction {
    reaction: String,
    actor: String,
}
impl Reaction {
    pub fn new(reaction: String, actor: String) -> Reaction{
        Reaction {
            reaction: reaction,
            actor: actor,
        }
    }
    pub fn get_reaction(&self) -> String {
        fix_encoding(self.reaction.clone())
    }
    pub fn get_actor(&self) -> String {
        fix_encoding(self.actor.clone())
    }
}

#[derive(Serialize, Deserialize)]
pub struct MagicWord {
    magic_word: String,
    creation_timestamp_ms: u64,
    animation_emoji: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    uri: String,
    creation_timestamp: u64,
}

#[derive(Serialize, Deserialize)]
pub struct JoinableMode {
    mode: i32,
    link: String,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Photo {
    uri: String,
    creation_timestamp: u64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Video {
    uri: String,
    creation_timestamp: u64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Audio {
    uri: String,
    creation_timestamp: u64,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct Gif {
    uri: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sticker {
    uri: String,
}


#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
pub struct BumpedMessageMetaData {
    bumped_message: String,
    is_bumped: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Participant {
    name: String,
}