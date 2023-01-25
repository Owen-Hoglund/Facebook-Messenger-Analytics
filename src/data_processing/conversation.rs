use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RawConversation {
    participants: Vec<Participant>,
    messages: Vec<RawMessage>,
    title: String,
    is_still_participant: bool,
    pub thread_path: String,
    magic_words: Vec<MagicWord>,
    image: Image,
    joinable_mode: JoinableMode,
}
impl RawConversation {
    pub fn get_title(&self) -> String{
        self.title.clone()
    }
    pub fn get_participants(&self) -> Vec<String>{
        self.participants.iter().map(|x| x.name.clone()).collect::<Vec<String>>()
    }
    pub fn get_messages(&self) -> Vec<RawMessage> {
        self.messages.clone()
    }
}
pub struct Conversation {
    pub title: String,
    pub participants: Vec<String>,
    pub messages: Vec<Message>,
}


pub struct Message {
    pub sender: String,
    pub timestamp: u64,
    pub content_type: ContentType,
    pub content: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
}

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

#[derive(Serialize, Deserialize)]
#[derive(Debug, Clone)]
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