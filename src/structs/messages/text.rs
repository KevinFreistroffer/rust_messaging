use crate::enums::MessageFrom;
use crate::utils::{gen_id};
use chrono::prelude::*;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct MessagePackage {
  id: u32,
  message: String,
  from: MessageFrom,
  timestamp: DateTime<Utc>
}

impl MessagePackage {
  pub fn new(from: MessageFrom, message: String) -> Result<Self, String> {
      if message.len() <= 512 {
        Ok(Self {
          id: gen_id(),
          from,
          message,
          timestamp: Utc::now()
        })
      } else {
        Err("Message is too long. Max characters is 512.".into())
      }
  }

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn from(&self) -> MessageFrom {
    self.from.clone()
  }

  pub fn message(&self) -> String {
    self.message.clone()
  }

  pub fn timestamp(&self) -> String {
    self.timestamp.to_string()
  }

  pub fn pretty_timestamp(&self) -> String {
    self.timestamp.format("%A, %B, %e, %Y at %I:%M%p").to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

