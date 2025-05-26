use crate::enums::Message;
use crate::enums::Message::{Text, Image};
use crate::enums::{MessageFrom};
use once_cell::sync::Lazy;
use std::sync::Mutex;

static HISTORY: Lazy<Mutex<Vec<Message>>> = Lazy::new(||  {
  Mutex::new(vec![])
});

pub fn get_history() -> Vec<Message> {
    HISTORY.lock().unwrap().clone()
}

pub fn add_to_history(message_package: Message) {
  HISTORY.lock().unwrap().push(message_package);
}

pub fn find_message_by_id(id: u32) -> Option<Message> {
  let history = HISTORY.lock().unwrap();
  
  history.iter().find(|message| {
    match message {
      Message::Text(message) => message.id() == id,
      Message::Image(message) => message.id() == id
    }
  }).cloned()
}

pub fn find_messages_by_text(text: String) -> Option<Vec<Message>> {

  let history: Vec<Message> = get_history();
  
  let results: Vec<Message> = history
    .iter()
    .filter(|message| {
      if let Message::Text(msg_package) = message {
        msg_package.message() == text
      } else {
        false
      }
    })
    .cloned()
    .collect();

  if !results.is_empty() {
    Some(results)
  } else {
    None
  }
}

pub fn find_messages_from_sender(from: MessageFrom) -> Option<Vec<Message>> {

  let history: Vec<Message> = get_history();
  
  let results: Vec<Message> = history.iter().filter(|message| {
    match(message) {
      Text(msg) => {
        msg.from() == from
      },
      Image(msg) => {
        msg.from() == from
      }
    }
  }).cloned().collect();

  if !results.is_empty() {
    Some(results)
  } else {
    None
  }
}

