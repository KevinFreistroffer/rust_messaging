mod enums;
mod structs;
mod messaging;
mod utils;
use enums::MessageFrom;
use enums::Message;
use enums::Message::{Image, Text, File};
use structs::messages::{image::ImageMessagePackage, text::MessagePackage, file::FileMessagePackage};
use image::ImageReader;
// use structs::messages::text::ImageMessagePackage;
// use std::fs::File;

fn main() {

    let send_text = false;
    let send_image_text = false;
    let send_file_text = true;
    
    if send_text {
        // =============================================
        // Text Message Handling Section
        // This section demonstrates creating and managing text messages
        // Future sections will include Image and File message handling
        // =============================================
        
        // Create sample text messages
        let agent_message = MessagePackage::new(MessageFrom::Agent,String::from("Hello, how are you?"));
        let user_message = MessagePackage::new(MessageFrom::User, String::from("Good, you?"));

        // Add messages to history
        match agent_message {
            Ok(msg) => messaging::add_to_history(Text(msg)),
            Err(error) => println!("{}", error)
        }

        match user_message {
            Ok(msg) => messaging::add_to_history(Text(msg)),
            Err(error) => println!("{}", error)
        }

        // Retrieve and display message history
        let history = messaging::get_history();

        if history.len() > 0 {
            // Example queries for text messages
            let message_by_id = messaging::find_message_by_id(123);
            let messages_by_text: Option<Vec<Message>>= messaging::find_messages_by_text(String::from("Hello, how are you?"));
            let messages_from_agent: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::Agent);
            let messages_from_user: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::User);
            
            if let Some(message) = message_by_id {
                // Display message details
                print_message_details(message);
            }
        
            if let Some(messages) = messages_by_text {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_agent {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_user {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
        }
    }
    
    if send_image_text {
        // =============================================
        // Image Message Handling Section
        // This section demonstrates creating and managing image messages
        // File message handling is implemented in the File variant of Message enum
        // =============================================
        
        // Create sample image messages


        let user_image = ImageReader::open("src/images/image.png");
        
        match user_image {
            Ok(image_reader) => {

                match image_reader.decode() {
                    Ok(image) => {
                        let agent_message: Result<ImageMessagePackage, String> = ImageMessagePackage::new(
                            MessageFrom::Agent,
                            image.clone().into_bytes(),
                        );
                        let user_message: Result<ImageMessagePackage, String> = ImageMessagePackage::new(
                            MessageFrom::User,
                            image.clone().into_bytes(),
                        );

                        match agent_message {
                            Ok(message) => messaging::add_to_history(Image(message)),
                            Err(error) => println!("Error creating a new Image message: {}", error)
                        }

                        match user_message {
                            Ok(message) => messaging::add_to_history(Image(message)),
                            Err(error) => println!("Error creating a new Image message: {}", error)
                        }
                    }
                    Err(error) => println!("Error decoding the image: {}", error)
                }
            },
            Err(error) => println!("error {:?}", error)
        }

        let _agent_message = MessagePackage::new(MessageFrom::Agent,String::from("Hello, how are you?"));
        let _user_message = MessagePackage::new(MessageFrom::User, String::from("Good, you?"));

        // Add image messages to history
        // match agent_message {
        //     Ok(msg) => messaging::add_to_history(msg),
        //     Err(error) => println!("{}", error)
        // }

        // match user_message {
        //     Ok(msg) => messaging::add_to_history(msg),
        //     Err(error) => println!("{}", error)
        // }

        // // Retrieve and display image message history
        let history = messaging::get_history();

        if history.len() > 0 {
            // Example queries for image messages
            let message_by_id = messaging::find_message_by_id(123);
            let messages_by_text: Option<Vec<Message>> = messaging::find_messages_by_text(String::from("Hello, how are you?"));
            let messages_from_agent: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::Agent);
            let messages_from_user: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::User);
        
            if let Some(message) = message_by_id {
                // Display image message details
                print_message_details(message);
            }
        
            if let Some(messages) = messages_by_text {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_agent {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_user {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
        }        
    }
    
    if send_file_text {
        // =============================================
        // Image Message Handling Section
        // This section demonstrates creating and managing image messages
        // File message handling is implemented in the File variant of Message enum
        // =============================================
        
        // Create sample file messages

        const FILE_PATH: &str = "src/files/file.txt";
        let file_data = std::fs::read(FILE_PATH);
        match file_data {
            Ok(data) => {
                let agent_message: Result<FileMessagePackage, String> = FileMessagePackage::new(
                    MessageFrom::Agent,
                    data.clone()
                );                
                let user_message: Result<FileMessagePackage, String> = FileMessagePackage::new(
                    MessageFrom::User,
                    data
                );

                match agent_message {
                    Ok(message) => messaging::add_to_history(File(message)),
                    Err(error) => println!("Could not add File message to history: {}", error)
                }

                match user_message {
                    Ok(message) => messaging::add_to_history(File(message)),
                    Err(error) => println!("Could not add File message to history: {}", error)
                }
            },
            Err(error) => println!("Error reading FILE: {:?}", error)
        }
        
        let history = messaging::get_history();

        if history.len() > 0 {
            // Example queries for image messages
            let message_by_id = messaging::find_message_by_id(123);
            let messages_by_text: Option<Vec<Message>> = messaging::find_messages_by_text(String::from("Hello, how are you?"));
            let messages_from_agent: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::Agent);
            let messages_from_user: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::User);
        
            if let Some(message) = message_by_id {
                // Display image message details
                print_message_details(message);
            }
        
            if let Some(messages) = messages_by_text {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_agent {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_user {
                for message in messages {
                    print_message_details(message);
                }
            }
            println!("--------------------------");
        }        
    }
    
    
    
}


pub fn print_message_details(message_package: Message) {
    match message_package {
        Text(message) => {
            println!("Message ({:?}) from {:?} on {:?}", message.id(), message.from(), message.pretty_timestamp());
            println!("Message: {:?}", message.message());
        },
        Image(message) => {
            println!("Message ({:?}) from {:?} on {:?}", message.id(), message.from(), message.pretty_timestamp());
            println!("Message image_data length: {:?}", message.image_data().len());
        },
        File(message) => {
            println!("Message ({:?}) from {:?} on {:?}", message.id(), message.from(), message.pretty_timestamp());
            println!("Message image_data length: {:?}", message.file_data().len());
        },
    }
}