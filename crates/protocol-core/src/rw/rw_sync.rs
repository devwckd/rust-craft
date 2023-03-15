pub trait Readable
where
    Self: Sized,
{
    fn read<T>(read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read;
}

pub trait Writeable {
    fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write;
}
