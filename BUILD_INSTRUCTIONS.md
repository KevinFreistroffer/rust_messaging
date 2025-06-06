# Build Instructions

## Current Build Issue

The project currently faces a transitive dependency issue where `base64ct v1.8.0` requires Rust edition 2024, which is not yet stable in Rust 1.82.0.

## Solutions

### Option 1: Use Rust Nightly (Recommended)

1. Install Rust nightly:
```bash
rustup install nightly
rustup default nightly
```

2. Build the project:
```bash
cargo build
cargo run
```

### Option 2: Use Specific Dependency Versions

If you prefer stable Rust, create a `.cargo/config.toml` file:

```toml
[patch.crates-io]
base64ct = { git = "https://github.com/RustCrypto/formats.git", rev = "OLDER_COMMIT" }
```

### Option 3: Docker Environment

Create a `Dockerfile`:

```dockerfile
FROM rust:1.75

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["cargo", "run"]
```

Build and run:
```bash
docker build -t messaging-app .
docker run -it messaging-app
```

### Option 4: Use Alternative Solana Versions

If the Solana dependencies are causing issues, you can try:

1. Update `Cargo.toml` to use older, more stable versions:
```toml
solana-program = "=1.10.32"
solana-client = "=1.10.32"
solana-sdk = "=1.10.32"
```

2. Clear cargo cache and rebuild:
```bash
rm -rf ~/.cargo/registry/cache
rm -rf ~/.cargo/registry/src
rm Cargo.lock
cargo update
cargo build
```

## MongoDB Setup

1. **Local MongoDB**:
```bash
# Install MongoDB
brew install mongodb-community  # macOS
sudo apt-get install mongodb     # Ubuntu

# Start MongoDB
mongod --dbpath /path/to/data
```

2. **MongoDB Atlas** (Cloud):
- Create free account at https://www.mongodb.com/atlas
- Create cluster
- Get connection string
- Update `src/db.rs` with your connection string

## Environment Variables

Create a `.env` file:
```env
MONGODB_URI=mongodb://localhost:27017
SOLANA_RPC_URL=https://api.devnet.solana.com
```

## Running the Application

Once dependencies are resolved:

```bash
# Run the demo
cargo run

# Run tests (if implemented)
cargo test

# Build for production
cargo build --release
```

## Verifying Functionality

1. **Check MongoDB Connection**:
```bash
mongo
> use rust
> db.messages.find()
```

2. **Test Solana Connection**:
```bash
solana --url devnet balance <PUBLIC_KEY>
```

## Troubleshooting

### Common Issues:

1. **"failed to parse manifest"**: Use Rust nightly or downgrade dependencies
2. **MongoDB connection failed**: Check connection string and network
3. **Solana RPC error**: Ensure you're using correct network (devnet/mainnet)

### Debug Commands:

```bash
# Check Rust version
rustc --version

# Check dependency tree
cargo tree

# Clean build
cargo clean
cargo build

# Verbose output
RUST_LOG=debug cargo run
```

## Alternative: Run Without Solana

If you only need the messaging functionality without crypto:

1. Comment out Solana dependencies in `Cargo.toml`
2. Comment out `mod solana_crypto;` in `lib.rs` and `main.rs`
3. Remove Solana-related code from `main.rs`

The messaging functionality will work independently.

## Support

For specific dependency issues:
- Rust: https://users.rust-lang.org/
- Solana: https://solana.stackexchange.com/
- MongoDB: https://www.mongodb.com/community/forums/