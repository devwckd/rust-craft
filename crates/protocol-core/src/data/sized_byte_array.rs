use std::ops::{Deref, DerefMut};

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::rw::{AsyncReadable, AsyncWriteable, SyncReadable, SyncWriteable};

use super::VarInt;

#[derive(Debug, Clone)]
pub struct SizedByteArray {
    inner: Vec<u8>,
}

impl SizedByteArray {
    pub fn new(value: Vec<u8>) -> Self {
        Self { inner: value }
    }

    pub fn inner(&self) -> &Vec<u8> {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl SyncReadable for SizedByteArray {
    fn read_sync<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let size = *VarInt::read_sync(&mut read)?;
        let mut buf = vec![0u8; size as usize];
        read.read_exact(&mut buf)?;
        Ok(Self { inner: buf })
    }
}

impl SyncWriteable for SizedByteArray {
    fn write_sync<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        VarInt::new(self.inner.len() as i32).write_sync(&mut write)?;
        write.write_all(&self.inner[..])?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncReadable for SizedByteArray {
    async fn read_async<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        let size = *VarInt::read_async(&mut read).await?;
        let mut buf = vec![0u8; size as usize];
        read.read_exact(&mut buf).await?;
        Ok(Self { inner: buf })
    }
}

#[async_trait::async_trait]
impl AsyncWriteable for SizedByteArray {
    async fn write_async<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        VarInt::new(self.inner.len() as i32)
            .write_async(&mut write)
            .await?;
        write.write_all(&self.inner[..]).await?;
        Ok(())
    }
}

impl Deref for SizedByteArray {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for SizedByteArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<Vec<u8>> for SizedByteArray {
    fn from(value: Vec<u8>) -> Self {
        Self { inner: value }
    }
}

impl From<&[u8]> for SizedByteArray {
    fn from(value: &[u8]) -> Self {
        Self {
            inner: value.into(),
        }
    }
}
