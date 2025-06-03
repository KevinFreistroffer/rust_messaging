use crate::enums::MessageFrom;
use crate::utils::gen_id;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageMessagePackage {
    message_id: u32,
    image_data: Vec<u8>,
    from: MessageFrom,
    timestamp: DateTime<Utc>,
}

impl ImageMessagePackage {
    pub fn new(from: MessageFrom, image_data: Vec<u8>) -> Result<Self, String> {
        Ok(Self {
            message_id: gen_id(),
            from,
            image_data: image_data,
            timestamp: Utc::now(),
        })
    }

    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    pub fn from(&self) -> MessageFrom {
        self.from.clone()
    }

    pub fn image_data(&self) -> Vec<u8> {
        self.image_data.clone()
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
