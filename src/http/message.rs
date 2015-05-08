//! Functions and types for interacting with HTTP messages.

use super::{HttpVersion, StatusCode};

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
        (Vec::from_iter("HTTP/1.0 ".chars()), (HttpVersion::Version10, 1)),
	];

	for (msg, (expect_msg, expect_len)) in test_cases {
		let result = parse_httpversion(msg.as_slice());
		assert!(result.is_some());
		let (actual_msg, actual_remain) = result.unwrap();

		assert_eq!(expect_msg, actual_msg);
		assert_eq!(expect_len, actual_remain.len());
	}
}

/// Parses a StatusCode from a slice of bytes.
/// 
/// # Return
/// If parsing was successful the return value is a tuple of the parsed StatusCode, and a slice
/// containing any remaining bytes from the message.
fn parse_statuscode(msg: &[char]) -> Option<(StatusCode, &[char])> {
    use std::iter::FromIterator;

    let status_code = StatusCode::Ok;
    let remain: &[char] = &[];
    let result = (status_code, remain);

    let mut is_digit = true;
    let mut end = 1;
    while is_digit && end < msg.len() {
        if msg[end].is_digit(10) {
            end = end + 1;
        } else {
            is_digit = false;
        }
        println!("is_digit: {:?} end: {:?} msg[0..end]: {:?}", is_digit, end, &msg[0..end]);
    }
    
    // convert the digits into a String
    let digits: &[char] = &msg[0..end];
    let mut accum: u16 = 0;
    for x in 0..end {
        let digit: u16 = (msg[x].to_digit(10).unwrap()) as u16;
        let exp = (end - 1 - x) as u32;
        accum = accum + (digit * 10u16.pow(exp));
        println!("digit: {:?} exp: {:?} digit^exp: {:?} accum: {:?}", digit, exp, (digit.pow(exp)), accum);
    }
    Some (result)
}

#[test]
fn parse_statuscode_test() {
    use super::StatusCode;
    use std::iter::FromIterator;

    let test_cases = vec![
        (Vec::from_iter("200".chars()), (StatusCode::Ok, 0)),
        (Vec::from_iter("300".chars()), (StatusCode::MultipleChoices, 0)),
    ];

    for (msg, (expect_msg, expect_len)) in test_cases {
        let result = parse_statuscode(msg.as_slice());
        assert!(result.is_some());
        let (actual_msg, actual_remain) = result.unwrap();

        assert_eq!(expect_msg, actual_msg);
        assert_eq!(expect_len, actual_remain.len());
    }
}
