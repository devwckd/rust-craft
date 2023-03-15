use protocol_derive::Packet;

#[derive(Packet)]
#[packet_id = 0x00]
pub struct StatusRequest {}
