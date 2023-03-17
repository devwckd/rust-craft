use crate::data::VarInt;

#[cfg(feature = "sync")]
impl crate::rw::SyncReadable for String {
    fn read_sync<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let size = *VarInt::read_sync(&mut read)?;
        let mut buf = vec![0u8; size as usize];
        read.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

#[cfg(feature = "sync")]
impl crate::rw::SyncWriteable for String {
    fn write_sync<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let size: VarInt = (self.len() as i32).into();
        size.write_sync(write)?;
        write.write(self.as_bytes())?;

        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl crate::rw::AsyncReadable for String {
    async fn read_async<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        use tokio::io::AsyncReadExt;
        let size = *VarInt::read_async(&mut read).await?;
        let mut buf = vec![0u8; size as usize];
        read.read_exact(&mut buf).await?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl crate::rw::AsyncWriteable for String {
    async fn write_async<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        use tokio::io::AsyncWriteExt;
        let size: VarInt = (self.len() as i32).into();
        size.write_async(write).await?;
        write.write(self.as_bytes()).await?;

        Ok(())
    }
}
