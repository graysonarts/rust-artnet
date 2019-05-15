mod r#async;
mod packet;

use packet::Packet;

pub struct Controller {}

pub struct Node {}

impl Responder for Node {
    fn on_art_poll(&self, _packet: Packet) -> Result<(), ()> {
        Err(())
    }
}

trait Responder {
    fn on_art_poll(&self, _packet: Packet) -> Result<(), ()> {
        Ok(())
    }

    fn on_art_poll_reply(&self, _packet: Packet) -> Result<(), ()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
