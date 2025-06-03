use crate::db::get_collection;
use crate::enums::{Message};
use crate::enums::Message::{File, Image};
use crate::enums::MessageFrom;
use crate::structs::messages::text::TextMessagePackage;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::bson::{Document, doc};
use mongodb::bson::to_document;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub async fn get_messages<'a>()
-> Result<Vec<Result<Document, mongodb::error::Error>>, mongodb::error::Error> {
    let collection = get_collection("message_history").await?;
    let cursor = collection.find(doc! { "type": "text" }).await?;
    let v: Vec<Result<Document, mongodb::error::Error>> = cursor.collect().await;

    Ok(v)
}

// pub async fn insert_message<'a>(message: Message<'a>) {
//     println!("db::saveMessage(): {:?}", message);
//     let collection = get_collection("message_history").await?;
//     let cursor = collection.insert_one(message).await?;
//     let v: Vec<Result<Document, mongodb::error::Error>> = cursor.collect().await;

//     Ok(())
// }

pub async fn insert_text_message<'a>(message: TextMessagePackage<'a>) -> Result<(), mongodb::error::Error> {
    println!("db::saveMessage(): {:?}", message);
    let collection = get_collection("message_history").await?;
    let result = collection.insert_one(to_document(&message).unwrap()).await?;
    let id = result.inserted_id;
    println!("db::saveMessage result: {:?}", id);

    Ok(())
}

// pub fn find_message_by_id<'a>(message_id: u32) -> Option<Message<'a>> {
//     let messages = get_messages();

//     messages
//         .iter()
//         .find(|message| match message {
//             Message::Text(message) => message.message_id() == message_id,
//             Message::Image(message) => message.message_id() == message_id,
//             Message::File(message) => message.message_id() == message_id,
//         })
//         .cloned()
// }

// pub fn find_messages_by_text<'a>(text: String) -> Option<Vec<Message<'a>>> {
//     let messages: Vec<Message> = get_messages();

//     let results: Vec<Message> = messages
//         .iter()
//         .filter(|message| {
//             if let Message::Text(msg_package) = message {
//                 msg_package.message() == text
//             } else {
//                 false
//             }
//         })
//         .cloned()
//         .collect();

//     if !results.is_empty() {
//         Some(results)
//     } else {
//         None
//     }
// }

// pub fn find_messages_from_sender<'a>(from: MessageFrom) -> Option<Vec<Message<'a>>> {
//     let messages: Vec<Message> = get_messages();

//     let results: Vec<Message> = messages
//         .iter()
//         .filter(|message| match message {
//             Message::Text(msg) => msg.from() == from,
//             Image(msg) => msg.from() == from,
//             File(msg) => msg.from() == from,
//         })
//         .cloned()
//         .collect();

//     if !results.is_empty() {
//         Some(results)
//     } else {
//         None
//     }
// }
