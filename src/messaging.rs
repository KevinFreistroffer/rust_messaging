use crate::db::get_collection;
use crate::enums::{Message, MessageType, MessageFrom};
use crate::structs::messages::{
    text::TextMessagePackage, 
    image::ImageMessagePackage, 
    file::FileMessagePackage
};
use chrono::{DateTime, Utc};
use mongodb::bson::{Document, doc, to_document, from_document};
use mongodb::error::Result as MongoResult;
use futures::StreamExt;

// Generic function to save any message type to MongoDB
pub async fn save_message(collection_name: &str, message: Document) -> MongoResult<()> {
    let collection = get_collection(collection_name).await?;
    collection.insert_one(message, None).await?;
    Ok(())
}

// Save text message
pub async fn save_text_message(message: TextMessagePackage<'_>) -> MongoResult<()> {
    let doc = to_document(&message)?;
    save_message("messages", doc).await
}

// Save image message
pub async fn save_image_message(message: ImageMessagePackage) -> MongoResult<()> {
    let doc = to_document(&message)?;
    save_message("messages", doc).await
}

// Save file message
pub async fn save_file_message(message: FileMessagePackage) -> MongoResult<()> {
    let doc = to_document(&message)?;
    save_message("messages", doc).await
}

// Retrieve all messages
pub async fn get_all_messages() -> MongoResult<Vec<Document>> {
    let collection = get_collection("messages").await?;
    let mut cursor = collection.find(doc! {}, None).await?;
    let mut messages = Vec::new();
    while let Some(doc) = cursor.next().await {
        messages.push(doc?);
    }
    Ok(messages)
}

// Retrieve messages by type
pub async fn get_messages_by_type(message_type: MessageType) -> MongoResult<Vec<Document>> {
    let collection = get_collection("messages").await?;
    let type_str = match message_type {
        MessageType::Text => "Text",
        MessageType::Image => "Image",
        MessageType::File => "File",
    };
    let filter = doc! { "type": type_str };
    let mut cursor = collection.find(filter, None).await?;
    let mut messages = Vec::new();
    while let Some(doc) = cursor.next().await {
        messages.push(doc?);
    }
    Ok(messages)
}

// Retrieve messages in date range
pub async fn get_messages_by_date_range(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
) -> MongoResult<Vec<Document>> {
    let collection = get_collection("messages").await?;
    let filter = doc! {
        "timestamp": {
            "$gte": start_date.to_rfc3339(),
            "$lte": end_date.to_rfc3339()
        }
    };
    let mut cursor = collection.find(filter, None).await?;
    let mut messages = Vec::new();
    while let Some(doc) = cursor.next().await {
        messages.push(doc?);
    }
    Ok(messages)
}

// Retrieve messages by user type (Agent/User)
pub async fn get_messages_by_user_type(user_type: MessageFrom) -> MongoResult<Vec<Document>> {
    let collection = get_collection("messages").await?;
    let type_str = match user_type {
        MessageFrom::Agent => "Agent",
        MessageFrom::User => "User",
    };
    let filter = doc! { "from": type_str };
    let mut cursor = collection.find(filter, None).await?;
    let mut messages = Vec::new();
    while let Some(doc) = cursor.next().await {
        messages.push(doc?);
    }
    Ok(messages)
}

// Retrieve messages by sender ID
pub async fn get_messages_by_sender_id(sender_id: u32) -> MongoResult<Vec<Document>> {
    let collection = get_collection("messages").await?;
    let filter = doc! { "sender_id": sender_id as i32 };
    let mut cursor = collection.find(filter, None).await?;
    let mut messages = Vec::new();
    while let Some(doc) = cursor.next().await {
        messages.push(doc?);
    }
    Ok(messages)
}

// Helper function to deserialize documents back to message types
pub fn deserialize_message(doc: &Document) -> Result<Message, Box<dyn std::error::Error>> {
    let message_type = doc.get_str("type")?;
    
    match message_type {
        "Text" => {
            let text_msg: TextMessagePackage = from_document(doc.clone())?;
            Ok(Message::Text(text_msg))
        },
        "Image" => {
            let img_msg: ImageMessagePackage = from_document(doc.clone())?;
            Ok(Message::Image(img_msg))
        },
        "File" => {
            let file_msg: FileMessagePackage = from_document(doc.clone())?;
            Ok(Message::File(file_msg))
        },
        _ => Err("Unknown message type".into()),
    }
}
