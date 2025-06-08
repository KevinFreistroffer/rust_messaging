use crate::structs::messages::{
    crypto::CryptoTransferMessagePackage, file::FileMessagePackage, image::ImageMessagePackage,
    text::TextMessagePackage,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    File,
    CryptoTransfer,
}

#[derive(Debug, Clone)]
pub enum Message<'a> {
    Text(TextMessagePackage<'a>),
    Image(ImageMessagePackage),
    File(FileMessagePackage),
    CryptoTransfer(CryptoTransferMessagePackage<'a>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageFromType {
    Agent,
    User,
}

impl MessageFromType {
    pub fn is_agent(&self) -> bool {
        matches!(self, MessageFromType::Agent)
    }

    pub fn to_string(&self) -> String {
        match self {
            MessageFromType::Agent => "Agent".to_string(),
            MessageFromType::User => "User".to_string(),
        }
    }
}
