use crate::structs::envvars::EnvVars;
use dotenv::dotenv;
use std::env;

pub fn validate_and_get_env_vars() -> EnvVars {
    dotenv().ok();

    // ======= SEND_TEXT_MESSAGES =======
    let send_text_messages = env::var("SEND_TEXT_MESSAGES").unwrap_or_else(|e| {
        println!("Couldn't read SEND_TEXT_MESSAGES ({})", e);
        String::new()
    });

    if send_text_messages.is_empty() {
        eprintln!(
            "The send_text_messages environment variable is not set. By default it's set to false. Please verify you want to allow sending text messages."
        );
    }

    // ======= SEND_FILE_MESSAGES =======
    let send_file_messages = env::var("SEND_FILE_MESSAGES").unwrap_or_else(|e| {
        println!("Couldn't read env var SEND_FILE_MESSAGES ({})", e);
        String::new()
    });

    if send_file_messages.is_empty() {
        eprintln!(
            "The send_file_messages environment variable is not set. By default it's set to false. Please verify you want to allow sending file messages."
        );
    }

    // ======= SEND_IMAGE_MESSAGES =======
    let send_image_messages = env::var("SEND_IMAGE_MESSAGES").unwrap_or_else(|e| {
        println!("Couldn't read env var SEND_IMAGE_MESSAGES ({})", e);
        String::new()
    });

    if send_image_messages.is_empty() {
        eprintln!(
            "The send_image_messages environment variable is not set. By default it's set to false. Please verify you want to allow sending image messages."
        );
    }

    // ======= SEND_CRYPTO_TRANSFER_MESSAGES =======
    let send_crypto_transfer_messages =
        env::var("SEND_CRYPTO_TRANSFER_MESSAGES").unwrap_or_else(|e| {
            println!(
                "Couldn't read env var SEND_CRYPTO_TRANSFER_MESSAGES ({})",
                e
            );
            String::new()
        });

    if send_crypto_transfer_messages.is_empty() {
        eprintln!(
            "The send_crypto_transfer_messages environment variable is not set. By default it's set to false. Please verify you want to allow sending crypto transfer messages."
        );
    }

    return EnvVars {
        send_text_messages,
        send_file_messages,
        send_image_messages,
        send_crypto_transfer_messages,
    };
}
