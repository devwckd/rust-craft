use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::rw::{Readable, Writeable};

pub struct Nbt<I> {
    inner: I,
}

impl<I> Debug for Nbt<I>
where
    I: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nbt").field("inner", &self.inner).finish()
    }
}

impl<I> Deref for Nbt<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I> DerefMut for Nbt<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl<I> Readable for Nbt<I>
where
    I: serde::de::DeserializeOwned,
{
    fn read<T>(read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let mut buf = Vec::new();
        read.read_to_end(&mut buf)?;

        Ok(Nbt {
            inner: nbt::from_reader::<_, I>(&buf[..])?,
        })
    }
}

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl<I> Writeable for Nbt<I>
where
    I: serde::Serialize,
{
    fn write<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        nbt::to_writer(&mut write, &self.inner, None)?;
        Ok(())
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl<I> Readable for Nbt<I>
where
    I: serde::de::DeserializeOwned,
{
    async fn read<T>(read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        use tokio::io::AsyncReadExt;

        let mut buf = Vec::new();
        read.read_to_end(&mut buf).await?;

        Ok(Nbt {
            inner: nbt::from_reader::<_, I>(&buf[..])?,
        })
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl<I> Writeable for Nbt<I>
where
    I: serde::Serialize + Send + Sync,
{
    async fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        use tokio::io::AsyncWriteExt;

        let mut buf = Vec::<u8>::new();
        nbt::to_writer(&mut buf, &self.inner, None)?;

        write.write_all(&buf[..]).await?;
        Ok(())
    }
}
