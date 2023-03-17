use super::VarInt;

#[cfg(feature = "sync")]
impl<I> crate::rw::SyncReadable for Vec<I>
where
    I: crate::rw::SyncReadable,
{
    fn read_sync<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let size = *VarInt::read_sync(&mut read)?;

        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(I::read_sync(&mut read)?);
        }
        Ok(vec)
    }
}

#[cfg(feature = "sync")]
impl<I> crate::rw::SyncWriteable for Vec<I>
where
    I: crate::rw::SyncWriteable,
{
    fn write_sync<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let size = self.len();
        let mut buf = Vec::new();
        for ele in self {
            ele.write_sync(&mut buf)?;
        }

        VarInt::new(size as i32).write_sync(&mut write)?;
        write.write_all(&mut buf[..])?;

        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<I> crate::rw::AsyncReadable for Vec<I>
where
    I: crate::rw::AsyncReadable + Send + Sync,
{
    async fn read_async<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        let size = *VarInt::read_async(&mut read).await?;

        let mut vec = Vec::new();
        for _ in 0..size {
            vec.push(I::read_async(&mut read).await?);
        }
        Ok(vec)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<I> crate::rw::AsyncWriteable for Vec<I>
where
    I: crate::rw::AsyncWriteable + Send + Sync,
{
    async fn write_async<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        use tokio::io::AsyncWriteExt;
        let size = self.len();
        let mut buf = Vec::new();
        for ele in self {
            ele.write_async(&mut buf).await?;
        }

        VarInt::new(size as i32).write_async(&mut write).await?;
        write.write_all(&mut buf[..]).await?;

        Ok(())
    }
}
