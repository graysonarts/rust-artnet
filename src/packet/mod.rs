use crate::r#async::FromRaw;

pub(crate) mod header;
pub(crate) mod op_poll;

#[derive(Debug)]
pub struct Packet {
	header: header::Header,
}

impl FromRaw<Packet> for Packet {
	fn from_raw(raw: &[u8]) -> Option<Packet> {
		use header::Opcode;

		let hdr = header::Header::from_raw(raw)?;
		let payload = match hdr.opcode {
			Opcode::OpPoll => op_poll::OpPoll::from_raw(raw),
			Opcode::OpPollReply => None,

			_ => None,
		};

		Some(Packet{
			header: hdr,
		})
	}
}

fn read_little_endian(data: &[u8]) -> u16 {
	use byteorder::{LittleEndian, ReadBytesExt};
	use std::io::Cursor;
	let mut rdr = Cursor::new(data);

	rdr.read_u16::<LittleEndian>().unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const PACKET: [u8; 14] = [65, 114, 116, 45, 78, 101, 116, 0, 0, 32, 0, 14, 0, 0];

	#[test]
	fn test_op_code_detection() {
		let remaining = &PACKET[8..];
		let op_code = read_little_endian(remaining);
		assert_eq!(op_code, 0x2000);
	}

	#[test]
	fn test_from_raw() {
		let packet = Packet::from_raw(&PACKET).expect("Unable to parse packet");
	}
}
