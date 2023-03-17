#[async_trait::async_trait]
pub trait AsyncReadable
where
    Self: Sized,
{
    async fn read_async<T>(read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync;
}

#[async_trait::async_trait]
pub trait AsyncWriteable {
    async fn write_async<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync;
}
