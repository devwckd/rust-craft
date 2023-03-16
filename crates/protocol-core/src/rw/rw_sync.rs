pub trait SyncReadable
where
    Self: Sized,
{
    fn read_sync<T>(read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read;
}

pub trait SyncWriteable {
    fn write_sync<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write;
}
