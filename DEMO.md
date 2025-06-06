# Messaging Application Demo

This document demonstrates the complete functionality of the messaging application with MongoDB storage and Solana crypto transfers.

## Application Overview

The application provides:
- Multi-type messaging (text, image, file)
- MongoDB storage with advanced querying
- Solana blockchain integration for crypto transfers

## Core Components

### 1. Message Types

#### Text Messages
```rust
let text_msg = TextMessagePackage::new(
    user_id,          // Sender's unique ID
    MessageFrom::User, // User type (User/Agent)
    "Hello, world!"   // Message content
)?;
```

#### Image Messages
```rust
let image_data = fs::read("photo.jpg")?;
let image_msg = ImageMessagePackage::new(
    user_id,
    MessageFrom::User,
    image_data
)?;
```

#### File Messages
```rust
let file_data = fs::read("document.pdf")?;
let file_msg = FileMessagePackage::new(
    user_id,
    MessageFrom::Agent,
    "document.pdf".to_string(),
    file_data
)?;
```

### 2. User Management

Users have unique IDs and can be either regular users or agents:

```rust
let alice = User::new("Alice".to_string(), MessageFrom::User)
    .with_pubkey("11111111111111111111111111111111".to_string());

let support = User::new("Support Agent".to_string(), MessageFrom::Agent)
    .with_pubkey("33333333333333333333333333333333".to_string());
```

### 3. Message Storage

All messages are saved to MongoDB with complete metadata:

```rust
// Save different message types
save_text_message(text_msg).await?;
save_image_message(image_msg).await?;
save_file_message(file_msg).await?;
```

### 4. Message Retrieval

#### Get All Messages
```rust
let all_messages = get_all_messages().await?;
println!("Total messages: {}", all_messages.len());
```

#### Filter by Message Type
```rust
let text_messages = get_messages_by_type(MessageType::Text).await?;
let image_messages = get_messages_by_type(MessageType::Image).await?;
let file_messages = get_messages_by_type(MessageType::File).await?;
```

#### Filter by Date Range
```rust
use chrono::{Duration, Utc};

let end_date = Utc::now();
let start_date = end_date - Duration::days(7);
let week_messages = get_messages_by_date_range(start_date, end_date).await?;
```

#### Filter by User Type
```rust
let user_messages = get_messages_by_user_type(MessageFrom::User).await?;
let agent_messages = get_messages_by_user_type(MessageFrom::Agent).await?;
```

#### Filter by Sender ID
```rust
let alice_messages = get_messages_by_sender_id(alice.id).await?;
```

### 5. Solana Crypto Transfers

#### Initialize Solana Client
```rust
// For development/testing
let client = SolanaTransfer::new_devnet();

// For production
let client = SolanaTransfer::new_mainnet();
```

#### Generate Keypairs
```rust
let (alice_keypair, alice_pubkey) = generate_keypair();
let (bob_keypair, bob_pubkey) = generate_keypair();
```

#### Request Airdrop (Devnet Only)
```rust
let signature = client.request_airdrop(&alice_pubkey, 1.0).await?;
println!("Airdrop transaction: {}", signature);
```

#### Transfer SOL
```rust
let transfer_sig = client.transfer_sol(
    &alice_keypair,  // Sender's keypair
    &bob_pubkey,     // Receiver's public key
    0.5              // Amount in SOL
).await?;
println!("Transfer successful: {}", transfer_sig);
```

#### Check Balance
```rust
let balance = client.get_balance_sol(&alice_pubkey).await?;
println!("Balance: {} SOL", balance);
```

## MongoDB Document Structure

Each message is stored as a BSON document:

### Text Message Document
```json
{
  "_id": ObjectId("..."),
  "message_id": 12345,
  "sender_id": 67890,
  "type": "Text",
  "message": "Hello, world!",
  "from": "User",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### Image Message Document
```json
{
  "_id": ObjectId("..."),
  "message_id": 12346,
  "sender_id": 67890,
  "type": "Image",
  "image_data": [255, 216, 255, ...], // Binary data
  "from": "User",
  "timestamp": "2024-01-15T10:31:00Z"
}
```

### File Message Document
```json
{
  "_id": ObjectId("..."),
  "message_id": 12347,
  "sender_id": 11111,
  "type": "File",
  "file_name": "report.pdf",
  "file_data": [37, 80, 68, ...], // Binary data
  "from": "Agent",
  "timestamp": "2024-01-15T10:32:00Z"
}
```

## Example Usage Flow

1. **Create Users**
   ```rust
   let alice = User::new("Alice".to_string(), MessageFrom::User);
   let bob = User::new("Bob".to_string(), MessageFrom::User);
   ```

2. **Send Messages**
   ```rust
   // Alice sends a text message
   let msg = TextMessagePackage::new(alice.id, MessageFrom::User, "Hi Bob!")?;
   save_text_message(msg).await?;
   
   // Bob sends an image
   let img_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
   let img = ImageMessagePackage::new(bob.id, MessageFrom::User, img_data)?;
   save_image_message(img).await?;
   ```

3. **Query Messages**
   ```rust
   // Get all messages between Alice and Bob
   let alice_msgs = get_messages_by_sender_id(alice.id).await?;
   let bob_msgs = get_messages_by_sender_id(bob.id).await?;
   
   // Get messages from last hour
   let recent = get_messages_by_date_range(
       Utc::now() - Duration::hours(1),
       Utc::now()
   ).await?;
   ```

4. **Transfer Crypto**
   ```rust
   // Assuming wallets are funded
   let sig = client.transfer_sol(&alice_keypair, &bob.solana_pubkey.unwrap(), 0.1).await?;
   ```

## Security Considerations

1. **Private Keys**: Never store private keys in plain text
2. **MongoDB**: Use authentication and encryption
3. **File Uploads**: Implement size limits and virus scanning
4. **Solana**: Always verify transaction signatures

## Performance Optimizations

1. **Indexing**: Create MongoDB indexes on frequently queried fields
   ```javascript
   db.messages.createIndex({ "sender_id": 1 })
   db.messages.createIndex({ "timestamp": -1 })
   db.messages.createIndex({ "type": 1 })
   ```

2. **Pagination**: For large message sets
   ```rust
   let messages = collection
       .find(filter)
       .skip(page * page_size)
       .limit(page_size)
       .await?;
   ```

3. **Caching**: Consider Redis for frequently accessed messages

## Error Handling

The application uses Rust's Result type for comprehensive error handling:

```rust
match save_text_message(msg).await {
    Ok(_) => println!("Message saved successfully"),
    Err(e) => eprintln!("Failed to save message: {}", e),
}

match client.transfer_sol(&keypair, &pubkey, amount).await {
    Ok(sig) => println!("Transfer successful: {}", sig),
    Err(SolanaError::InsufficientBalance) => println!("Not enough SOL"),
    Err(e) => eprintln!("Transfer failed: {}", e),
}
```

## Testing

Run the demo with:
```bash
cargo run
```

This will:
1. Create sample users
2. Send various message types
3. Demonstrate all query capabilities
4. Show Solana integration (keypair generation)

For full Solana testing on devnet:
1. Generate keypairs
2. Request airdrops
3. Execute transfers
4. Verify balances

## Future Enhancements

- WebSocket support for real-time messaging
- Message encryption using Solana keys
- SPL token transfers
- Group messaging
- Message reactions and threading
- File preview generation
- Push notifications