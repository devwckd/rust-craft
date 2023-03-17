use protocol_core::data::Json;
use protocol_derive::Packet;

use crate::models::chat::Chat;

#[derive(Packet)]
#[packet_id = 0x00]
pub struct Disconnect {
    pub reason: Json<Chat>,
}
