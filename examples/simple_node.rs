use artnet::prelude::*;

fn poll(recv: &mut PacketReceiver) -> Option<Packet> {
	recv.poll().ok()
}

fn main() {
	let node = Node {};
	let mut receiver = PacketReceiver::create("0.0.0.0:6454").expect("Unable to create packet receiver");
	poll(&mut receiver)
		.and_then(|p| {
			println!("{:#?}", p);
			Some(())
		})
		.or_else(|| {
			println!("Unable to receive packet");
			Some(())
		});
}
