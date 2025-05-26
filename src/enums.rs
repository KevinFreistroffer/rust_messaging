use crate::structs::messages::{image::ImageMessagePackage, text::MessagePackage, file::FileMessagePackage};

#[derive(Debug, Clone)]
pub enum Message {
    Image(ImageMessagePackage),
    Text(MessagePackage),
    File(FileMessagePackage)
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