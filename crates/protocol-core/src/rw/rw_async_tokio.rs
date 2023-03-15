#[async_trait::async_trait]
pub trait Readable
where
    Self: Sized,
{
    async fn read<T>(read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync;
}

#[async_trait::async_trait]
pub trait Writeable {
    async fn write<T>(&self, write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync;
}
