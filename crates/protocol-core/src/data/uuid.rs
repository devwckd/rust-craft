use uuid::Uuid;

use crate::rw::{AsyncReadable, AsyncWriteable, SyncReadable, SyncWriteable};

impl SyncReadable for Uuid {
    fn read_sync<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let msb = u64::read_sync(&mut read)?;
        let lsb = u64::read_sync(&mut read)?;

        Ok(Uuid::from_u64_pair(msb, lsb))
    }
}

impl SyncWriteable for Uuid {
    fn write_sync<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let (msb, lsb) = self.as_u64_pair();
        msb.write_sync(&mut write)?;
        lsb.write_sync(&mut write)?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncReadable for Uuid {
    async fn read_async<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        let msb = u64::read_async(&mut read).await?;
        let lsb = u64::read_async(&mut read).await?;

        Ok(Uuid::from_u64_pair(msb, lsb))
    }
}

#[async_trait::async_trait]
impl AsyncWriteable for Uuid {
    async fn write_async<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        let (msb, lsb) = self.as_u64_pair();
        msb.write_async(&mut write).await?;
        lsb.write_async(&mut write).await?;
        Ok(())
    }
}
