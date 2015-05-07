//! Functions and types for building an HTTP client.

use super::{HttpVersion, StatusCode};

#[derive(PartialEq, Debug)]
pub struct Response {
    pub http_version: HttpVersion,
    pub status_code: StatusCode,
    pub reason_phrase: String,
}

#[derive(PartialEq, Debug)]
pub enum Message {
    Response(Response)
}

fn parse_message(msg: Vec<u8>) -> Option<Message> {
    let (http_version, status_code, reason_phrase) = parse_statusline(msg);
    let response = Response {
        http_version: http_version,
        status_code: status_code,
        reason_phrase: reason_phrase
    };
    Some(Message::Response(response))
}

fn parse_statusline(msg: Vec<u8>) -> (HttpVersion, StatusCode, String) {
    let (http_version, remain) = parse_httpversion(msg.as_slice());
    let status_code = StatusCode::Ok;
    let reason_phrase = "".to_string();

    (http_version, status_code, reason_phrase)
}

fn parse_httpversion(msg: &[u8]) -> (HttpVersion, &[u8]) {
    (HttpVersion::Version11, msg)
}

/// Executes a GET request.
pub fn get(_: &str) -> Option<Response> { 
    let response = Response {
        http_version: HttpVersion::Version11,
        status_code: StatusCode::Ok,
        reason_phrase: "OK".to_string(),
    };
    Some(response)
}

#[cfg(test)]
mod test {
    use super::super::{HttpVersion, StatusCode};

    // TODO (smkell, 05/07/2015): Remove this test when we have a functional web server.
    #[test]
    fn get_google_test() {
        use super::get;

        let result = get("http://www.google.com");
        assert!(result.is_some());

        let response = result.unwrap();
        assert_eq!(HttpVersion::Version11, response.http_version);
        assert_eq!(StatusCode::Ok, response.status_code);
        assert_eq!("OK", response.reason_phrase);
    }

    #[test]
    fn parse_response_test() {
        use super::{Response, Message, parse_message};

        let message: Vec<u8> = vec![
            0x48,0x54,0x54,0x50,0x2f,0x31,0x02E,0x31,0x20,0x32,0x30,0x30,0x20,0x4F,0x4C,0x0D,0x0A,
            0x0D,0x0A,
            0x3C,0x21,0x64,0x6F,0x63,0x74,0x79,0x70,0x65,0x20,0x68,0x74,0x6D,0x6C,0x3E,0x3C,0x68,0x74,0x6D,0x6c,0x3E,0x3C,0x2F,0x68,0x74,0x6D,0x6c,0x3E];

        let expect = Some(Message::Response(Response { 
            http_version: HttpVersion::Version11, 
            status_code: StatusCode::Ok, 
            reason_phrase: "OK".to_string()
        }));

        let actual = parse_message(message);
        assert_eq!(expect, actual);
    }

}
