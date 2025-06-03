mod db;
mod enums;
mod messaging;
mod structs;
mod utils;
use db::get_collection;
use enums::Message::{File, Image, Text};
use enums::{Message, MessageFrom, MessageType};
use futures::stream::{StreamExt, TryStreamExt};
use image::ImageReader;
use messaging::{get_messages, insert_text_message};
use mongodb::{
    Client, Collection, Database,
    bson::{Document, doc},
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use structs::messages::{
    file::FileMessagePackage, image::ImageMessagePackage, text::TextMessagePackage,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {

    // =============================
    // Get messages
    // =============================
    let messages = get_messages().await?;
    println!("Messages {:#?}", messages);

    // =============================
    // Insert 5 text messages
    // =============================
    let v = vec![0,1,2,3,4];
    for (index, item) in v.iter().enumerate() {
        let text = format!("Hello, world{index}");
        let new_text_message: Result<TextMessagePackage<'_>, String> = TextMessagePackage::new(
            MessageFrom::User,
            &text
            
        );

        match new_text_message {
            Ok(message) => {
                println!("Message {:?}", message);
                let result = insert_text_message(message).await?;
                println!("Result {:?}", result);
            }
            Err(e) => {
                println!("Error creating a message to insert {:?}", e);
            }
        }
    }

    
    
    Ok(())
}

pub fn print_details(message_package: Message) {
    match message_package {
        Text(message) => {
            println!(
                "Message ({:?}) from {:?} on {:?}",
                message.message_id(),
                message.from(),
                message.pretty_timestamp()
            );
            println!("Message: {:?}", message.message());
        }
        Image(message) => {
            println!(
                "Message ({:?}) from {:?} on {:?}",
                message.message_id(),
                message.from(),
                message.pretty_timestamp()
            );
            println!(
                "Message image_data length: {:?}",
                message.image_data().len()
            );
        }
        File(message) => {
            println!(
                "Message ({:?}) from {:?} on {:?}",
                message.message_id(),
                message.from(),
                message.pretty_timestamp()
            );
            println!("Message image_data length: {:?}", message.file_data().len());
        }
    }
}
