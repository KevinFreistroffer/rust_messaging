use ring::rand::{SecureRandom, SystemRandom};

pub fn gen_message_id() -> u32 {
    let range = SystemRandom::new();
    let mut bytes = [0u8; 4];
    range
        .fill(&mut bytes)
        .expect("Failed to generate random bytes");
    u32::from_be_bytes(bytes)
}

pub fn gen_sender_id() -> u32 {
    let range = SystemRandom::new();
    let mut bytes = [0u8; 4];
    range
        .fill(&mut bytes)
        .expect("Failed to generate random bytes");
    u32::from_be_bytes(bytes)
}
