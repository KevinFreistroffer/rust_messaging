use crate::structs::messages::{image::ImageMessagePackage, text::MessagePackage};

#[derive(Debug, Clone)]
pub enum Message {
    Image(ImageMessagePackage),
    Text(MessagePackage)
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageFrom {
  Agent,
  User
}

impl MessageFrom {
  pub fn is_agent(&self) -> bool {
      matches!(self, MessageFrom::Agent)
  }

  pub fn to_string(&self) -> String {
      match self {
          MessageFrom::Agent => "Agent".to_string(),
          MessageFrom::User => "User".to_string()
      }
  }
}