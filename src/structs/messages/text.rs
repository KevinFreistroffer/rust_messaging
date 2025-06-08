use crate::enums::{MessageFromType, MessageType};
use crate::structs::user::User;
use crate::utils::gen_message_id;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TextMessagePackage<'a> {
    message_id: u32,
    r#type: MessageType,
    message: &'a str,
    sender: User,
    timestamp: DateTime<Utc>,
}

impl<'a> TextMessagePackage<'a> {
    pub fn new(sender: User, message: &'a str) -> Result<Self, String> {
        if message.len() <= 512 {
            Ok(Self {
                message_id: gen_message_id(),
                r#type: MessageType::Text,
                sender,
                message,
                timestamp: Utc::now(),
            })
        } else {
            Err("Message is too long. Max characters is 512.".into())
        }
    }

    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    pub fn r#type(&self) -> MessageType {
        self.r#type.clone()
    }

    pub fn sender(&self) -> &User {
        &self.sender
    }

    pub fn message(&self) -> &str {
        self.message
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
