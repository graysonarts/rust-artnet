use std::marker::PhantomData;
use std::net::{ToSocketAddrs, UdpSocket};

pub(crate) trait FromRaw<T> {
    fn from_raw(raw: &[u8]) -> Option<T>;
}

#[derive(Debug)]
pub(super) enum AsyncError {
    GeneralError,
    UnableToBind,
    ReadError,

    Noncritical(AsyncResumableErrors),
}

#[derive(Debug)]
pub(super) enum AsyncResumableErrors {
    NoData,
}

pub(super) struct AsyncPacketReceiver<T>
where
    T: FromRaw<T>,
{
    socket: UdpSocket,
    cache: [u8; 1024],
    _type: PhantomData<T>,
}

impl<T> AsyncPacketReceiver<T>
where
    T: FromRaw<T>,
{
    pub fn create<A: ToSocketAddrs>(addr: A) -> Result<Self, AsyncError> {
        let socket = UdpSocket::bind(addr).map_err(|_| AsyncError::UnableToBind)?;

        Ok(AsyncPacketReceiver {
            socket,
            cache: [0; 1024],
            _type: PhantomData::default(),
        })
    }

    /// Note: This is blocking for now
    pub fn poll(&mut self) -> Result<T, AsyncError> {
        let (bytes_read, addr) = self
            .socket
            .recv_from(&mut self.cache)
            .map_err(|e| AsyncError::ReadError)?;
        println!("{}: {:#?}", bytes_read, addr);

        let slice = &self.cache[..bytes_read];
        println!("{:?}", slice);
        T::from_raw(slice).map_or(Err(AsyncError::GeneralError), |s| Ok(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl FromRaw<()> for () {
        fn from_raw(data: &[u8]) -> Option<()> {
            Some(())
        }
    }

    fn send_test_packet() {
        let socket =
            UdpSocket::bind("0.0.0.0:6453").expect("Unable to bind to udp socket for send");
        socket
            .send_to(
                &[65, 114, 116, 45, 78, 101, 116, 0, 0, 32, 0, 14, 0, 0],
                "127.0.0.1:6454",
            )
            .expect("Unable to send to 6454");
    }

    #[test]
    fn test_async_recv() {
        let mut v =
            AsyncPacketReceiver::<()>::create("0.0.0.0:6454").expect("Unable to create receiver");
        send_test_packet();
        let packet = v.poll().expect("");
    }
}
