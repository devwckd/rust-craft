use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::rw::{Readable, Writeable};

pub struct Json<I> {
    inner: I,
}

impl<I> Json<I> {
    pub fn new(data: I) -> Self {
        Self { inner: data }
    }
}

impl<I> Debug for Json<I>
where
    I: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Json").field("inner", &self.inner).finish()
    }
}

impl<I> Deref for Json<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<I> DerefMut for Json<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl<I> Readable for Json<I>
where
    I: serde::de::DeserializeOwned + serde::Serialize + Send + Sync,
{
    fn read<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let raw = String::read(&mut read)?;
        let deserialized = serde_json::from_str(&raw)?;
        Ok(Json::new(deserialized))
    }
}

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl<I> Writeable for Json<I>
where
    I: serde::de::DeserializeOwned + serde::Serialize + Send + Sync,
{
    fn write<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let raw = serde_json::to_string(&self.inner)?;
        raw.write(&mut write)?;
        Ok(())
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl<I> Readable for Json<I>
where
    I: serde::de::DeserializeOwned + serde::Serialize + Send + Sync,
{
    async fn read<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        let raw = String::read(&mut read).await?;
        let deserialized = serde_json::from_str(&raw)?;
        Ok(Json::new(deserialized))
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl<I> Writeable for Json<I>
where
    I: serde::de::DeserializeOwned + serde::Serialize + Send + Sync,
{
    async fn write<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        let raw = serde_json::to_string(&self.inner)?;
        raw.write(&mut write).await?;
        Ok(())
    }
}
