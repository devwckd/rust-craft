use protocol_derive::{Readable, Writeable};

#[derive(Readable, Writeable)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}
