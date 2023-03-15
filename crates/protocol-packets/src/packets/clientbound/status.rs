use protocol_derive::Packet;

use crate::models::json_response::JsonResponse;

#[derive(Packet)]
#[packet_id = 0x00]
pub struct StatusResponse {
    pub json_response: JsonResponse,
}
