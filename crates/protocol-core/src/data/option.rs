#[cfg(feature = "sync")]
impl<I> crate::rw::SyncReadable for Option<I>
where
    I: crate::rw::SyncReadable,
{
    fn read_sync<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let bool_byte = u8::read_sync(&mut read)?;
        if bool_byte == 0u8 {
            return Ok(None);
        }

        Ok(Some(I::read_sync(&mut read)?))
    }
}

#[cfg(feature = "sync")]
impl<I> crate::rw::SyncWriteable for Option<I>
where
    I: crate::rw::SyncWriteable,
{
    fn write_sync<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let Some(value) = self else {
            0u8.write_sync(&mut write)?;
            return Ok(());
        };

        1u8.write_sync(&mut write)?;
        value.write_sync(&mut write)?;
        Ok(())
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<I> crate::rw::AsyncReadable for Option<I>
where
    I: crate::rw::AsyncReadable + Send + Sync,
{
    async fn read_async<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        let bool_byte = u8::read_async(&mut read).await?;
        if bool_byte == 0u8 {
            return Ok(None);
        }

        Ok(Some(I::read_async(&mut read).await?))
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait]
impl<I> crate::rw::AsyncWriteable for Option<I>
where
    I: crate::rw::AsyncWriteable + Send + Sync,
{
    async fn write_async<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        let Some(value) = self else {
            0u8.write_async(&mut write).await?;
            return Ok(());
        };

        1u8.write_async(&mut write).await?;
        value.write_async(&mut write).await?;
        Ok(())
    }
}
