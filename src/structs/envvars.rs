#[derive(Debug, Clone)]
pub struct EnvVars {
    pub send_text_messages: String,
    pub send_file_messages: String,
    pub send_image_messages: String,
    pub send_crypto_transfer_messages: String,
}
