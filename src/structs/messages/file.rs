use crate::enums::MessageFromType;
use crate::utils::gen_message_id;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileMessagePackage {
    message_id: u32,
    file_data: Vec<u8>,
    from: MessageFromType,
    timestamp: DateTime<Utc>,
}

impl FileMessagePackage {
    pub fn new(from: MessageFromType, file_data: Vec<u8>) -> Result<Self, String> {
        Ok(Self {
            message_id: gen_message_id(),
            from,
            file_data,
            timestamp: Utc::now(),
        })
    }

    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    pub fn from(&self) -> MessageFromType {
        self.from.clone()
    }

    pub fn file_data(&self) -> Vec<u8> {
        self.file_data.clone()
    }

    pub fn timestamp(&self) -> String {
        self.timestamp.to_string()
    }

    pub fn pretty_timestamp(&self) -> String {
        self.timestamp
            .format("%A, %B, %e, %Y at %I:%M%p")
            .to_string()
    }

    // fn as_any(&self) -> &dyn Any {
    //   self
    // }
}
