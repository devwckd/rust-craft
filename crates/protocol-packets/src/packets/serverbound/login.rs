use protocol_derive::{Packet, Readable, Writeable};
use uuid::Uuid;

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x00]
pub struct LoginStart {
    pub name: String,
    pub uuid: Option<Uuid>,
}
