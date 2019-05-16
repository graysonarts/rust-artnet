use crate::r#async::FromRaw;
use bitfield::bitfield;

const VERSION_LOCATION: usize = 10;
const TALK_LOCATION: usize = 12;
const PRIORITY_LOCATION: usize = 13;

fn validate_version(data: &[u8]) -> Option<()> {
	let offset_data = &data[VERSION_LOCATION..];
	if offset_data[0] == 0 && offset_data[1] == 14 {
		Some(())
	} else {
		None
	}
}

bitfield! {
	struct TalkToMe(u8);
	impl Debug;
	reserved, _: 7, 5;
	enable_vlc, set_enable_vlc: 4;
	broadcast_diag, set_broadcast_diag: 3;
	send_diag, set_send_diag: 2;
	send_reply_on_change, set_send_reply_on_change: 1;
	deprecated, _: 0;
}

pub struct OpPoll {
	talk_to_me: TalkToMe,
	priority: u8,
}

impl FromRaw<OpPoll> for OpPoll {
	fn from_raw(raw: &[u8]) -> Option<Self> {
		validate_version(raw)?;
		let talk_to_me = TalkToMe(raw[TALK_LOCATION]);
		let priority = raw[PRIORITY_LOCATION];

		Some(OpPoll { talk_to_me, priority })
	}
}

pub struct OpPollReply {}

#[cfg(test)]
mod tests {
	use super::*;

	const PACKET: [u8; 14] = [65, 114, 116, 45, 78, 101, 116, 0, 0, 32, 0, 14, 0b0000_1010, 0];

	#[test]
	fn test_valid_version() {
		assert_eq!(validate_version(&PACKET), Some(()));
	}

	#[test]
	fn test_from_raw() {
		let packet = OpPoll::from_raw(&PACKET).expect("Unable to parse packet");
		assert!(packet.talk_to_me.broadcast_diag());
		assert!(packet.talk_to_me.send_reply_on_change());
		assert!(!packet.talk_to_me.send_diag());
	}
}
