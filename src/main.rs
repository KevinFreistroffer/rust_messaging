mod enums;
mod structs;
mod messaging;
mod utils;
use enums::MessageFrom;
use enums::Message;
use enums::Message::{Image, Text};
use structs::messages::{image::ImageMessagePackage, text::MessagePackage};
use image::ImageReader;
// use structs::messages::text::ImageMessagePackage;

fn main() {

    let send_text = false;
    let send_image_text = true;
    
    if send_text {
        // =============================================
        // Text Message Handling Section
        // This section demonstrates creating and managing text messages
        // Future sections will include Image and File message handling
        // =============================================
        
        // Create sample text messages
        let agent_message = MessagePackage::new(MessageFrom::Agent,"Hello, how are you?".to_string());
        let user_message = MessagePackage::new(MessageFrom::User,"Good, you?".to_string());

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
            let messages_by_text: Option<Vec<Message>>= messaging::find_messages_by_text("Hello, how are you?".to_string());
            let messages_from_agent: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::Agent);
            let messages_from_user: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::User);
        
            // Display message details
            print_message_details(message_by_id);
        
            if let Some(messages) = messages_by_text {
                for msg in messages {
                    print_message_details(Some(msg));
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_agent {
                for msg in messages {
                    print_message_details(Some(msg));
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_user {
                for msg in messages {
                    print_message_details(Some(msg));
                }
            }
            println!("--------------------------");
        }
    }
    
    
    if send_image_text {
        // =============================================
        // Image Message Handling Section
        // This section demonstrates creating and managing image messages
        // Future sections will include File message handling
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
                            Err(error) => println!("error {}", error)
                        }

                        match user_message {
                            Ok(message) => messaging::add_to_history(Image(message)),
                            Err(error) => println!("error")
                        }
                    }
                    Err(error) => println!("Error with the image: {}", error)
                }
            },
            Err(error) => println!("error {:?}", error)
        }

        let agent_message = MessagePackage::new(MessageFrom::Agent,"Hello, how are you?".to_string());
        let user_message = MessagePackage::new(MessageFrom::User,"Good, you?".to_string());

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
            let messages_by_text: Option<Vec<Message>> = messaging::find_messages_by_text("Hello, how are you?".to_string());
            let messages_from_agent: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::Agent);
            let messages_from_user: Option<Vec<Message>> = messaging::find_messages_from_sender(MessageFrom::User);
        
            // Display image message details
            print_message_details(message_by_id);
        
            if let Some(messages) = messages_by_text {
                for msg in messages {
                    print_message_details(Some(msg));
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_agent {
                for msg in messages {
                    print_message_details(Some(msg));
                }
            }
            println!("--------------------------");
            if let Some(messages) = messages_from_user {
                for msg in messages {
                    print_message_details(Some(msg));
                }
            }
            println!("--------------------------");
        }        
    }
    
    
    
}


pub fn print_message_details(message_package: Option<Message>) {
    if let Some(msg) = message_package {
       
        match msg {
            Text(message) => {
                println!("Message ({:?}) from {:?} on {:?}", message.id(), message.from(), message.pretty_timestamp());
                println!("Message: {:?}", message.message());
            },
            Image(message) => {
                println!("Message ({:?}) from {:?} on {:?}", message.id(), message.from(), message.pretty_timestamp());
                println!("Message image_data length: {:?}", message.image_data().len());
            }
        }
    } else {
        println!("This message does not exist.");
    }
}