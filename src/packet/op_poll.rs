use crate::r#async::FromRaw;
use super::read_little_endian;

const VERSION_LOCATION: usize = 11;

fn validate_version(data: &[u8]) -> Option<()> {
	let offset_data = &data[VERSION_LOCATION..];
	let version = read_little_endian(offset_data);
	if version == 14 {
		Some(())
	} else {
		None
	}
}

pub struct OpPoll {

}

impl FromRaw<OpPoll> for OpPoll {
	fn from_raw(raw: &[u8]) -> Option<Self> {
		validate_version(raw)?;

		Some(OpPoll {

		})
	}
}

pub struct OpPollReply {

}

#[cfg(test)]
mod tests {
	use super::*;

	const PACKET: [u8; 14] = [65, 114, 116, 45, 78, 101, 116, 0, 0, 32, 0, 14, 0, 0];

	#[test]
	fn test_valid_version() {
		assert_eq!(validate_version(&PACKET), Some(()));
	}

	#[test]
	fn test_from_raw() {
		let packet = OpPoll::from_raw(&PACKET).expect("Unable to parse packet");
		// TODO: Add asserts
	}
}