use protocol_core::data::{Json, VarInt};
use protocol_derive::{Packet, Readable, Writeable};
use uuid::Uuid;

use crate::models::{chat::Chat, property::Property};

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
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: Vec<Property>,
}

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x03]
pub struct SetCompression {
    pub threshold: VarInt,
}
