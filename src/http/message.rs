//! Functions and types for interacting with HTTP messages.

use super::HttpVersion;

fn parse_httpname(msg: &[u8]) -> Option<&[u8]> {
	match msg {
		[0x48, 0x54, 0x54, 0x50, rest..] => {
			println!("Found the HTTP name! Rest: {:?}", rest);
			Some(rest)
		},
		_ => None
	}
}

/// Parses a HttpVersion from a slice of bytes.
///
/// # Return
/// If parsing was successful the return value is a tuple of the parsed HttpVersion, and a slice
/// containing any remaining bytes from the message.
fn parse_httpversion(msg: &[u8]) -> Option<(HttpVersion, &[u8])> {

	// Parse out the http name.
	match parse_httpname(msg) {
		Some(remain) => match remain {
			[0x2f, major, 0x2E, minor, rest..] => {
				println!("Matched /major.minor! major: {:?} minor: {:?} rest: {:?}", major, minor, rest);
				match (major, minor) {
					(0x31, 0x30) => Some((HttpVersion::Version10, rest)),
					(0x31, 0x31) => Some((HttpVersion::Version11, rest)),
					_ => None
				}
			},
			_ => None,
		},
		None => None
	}
}

#[test]
fn parse_httpversion_test() {
	use super::HttpVersion;

	let test_cases = vec![
		(vec![0x48,0x54,0x54,0x50,0x2F,0x31,0x2E,0x31], (HttpVersion::Version11, 0)),
		(vec![0x48,0x54,0x54,0x50,0x2F,0x31,0x2E,0x30], (HttpVersion::Version10, 0)),
	];

	for (msg, (expect_msg, expect_len)) in test_cases {
		let result = parse_httpversion(msg.as_slice());
		assert!(result.is_some());
		let (actual_msg, actual_remain) = result.unwrap();

		assert_eq!(expect_msg, actual_msg);
		assert_eq!(expect_len, actual_remain.len());
	}
}
