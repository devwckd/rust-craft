use protocol_core::data::VarInt;
use protocol_derive::{Readable, Writeable};

#[derive(Readable, Writeable)]
pub struct Statistic {
    pub category_id: VarInt,
    pub statistic_id: VarInt,
    pub value: VarInt,
}
