use protocol_core::data::Json;
use protocol_derive::{Packet, Readable, Writeable};

use crate::models::chat::Chat;

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x00]
pub struct Disconnect {
    pub reason: Json<Chat>,
}

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x01]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: Vec<u8>,
    pub verify_token: Vec<u8>,
}

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x02]
pub struct LoginSuccess {}
