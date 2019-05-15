use crate::r#async::FromRaw;

const ARTNET_SIGNATURE: &str = "Art-Net";
const OPCODE_LOCATION: usize = 8;
const VERSION_LOCATION: usize = 11;

#[repr(u16)]
#[derive(Debug, PartialEq)]
enum Opcode {
    OpPoll = 0x2000,
    OpPollReply = 0x2100,

    Unknown = 0xFFFF,
}

impl From<u16> for Opcode {
	fn from(i: u16) -> Opcode {
		use Opcode::*;
		match i {
			0x2000 => OpPoll,
			0x2100 => OpPollReply,
			_ => { println!("{:x?}", i); Unknown },
		}
	}
}

pub struct Packet {
	opcode: Opcode,
}

fn validate_signature(data: &[u8]) -> Option<()> {
		let signature_expected_length = ARTNET_SIGNATURE.len() + 1;
    let signature = &data[..signature_expected_length];
    let is_valid = signature
        .iter()
        .zip(ARTNET_SIGNATURE.as_bytes().iter())
        .map(|(l, r)| l == r)
        .fold(true, |acc, x| acc && x)
        && data[signature_expected_length] == 0;

    if is_valid {
        Some(())
    } else {
        None
    }
}

fn validate_version(data: &[u8]) -> Option<()> {
	let offset_data = &data[VERSION_LOCATION..];
	let version = read_little_endian(offset_data);
	println!("version = {:x?}/{:?}", version, version);
	if version == 14 {
		Some(())
	} else {
		None
	}
}

fn read_little_endian(data: &[u8]) -> u16 {
	use std::io::Cursor;
	use byteorder::{LittleEndian, ReadBytesExt};
	let mut rdr = Cursor::new(data);

	rdr.read_u16::<LittleEndian>().unwrap()
}

impl FromRaw<Packet> for Packet {
    fn from_raw(data: &[u8]) -> Option<Packet> {
        validate_signature(data)?;
				validate_version(data)?;
				let remaining = &data[OPCODE_LOCATION..];
				let opcode = Opcode::from(read_little_endian(remaining));

        Some(Packet {
					opcode
				})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PACKET: [u8; 14] = [65, 114, 116, 45, 78, 101, 116, 0, 0, 32, 0, 14, 0, 0];

    #[test]
    fn test_valid_signature() {
        assert_eq!(validate_signature(&PACKET), Some(()));
    }

		#[test]
		fn test_valid_version() {
			assert_eq!(validate_version(&PACKET), Some(()));
		}

		#[test]
		fn test_op_code_detection() {
			let remaining = &PACKET[8..];
			let op_code = read_little_endian(remaining);
			assert_eq!(op_code, Opcode::OpPoll as u16);
		}

		#[test]
		fn test_packet_generation() {
			let packet = Packet::from_raw(&PACKET).expect("Unable to create packet");
			assert_eq!(packet.opcode, Opcode::OpPoll);
		}
}
