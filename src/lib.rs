mod r#async;

#[repr(u16)]
enum Opcode {
    OpPoll = 0x2000,
    OpPollReply = 0x2100,

    Unknown = 0xFFFF,
}

pub struct Packet {}

pub struct Controller {}

pub struct Node {}

impl Responder for Node {
    fn on_art_poll(&self, packet: Packet) -> Result<(), ()> {
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
