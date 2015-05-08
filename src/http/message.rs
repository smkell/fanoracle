//! Functions and types for interacting with HTTP messages.

use super::HttpVersion;

fn parse_httpname(msg: &[char]) -> Option<&[char]> {
	match msg {
		['H', 'T', 'T', 'P', rest..] => {
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
fn parse_httpversion(msg: &[char]) -> Option<(HttpVersion, &[char])> {

	// Parse out the http name.
	match parse_httpname(msg) {
		Some(remain) => match remain {
			['/', major, '.', minor, rest..] => {
				match (major, minor) {
					('1', '0') => Some((HttpVersion::Version10, rest)),
					('1', '1') => Some((HttpVersion::Version11, rest)),
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
    use std::iter::FromIterator;

	let test_cases = vec![
        (Vec::from_iter("HTTP/1.1".chars()), (HttpVersion::Version11, 0)),
        (Vec::from_iter("HTTP/1.0".chars()), (HttpVersion::Version10, 0)),
	];

	for (msg, (expect_msg, expect_len)) in test_cases {
		let result = parse_httpversion(msg.as_slice());
		assert!(result.is_some());
		let (actual_msg, actual_remain) = result.unwrap();

		assert_eq!(expect_msg, actual_msg);
		assert_eq!(expect_len, actual_remain.len());
	}
}
