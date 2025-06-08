use crate::enums::{MessageFromType, MessageType};
use crate::utils::gen_message_id;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CryptoTransferMessagePackage<'a> {
    message_id: u32,
    r#type: MessageType,
    from: MessageFromType,
    recipient_address: &'a str,
    amount: f64,
    token_symbol: &'a str,
    timestamp: DateTime<Utc>,
    transaction_signature: Option<String>,
}

impl<'a> CryptoTransferMessagePackage<'a> {
    pub fn new(
        from: MessageFromType,
        recipient_address: &'a str,
        amount: f64,
        token_symbol: &'a str,
    ) -> Result<Self, String> {
        // Validate Solana address format (base58, 32-44 characters)
        if !recipient_address.chars().all(|c| c.is_alphanumeric())
            || recipient_address.len() < 32
            || recipient_address.len() > 44
        {
            return Err("Invalid Solana address format".into());
        }

        // Validate amount
        if amount <= 0.0 {
            return Err("Amount must be greater than 0".into());
        }

        Ok(Self {
            message_id: gen_message_id(),
            r#type: MessageType::CryptoTransfer,
            from,
            recipient_address,
            amount,
            token_symbol,
            timestamp: Utc::now(),
            transaction_signature: None,
        })
    }

    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    pub fn r#type(&self) -> MessageType {
        self.r#type.clone()
    }

    pub fn from(&self) -> MessageFromType {
        self.from.clone()
    }

    pub fn recipient_address(&self) -> &str {
        self.recipient_address
    }

    pub fn amount(&self) -> f64 {
        self.amount
    }

    pub fn token_symbol(&self) -> &str {
        self.token_symbol
    }

    pub fn timestamp(&self) -> String {
        self.timestamp.to_string()
    }

    pub fn pretty_timestamp(&self) -> String {
        self.timestamp
            .format("%A, %B, %e, %Y at %I:%M%p")
            .to_string()
    }

    pub fn set_transaction_signature(&mut self, signature: String) {
        self.transaction_signature = Some(signature);
    }

    pub fn transaction_signature(&self) -> Option<&str> {
        self.transaction_signature.as_deref()
    }
}
