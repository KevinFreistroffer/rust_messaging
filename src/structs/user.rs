use crate::enums::MessageFromType;
use crate::utils::gen_sender_id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    sender_id: u32,
    sender_type: MessageFromType,
}

impl User {
    pub fn new(sender_type: MessageFromType) -> Self {
        Self {
            sender_id: gen_sender_id(),
            sender_type,
        }
    }

    pub fn sender_id(&self) -> &u32 {
        &self.sender_id
    }

    pub fn sender_type(&self) -> &MessageFromType {
        &self.sender_type
    }
}
