mod db;
mod enums;
mod messaging;
mod structs;
mod utils;
mod solana_crypto;

use chrono::{Duration, Utc};
use enums::{MessageFrom, MessageType};
use messaging::*;
use solana_crypto::{SolanaTransfer, generate_keypair, keypair_to_base58};
use structs::{
    messages::{
        file::FileMessagePackage, 
        image::ImageMessagePackage, 
        text::TextMessagePackage
    },
    user::User,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Messaging Application Demo ===\n");

    // Create some users
    let user1 = User::new("Alice".to_string(), MessageFrom::User)
        .with_pubkey("11111111111111111111111111111111".to_string());
    let user2 = User::new("Bob".to_string(), MessageFrom::User)
        .with_pubkey("22222222222222222222222222222222".to_string());
    let agent = User::new("Support Agent".to_string(), MessageFrom::Agent)
        .with_pubkey("33333333333333333333333333333333".to_string());

    println!("Created users:");
    println!("- User 1: {} (ID: {})", user1.username, user1.id);
    println!("- User 2: {} (ID: {})", user2.username, user2.id);
    println!("- Agent: {} (ID: {})\n", agent.username, agent.id);

    // Save different types of messages
    println!("=== Saving Messages ===");

    // Text messages
    let text_msg1 = TextMessagePackage::new(
        user1.id,
        MessageFrom::User,
        "Hello, this is Alice!"
    )?;
    save_text_message(text_msg1).await?;
    println!("✓ Saved text message from Alice");

    let text_msg2 = TextMessagePackage::new(
        user2.id,
        MessageFrom::User,
        "Hi Alice, this is Bob!"
    )?;
    save_text_message(text_msg2).await?;
    println!("✓ Saved text message from Bob");

    let text_msg3 = TextMessagePackage::new(
        agent.id,
        MessageFrom::Agent,
        "Hello users, this is support."
    )?;
    save_text_message(text_msg3).await?;
    println!("✓ Saved text message from Agent");

    // Image message
    let fake_image_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // Fake JPEG header
    let image_msg = ImageMessagePackage::new(
        user1.id,
        MessageFrom::User,
        fake_image_data
    )?;
    save_image_message(image_msg).await?;
    println!("✓ Saved image message from Alice");

    // File message
    let fake_file_data = b"This is a test file content".to_vec();
    let file_msg = FileMessagePackage::new(
        user2.id,
        MessageFrom::User,
        "test_document.txt".to_string(),
        fake_file_data
    )?;
    save_file_message(file_msg).await?;
    println!("✓ Saved file message from Bob\n");

    // Retrieve all messages
    println!("=== Retrieving All Messages ===");
    let all_messages = get_all_messages().await?;
    println!("Total messages in database: {}\n", all_messages.len());

    // Retrieve messages by type
    println!("=== Retrieving Messages by Type ===");
    let text_messages = get_messages_by_type(MessageType::Text).await?;
    println!("Text messages: {}", text_messages.len());
    
    let image_messages = get_messages_by_type(MessageType::Image).await?;
    println!("Image messages: {}", image_messages.len());
    
    let file_messages = get_messages_by_type(MessageType::File).await?;
    println!("File messages: {}\n", file_messages.len());

    // Retrieve messages by user type
    println!("=== Retrieving Messages by User Type ===");
    let user_messages = get_messages_by_user_type(MessageFrom::User).await?;
    println!("Messages from Users: {}", user_messages.len());
    
    let agent_messages = get_messages_by_user_type(MessageFrom::Agent).await?;
    println!("Messages from Agents: {}\n", agent_messages.len());

    // Retrieve messages by date range
    println!("=== Retrieving Messages by Date Range ===");
    let end_date = Utc::now();
    let start_date = end_date - Duration::hours(1); // Last hour
    let recent_messages = get_messages_by_date_range(start_date, end_date).await?;
    println!("Messages in the last hour: {}\n", recent_messages.len());

    // Retrieve messages by sender ID
    println!("=== Retrieving Messages by Sender ID ===");
    let alice_messages = get_messages_by_sender_id(user1.id).await?;
    println!("Messages from Alice (ID: {}): {}", user1.id, alice_messages.len());
    
    let bob_messages = get_messages_by_sender_id(user2.id).await?;
    println!("Messages from Bob (ID: {}): {}\n", user2.id, bob_messages.len());

    // Solana Crypto Demo
    println!("=== Solana Crypto Transfer Demo ===");
    println!("Setting up Solana devnet connection...\n");

    // Create Solana client for devnet
    let solana_client = SolanaTransfer::new_devnet();

    // Generate keypairs for demo (in production, users would provide their own)
    let (alice_keypair, alice_pubkey) = generate_keypair();
    let (bob_keypair, bob_pubkey) = generate_keypair();
    
    println!("Generated Solana keypairs:");
    println!("- Alice's public key: {}", alice_pubkey);
    println!("- Bob's public key: {}\n", bob_pubkey);

    // Note: In a real application, you would:
    // 1. Request airdrops on devnet for testing
    // 2. Or use real SOL on mainnet
    // 3. Store private keys securely (never in plain text)
    
    println!("To test Solana transfers:");
    println!("1. Request devnet SOL airdrops for the generated addresses");
    println!("2. Use the transfer_sol function to send SOL between users");
    println!("3. Check balances using get_balance_sol\n");

    // Example of how to use the transfer function (commented out as it requires funded accounts)
    /*
    // Request airdrop for Alice (devnet only)
    let airdrop_sig = solana_client.request_airdrop(&alice_pubkey, 1.0).await?;
    println!("Airdrop transaction: {}", airdrop_sig);

    // Check Alice's balance
    let alice_balance = solana_client.get_balance_sol(&alice_pubkey).await?;
    println!("Alice's balance: {} SOL", alice_balance);

    // Transfer 0.5 SOL from Alice to Bob
    let transfer_sig = solana_client.transfer_sol(
        &alice_keypair,
        &bob_pubkey,
        0.5
    ).await?;
    println!("Transfer transaction: {}", transfer_sig);

    // Check balances after transfer
    let alice_balance = solana_client.get_balance_sol(&alice_pubkey).await?;
    let bob_balance = solana_client.get_balance_sol(&bob_pubkey).await?;
    println!("Alice's balance after transfer: {} SOL", alice_balance);
    println!("Bob's balance after transfer: {} SOL", bob_balance);
    */

    println!("=== Demo Complete ===");
    
    Ok(())
}
