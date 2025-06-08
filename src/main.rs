mod db;
mod enums;
mod messaging;
mod structs;
mod utils;
mod validate_and_get_env_vars;

use dotenv::dotenv;
use enums::Message::{CryptoTransfer, File, Image, Text};
use enums::{Message, MessageFromType, MessageType};
use futures::stream::{StreamExt, TryStreamExt};
use image::ImageReader;
use messaging::{debug_saved_message, get_messages, insert_text_message};
use mongodb::{
    Client, Collection, Database,
    bson::{Document, doc},
    options::{ClientOptions, ServerApi, ServerApiVersion},
};
use std::env;
use structs::envvars::EnvVars;
use structs::messages::{
    crypto::CryptoTransferMessagePackage, file::FileMessagePackage, image::ImageMessagePackage,
    text::TextMessagePackage,
};
use structs::user::User;
use validate_and_get_env_vars::validate_and_get_env_vars;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let EnvVars {
        send_text_messages,
        send_file_messages,
        send_image_messages,
        send_crypto_transfer_messages,
    } = validate_and_get_env_vars();

    // ===== Get all messages from the database =====
    let messages = get_messages().await?;
    println!("Messages {:#?}", messages);

    // ===== Send Text Messages =====
    if !send_text_messages.is_empty() {
        let user = User::new(MessageFromType::User);
        let agent = User::new(MessageFromType::Agent);

        // ===== Create a User text message =====
        let text = "USER text message";
        let user_text_message: Result<TextMessagePackage<'_>, String> =
            TextMessagePackage::new(user, &text);

        match user_text_message {
            Ok(message) => {
                println!("Message {:?}", message);
                let result = insert_text_message(message).await?;

                if let Some(document) = result {
                    debug_saved_message(document, true);
                }
            }
            Err(e) => {
                println!("Error creating a message to insert {:?}", e);
            }
        }

        // =============================
        // Insert an Agent text message
        // =============================
        let text = "AGENT text message";
        let agent_text_message: Result<TextMessagePackage<'_>, String> =
            TextMessagePackage::new(agent, &text);

        match agent_text_message {
            Ok(message) => {
                println!("Message {:?}", message);
                let result = insert_text_message(message).await?;
                if let Some(document) = result {
                    debug_saved_message(document, true);
                }
            }
            Err(e) => {
                println!("Error creating a message to insert {:?}", e);
            }
        }
    }

    // if send_file_text {
    //     // =============================================
    //     // Image Message Handling Section
    //     // This section demonstrates creating and managing image messages
    //     // File message handling is implemented in the File variant of Message enum
    //     // =============================================

    //     // Create sample file messages

    //     const FILE_PATH: &str = "src/files/file.txt";
    //     let file_data = std::fs::read(FILE_PATH);
    //     match file_data {
    //         Ok(data) => {
    //             let agent_message: Result<FileMessagePackage, String> = FileMessagePackage::new(
    //                 MessageFrom::Agent,
    //                 data.clone()
    //             );
    //             let user_message: Result<FileMessagePackage, String> = FileMessagePackage::new(
    //                 MessageFrom::User,
    //                 data
    //             );

    //             match agent_message {
    //                 Ok(message) => messaging::add_to_history(File(message)),
    //                 Err(error) => println!("Could not add File message to history: {}", error)
    //             }

    //             match user_message {
    //                 Ok(message) => messaging::add_to_history(File(message)),
    //                 Err(error) => println!("Could not add File message to history: {}", error)
    //             }
    //         },
    //         Err(error) => println!("Error reading FILE: {:?}", error)
    //     }

    // }

    Ok(())
}
