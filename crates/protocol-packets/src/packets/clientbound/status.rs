use protocol_derive::{Packet, Readable, Writeable};

use crate::models::json_response::JsonResponse;

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x00]
pub struct StatusResponse {
    pub json_response: JsonResponse,
}

#[derive(Packet, Readable, Writeable)]
#[packet_id = 0x01]
pub struct PingResponse {
    pub payload: i64,
}
