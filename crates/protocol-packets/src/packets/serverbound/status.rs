use protocol_derive::{Packet, Readable, Writeable};

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x00]
pub struct StatusRequest {}

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x01]
pub struct PingRequest {
    pub payload: i64,
}
