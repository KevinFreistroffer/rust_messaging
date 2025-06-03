use crate::structs::messages::{
    file::FileMessagePackage, image::ImageMessagePackage, text::TextMessagePackage,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    File,
}

#[derive(Debug, Clone)]
pub enum Message<'a> {
    Text(TextMessagePackage<'a>),
    Image(ImageMessagePackage),
    File(FileMessagePackage),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageFrom {
    Agent,
    User,
}

impl MessageFrom {
    pub fn is_agent(&self) -> bool {
        matches!(self, MessageFrom::Agent)
    }

    pub fn to_string(&self) -> String {
        match self {
            MessageFrom::Agent => "Agent".to_string(),
            MessageFrom::User => "User".to_string(),
        }
    }
}
