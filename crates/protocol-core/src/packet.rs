pub trait Packet
where
    Self: Sized,
{
    const ID: i32;
}
