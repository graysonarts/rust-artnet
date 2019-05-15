use std::marker::PhantomData;
use std::net::{ToSocketAddrs, UdpSocket};

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

pub(super) struct AsyncPacketReceiver<T> {
    socket: UdpSocket,
		cache: [u8; 1024],
    _type: PhantomData<T>,
}

impl<T> AsyncPacketReceiver<T> {
    pub fn create<A: ToSocketAddrs>(addr: A) -> Result<Self, AsyncError> {
        let socket = UdpSocket::bind(addr).map_err(|_| AsyncError::UnableToBind)?;

        Ok(AsyncPacketReceiver {
            socket,
						cache: [0; 1024],
            _type: PhantomData::default(),
        })
    }

    pub fn poll(&mut self) -> Result<Self, AsyncError> {
			let (bytes_read, addr) = self.socket.recv_from(&mut self.cache).map_err(|e| AsyncError::ReadError)?;
			println!("{}: {:#?}", bytes_read, addr);
			println!("{:#?}", &self.cache[..bytes_read]);

			let slice = &self.cache[..bytes_read];
			Err(AsyncError::Noncritical(AsyncResumableErrors::NoData))
		}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_recv() {
        let mut v =
            AsyncPacketReceiver::<()>::create("0.0.0.0:6454").expect("Unable to create receiver");
        let packet = v.poll().expect("");
    }
}
