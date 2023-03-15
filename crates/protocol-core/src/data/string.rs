use crate::{
    data::VarInt,
    rw::{Readable, Writeable},
};

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl Readable for String {
    fn read<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let size = *VarInt::read(&mut read)?;
        let mut buf = vec![0u8; size as usize];
        read.read_exact(&mut buf)?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl Writeable for String {
    fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let size: VarInt = (self.len() as i32).into();
        size.write(write)?;
        write.write(self.as_bytes())?;

        Ok(())
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl Readable for String {
    async fn read<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        use tokio::io::AsyncReadExt;
        let size = *VarInt::read(&mut read).await?;
        let mut buf = vec![0u8; size as usize];
        read.read_exact(&mut buf).await?;
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl Writeable for String {
    async fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        use tokio::io::AsyncWriteExt;
        let size: VarInt = (self.len() as i32).into();
        size.write(write).await?;
        write.write(self.as_bytes()).await?;

        Ok(())
    }
}
