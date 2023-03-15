use std::ops::{Deref, DerefMut};

use crate::rw::{Readable, Writeable};

pub struct VarLong {
    inner: i64,
}

impl VarLong {
    pub fn new(value: i64) -> Self {
        Self { inner: value }
    }
}

impl From<i64> for VarLong {
    fn from(value: i64) -> Self {
        VarLong::new(value)
    }
}

impl From<VarLong> for i64 {
    fn from(value: VarLong) -> Self {
        value.inner
    }
}

impl Deref for VarLong {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for VarLong {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl Readable for VarLong {
    fn read<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: std::io::Read,
    {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            let read = u8::read(&mut read)?;
            let value = i64::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if num_read > 10 {
                return Err(anyhow::anyhow!("varlong to long"));
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }

        Ok(VarLong::new(result))
    }
}

#[cfg(all(feature = "rw-sync", not(feature = "rw-async-tokio")))]
impl Writeable for VarLong {
    fn write<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: std::io::Write,
    {
        let mut x = self.inner.clone();
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            temp.write(&mut write)?;

            if x == 0 {
                break;
            }
        }

        Ok(())
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl Readable for VarLong {
    async fn read<T>(mut read: &mut T) -> anyhow::Result<Self>
    where
        T: tokio::io::AsyncRead + std::marker::Unpin + Send + Sync,
    {
        let mut num_read = 0;
        let mut result = 0;

        loop {
            let read = u8::read(&mut read).await?;
            let value = i64::from(read & 0b0111_1111);
            result |= value.overflowing_shl(7 * num_read).0;

            num_read += 1;

            if num_read > 10 {
                return Err(anyhow::anyhow!("varlong too long"));
            }
            if read & 0b1000_0000 == 0 {
                break;
            }
        }

        Ok(VarLong::new(result))
    }
}

#[cfg(all(feature = "rw-async-tokio", not(feature = "rw-sync")))]
#[async_trait::async_trait]
impl Writeable for VarLong {
    async fn write<T>(&self, mut write: &mut T) -> anyhow::Result<()>
    where
        T: tokio::io::AsyncWrite + std::marker::Unpin + Send + Sync,
    {
        let mut x = self.inner.clone() as u32;
        loop {
            let mut temp = (x & 0b0111_1111) as u8;
            x >>= 7;
            if x != 0 {
                temp |= 0b1000_0000;
            }

            temp.write(&mut write).await?;

            if x == 0 {
                break;
            }
        }
        Ok(())
    }
}
