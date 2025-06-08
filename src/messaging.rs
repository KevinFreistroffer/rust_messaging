use crate::db::get_collection;
use crate::enums::Message;
use crate::enums::Message::{File, Image};
use crate::enums::MessageFromType;
use crate::enums::MessageType;
use crate::structs::messages::text::TextMessagePackage;
use futures::stream::{StreamExt, TryStreamExt};
use mongodb::bson::{doc, to_document, Bson, DateTime, Document};
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

pub async fn insert_text_message<'a>(
    message: TextMessagePackage<'a>,
) -> Result<Option<Document>, mongodb::error::Error> {
    println!("db::saveMessage(): {:?}", message);
    let collection = get_collection("message_history").await?;
    let insert_one_result = collection
        .insert_one(to_document(&message).unwrap())
        .await?;
    // println!("db::saveMessage result: {:?}", result);
    let document = collection
        .find_one(doc! { "_id": insert_one_result.inserted_id })
        .await?;

    Ok(document)
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

// pub fn find_messages_from_sender<'a>(from: MessageFromType) -> Option<Vec<Message<'a>>> {
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

pub fn debug_saved_message(message_package: Document, pretty_print:bool) {
    if let Some(id) = message_package.get("_id") {
        println!("Document ID: {:?}", id);
    }
    if let Some(message_id) = message_package.get("message_id") {
        println!("Message ID: {:?}", message_id);
    }
    if let Some(msg_type) = message_package.get("type") {
        println!("Message Type: {:?}", msg_type);
    }
    if let Some(message) = message_package.get("message") {
        println!("Message Content: {:?}", message);
    }
    if let Some(sender) = message_package.get("sender") {
        if let Some(sender_doc) = sender.as_document() {
            if let Some(sender_id) = sender_doc.get("sender_id") {
                println!("Sender ID: {:?}", sender_id);
            }
            if let Some(sender_type) = sender_doc.get("sender_type") {
                println!("Sender Type: {:?}", sender_type);
            }
        }
    }
    if let Some(timestamp) = message_package.get("timestamp") {
        if pretty_print {
            // IMPLEMENT THIS
            println!("Timestamp: {:?}", timestamp);
        }
    }
    // match message_package {
    // Text(message) => {
    //     println!("Message ID: {:?}", message.message_id());
    //     println!("From Sender ID: {:?}", message.sender().sender_id());
    //     println!("From Sender Type: {:?}", message.sender().sender_type());
    //     println!("Timestamp: {:?}", message.pretty_timestamp());
    //     println!("Message: {:?}", message.message());
    // }
    // Image(message) => {
    //     println!(
    //         "Message ({:?}) from {:?} on {:?}",
    //         message.message_id(),
    //         message.from(),
    //         message.pretty_timestamp()
    //     );
    //     println!(
    //         "Message image_data length: {:?}",
    //         message.image_data().len()
    //     );
    // }
    // File(message) => {
    //     println!(
    //         "Message ({:?}) from {:?} on {:?}",
    //         message.message_id(),
    //         message.from(),
    //         message.pretty_timestamp()
    //     );
    //     println!("Message image_data length: {:?}", message.file_data().len());
    // }
    // CryptoTransfer(message) => {
    //     println!(
    //         "Crypto Transfer ({:?}) from {:?} on {:?}",
    //         message.message_id(),
    //         message.from(),
    //         message.pretty_timestamp()
    //     );
    //     println!(
    //         "Transfer: {} {} to {}",
    //         message.amount(),
    //         message.token_symbol(),
    //         message.recipient_address()
    //     );
    //     if let Some(signature) = message.transaction_signature() {
    //         println!("Transaction signature: {}", signature);
    //     }
    // }
    // _ => {

    // }
    // }
}
