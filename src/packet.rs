use crate::r#async::FromRaw;

const ARTNET_SIGNATURE: &str = "Art-Net";

#[repr(u16)]
enum Opcode {
    OpPoll = 0x2000,
    OpPollReply = 0x2100,

    Unknown = 0xFFFF,
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

impl FromRaw<Packet> for Packet {
    fn from_raw(data: &[u8]) -> Option<Packet> {
        validate_signature(data)?;

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
}
