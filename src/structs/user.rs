use serde::{Deserialize, Serialize};
use crate::utils::gen_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub user_type: crate::enums::MessageFrom,
    pub solana_pubkey: Option<String>, // Base58 encoded public key
}

impl User {
    pub fn new(username: String, user_type: crate::enums::MessageFrom) -> Self {
        Self {
            id: gen_id(),
            username,
            user_type,
            solana_pubkey: None,
        }
    }

    pub fn with_pubkey(mut self, pubkey: String) -> Self {
        self.solana_pubkey = Some(pubkey);
        self
    }
}