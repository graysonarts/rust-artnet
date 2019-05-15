use crate::r#async::FromRaw;

const ARTNET_SIGNATURE: &str = "Art-Net";

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
			_ => Unknown,
		}
	}
}

pub struct Packet {}

fn validate_signature(data: &[u8]) -> Option<usize> {
		let signature_expected_length = ARTNET_SIGNATURE.len() + 1;
    let signature = &data[..signature_expected_length];
    let is_valid = signature
        .iter()
        .zip(ARTNET_SIGNATURE.as_bytes().iter())
        .map(|(l, r)| l == r)
        .fold(true, |acc, x| acc && x)
        && data[signature_expected_length] == 0;

    if is_valid {
        Some(signature_expected_length + 1)
    } else {
        None
    }
}

fn get_opcode(data: &[u8]) -> Option<Opcode> {
	Some(Opcode::from(u16::from(data[0]) | u16::from(data[1]) << 8))
}

impl FromRaw<Packet> for Packet {
    fn from_raw(data: &[u8]) -> Option<Packet> {
        let position = validate_signature(data)?;
				let remaining = &data[position..];

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PACKET: [u8; 14] = [65, 114, 116, 45, 78, 101, 116, 0, 0, 32, 0, 14, 0, 0];

    #[test]
    fn test_valid_signature() {
        assert_eq!(validate_signature(&PACKET), Some(9));
    }

		#[test]
		fn test_op_code_detection() {
			let remaining = &PACKET[8..];
			let op_code = get_opcode(remaining).expect("Unable to get opcode");
			assert_eq!(op_code, Opcode::OpPoll);
		}
}
