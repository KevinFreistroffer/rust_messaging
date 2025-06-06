# Messaging Application with Solana Crypto Transfers

A Rust-based messaging application that stores text, image, and file messages in MongoDB and enables crypto transfers between users using Solana.

## Features

### Messaging Capabilities
- **Multiple Message Types**: Support for text, image, and file messages
- **User Management**: Users with unique IDs and types (Agent/User)
- **Message Storage**: All messages stored in MongoDB with metadata
- **Advanced Querying**:
  - Retrieve all messages
  - Filter by message type (text/image/file)
  - Filter by date range
  - Filter by user type (Agent/User)
  - Filter by specific sender ID

### Solana Crypto Integration
- Send SOL between users
- Check wallet balances
- Generate new Solana keypairs
- Support for devnet and mainnet
- Airdrop functionality for testing (devnet only)

## Prerequisites

- Rust 1.70 or higher
- MongoDB instance (local or cloud)
- Solana CLI tools (optional, for advanced usage)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd messaging
```

2. Update MongoDB connection string in `src/db.rs`:
```rust
let uri = "your-mongodb-connection-string";
```

3. Build the project:
```bash
cargo build --release
```

## Usage

### Running the Demo

```bash
cargo run
```

This will execute a comprehensive demo that:
1. Creates sample users (Alice, Bob, and a Support Agent)
2. Saves various types of messages
3. Demonstrates all query capabilities
4. Shows Solana keypair generation

### API Examples

#### Creating and Saving Messages

```rust
use messaging::*;
use structs::messages::text::TextMessagePackage;
use enums::MessageFrom;

// Create a text message
let message = TextMessagePackage::new(
    user_id,
    MessageFrom::User,
    "Hello, world!"
)?;

// Save to MongoDB
save_text_message(message).await?;
```

#### Querying Messages

```rust
// Get all messages
let all_messages = get_all_messages().await?;

// Get messages by type
let text_messages = get_messages_by_type(MessageType::Text).await?;

// Get messages in date range
let start = Utc::now() - Duration::days(7);
let end = Utc::now();
let week_messages = get_messages_by_date_range(start, end).await?;

// Get messages from agents only
let agent_messages = get_messages_by_user_type(MessageFrom::Agent).await?;
```

#### Solana Transfers

```rust
use solana_crypto::{SolanaTransfer, generate_keypair};

// Initialize Solana client
let client = SolanaTransfer::new_devnet();

// Generate keypairs
let (sender_keypair, sender_pubkey) = generate_keypair();
let (receiver_keypair, receiver_pubkey) = generate_keypair();

// Request airdrop (devnet only)
client.request_airdrop(&sender_pubkey, 1.0).await?;

// Transfer SOL
let signature = client.transfer_sol(
    &sender_keypair,
    &receiver_pubkey,
    0.5
).await?;

// Check balance
let balance = client.get_balance_sol(&receiver_pubkey).await?;
```

## Project Structure

```
messaging/
├── src/
│   ├── main.rs              # Demo application
│   ├── lib.rs               # Library exports
│   ├── db.rs                # MongoDB connection
│   ├── messaging.rs         # Message operations
│   ├── solana_crypto.rs     # Solana integration
│   ├── utils.rs             # Utility functions
│   ├── enums.rs             # Message types and enums
│   └── structs/
│       ├── user.rs          # User structure
│       └── messages/
│           ├── text.rs      # Text message structure
│           ├── image.rs     # Image message structure
│           └── file.rs      # File message structure
├── Cargo.toml               # Dependencies
└── README.md                # This file
```

## Message Structure

All messages contain:
- `message_id`: Unique identifier
- `sender_id`: ID of the sending user
- `type`: Message type (Text/Image/File)
- `from`: User type (Agent/User)
- `timestamp`: UTC timestamp

Additional fields by type:
- **Text**: `message` (string content)
- **Image**: `image_data` (byte array)
- **File**: `file_name` and `file_data`

## Security Considerations

1. **MongoDB**: Update the connection string in `src/db.rs` with your own credentials
2. **Solana Private Keys**: Never store private keys in plain text in production
3. **File Size Limits**: Consider implementing size limits for image and file uploads
4. **Rate Limiting**: Implement rate limiting for message sending in production

## Dependencies

- `mongodb`: Database storage
- `solana-program`, `solana-client`, `solana-sdk`: Solana blockchain integration
- `tokio`: Async runtime
- `chrono`: Date/time handling
- `serde`: Serialization
- `thiserror`: Error handling

## Future Enhancements

- Message encryption
- Group messaging/channels
- Message editing and deletion
- Rich media previews
- SPL token transfers (not just SOL)
- WebSocket support for real-time messaging
- REST API endpoints

## License

This project is open source. Please check the LICENSE file for details.