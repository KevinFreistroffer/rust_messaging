use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SolanaError {
    #[error("Invalid public key: {0}")]
    InvalidPubkey(String),
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("RPC error: {0}")]
    RpcError(#[from] solana_client::client_error::ClientError),
}

pub struct SolanaTransfer {
    client: RpcClient,
}

impl SolanaTransfer {
    pub fn new(rpc_url: &str) -> Self {
        let client = RpcClient::new_with_commitment(
            rpc_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        Self { client }
    }

    pub fn new_devnet() -> Self {
        Self::new("https://api.devnet.solana.com")
    }

    pub fn new_mainnet() -> Self {
        Self::new("https://api.mainnet-beta.solana.com")
    }

    /// Get balance in lamports
    pub async fn get_balance(&self, pubkey: &str) -> Result<u64, SolanaError> {
        let pubkey = Pubkey::from_str(pubkey)
            .map_err(|_| SolanaError::InvalidPubkey(pubkey.to_string()))?;
        
        Ok(self.client.get_balance(&pubkey)?)
    }

    /// Get balance in SOL
    pub async fn get_balance_sol(&self, pubkey: &str) -> Result<f64, SolanaError> {
        let lamports = self.get_balance(pubkey).await?;
        Ok(lamports as f64 / LAMPORTS_PER_SOL as f64)
    }

    /// Transfer SOL between users
    pub async fn transfer_sol(
        &self,
        from_keypair: &Keypair,
        to_pubkey: &str,
        amount_sol: f64,
    ) -> Result<String, SolanaError> {
        let to_pubkey = Pubkey::from_str(to_pubkey)
            .map_err(|_| SolanaError::InvalidPubkey(to_pubkey.to_string()))?;
        
        let lamports = (amount_sol * LAMPORTS_PER_SOL as f64) as u64;
        
        // Check balance
        let balance = self.client.get_balance(&from_keypair.pubkey())?;
        if balance < lamports {
            return Err(SolanaError::InsufficientBalance);
        }

        // Create transfer instruction
        let instruction = system_instruction::transfer(
            &from_keypair.pubkey(),
            &to_pubkey,
            lamports,
        );

        // Get recent blockhash
        let recent_blockhash = self.client.get_latest_blockhash()?;

        // Create transaction
        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&from_keypair.pubkey()),
            &[from_keypair],
            recent_blockhash,
        );

        // Send transaction
        let signature = self.client.send_and_confirm_transaction(&transaction)
            .map_err(|e| SolanaError::TransactionFailed(e.to_string()))?;

        Ok(signature.to_string())
    }

    /// Airdrop SOL (only works on devnet/testnet)
    pub async fn request_airdrop(&self, pubkey: &str, sol_amount: f64) -> Result<String, SolanaError> {
        let pubkey = Pubkey::from_str(pubkey)
            .map_err(|_| SolanaError::InvalidPubkey(pubkey.to_string()))?;
        
        let lamports = (sol_amount * LAMPORTS_PER_SOL as f64) as u64;
        
        let signature = self.client.request_airdrop(&pubkey, lamports)?;
        self.client.confirm_transaction(&signature)?;
        
        Ok(signature.to_string())
    }
}

/// Helper function to create a new keypair
pub fn generate_keypair() -> (Keypair, String) {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    (keypair, pubkey)
}

/// Helper function to restore keypair from base58 private key
pub fn keypair_from_base58(base58_privkey: &str) -> Result<Keypair, SolanaError> {
    let bytes = bs58::decode(base58_privkey)
        .into_vec()
        .map_err(|_| SolanaError::InvalidPubkey("Invalid base58 private key".to_string()))?;
    
    Keypair::from_bytes(&bytes)
        .map_err(|_| SolanaError::InvalidPubkey("Invalid private key bytes".to_string()))
}

/// Helper function to export keypair as base58
pub fn keypair_to_base58(keypair: &Keypair) -> String {
    bs58::encode(keypair.to_bytes()).into_string()
}