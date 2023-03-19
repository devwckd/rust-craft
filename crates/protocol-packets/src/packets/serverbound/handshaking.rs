use protocol_core::data::VarInt;
use protocol_derive::{Packet, Readable, Writeable};

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x00]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}
