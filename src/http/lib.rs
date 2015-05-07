/// A StatusCode represents a HTTP response status code.
///
/// The status-code element is a three-digit integer code giving the result of the attempt to
/// understand and satisfy the request.
#[derive(PartialEq, Debug)]
pub enum StatusCode {
	/// 100 - Continue (http://tools.ietf.org/html/rfc7231#section-6.2.1)
	///
	/// The 100 (Continue) status code indicates that the initial part of a request has been
	/// received and has not yet been rejected by the server. The server intends to send a final
	/// response after the request has been fully received and acted upon.
	///
	/// When the request contains an Expect header field that includes a 100-continue expectation,
	/// the 100 response indicates that the server wishes to receive the request payload body, as
	/// described in http://tools.ietf.org/html/rfc7231#section-5.1.1. The client ought to continue
	/// sending the request and discard the 100 response.
	///
	/// If the request did not contain an Expect header field containg the 100-continue
	/// expectation, the client can simply discard this interim response.
	Continue,
	/// 101 - Switching Protocols (http://tools.ietf.org/html/rfc7231#section-6.2.2)
	///
	/// The 101 (Switching Protocols) status code indicates that the server understands and is
	/// willing to comply with the client's request, via, the Upgrade header field
	/// [Section 6.7 of \[RFC7230\]](http://tools.ietf.org/html/rfc7230#section-6.7), for a change in
	/// the application protocol being used on this connection. The server MUST generate an Upgrade
	/// header field in the response that indicates which protocol(s) will be switched to
	/// immediatly after the empty line that terminates the 101 response.
	///
	/// It is assumed that the server will only agree to switch protocols when it is adventageous
	/// to do so. For example, switching to a newer version of HTTP might be adventageous over
	/// older versions, and switching to a real-time, synchronous protocol might be adventageous
	/// when delivering resources that use such features.
	SwitchingProtocols,
	/// 200 - OK (http://tools.ietf.org/html/rfc7231#section-6.3.1)
	///
	/// The 200 (OK) status code indicates that the request has succeeded. The payload sent in a
	/// 200 response depends on the request method. For the methods defined by this specification,
	/// the intended meaning of the payload can be summarized as:
	///
	/// * GET - a representation of the target resource;
	/// * HEAD - the same representation as GET, but without the representation data;
	/// * POST - a representation of the status of, or results obtained from, the action;
	/// * PUT, DELETE - a representation of the status of the action;
	/// * OPTIONS - a representation of the communications options;
	/// * TRACE - a representation of the request message as received by the end server.
	///
	/// Aside from responses to CONNECT, a 200 response always has a payload, though an origin
	/// server MAY generate a payload body of zero length. If no payload is desired, an origin
	/// server ought to send 204 (No Content) instead. For CONNECT, no payload is allowed because
	/// the successful result is a tunnelm, which begins immediatly after the 200 response header
	/// section.
	///
	/// A 200 response is cacheable by default; i.e., unless otherwise indicated by the method
	/// definition or explicit cache controls
	/// (see [Section 3.2.2 of \[RFC7234\]](http://tools.ietf.org/html/rfc7234#section-4.2.2)).
	Ok,
	/// 201 - Created (http://tools.ietf.org/html/rfc7231#section-6.3.2)
	///
	/// The 201 (Created) status code indicates that the request has been fufilled and has resulted
	/// in one or more new resources being created. The primary resource created by the request is
	/// identified by either a Location header field in the response or, if no Location field is
	/// received, by the effective request URI.
	///
	/// The 201 response payload typically describes and links to the resource(s) created.
	/// See [Section 7.2 of \[RFC7231\]](http://tools.ietf.org/html/rfc7231#section-7.2) for a
	/// discussion of the meaning and purpose of validator header fields, such as ETag and
	/// Last-Modified, in a 201 response.
	Created,
	/// 202 - Accepted (http://tools.ietf.org/html/rfc7231#section-6.3.3)
	///
	/// The 202 (Accepted) status code indicates that the request has been accepted for processing,
	/// but the processing has not been completed. The request might or might not eventually be
	/// acted upon, as it might be disallowed when processing actually takes place. There is no
	/// facility in HTTP for re-sending a status code from an asynchronous operation.
	///
	/// The 202 response is intentionally noncommittal. Its purpose is to allow a server to accept
	/// a request for some other process (perhaps a batch-oriented process that is only run once
	/// per day) without requiring that the user agent's connection to the server persist until the
	/// process is completed. The representation sent with this response ought to describe the
	/// request's current status and point to (or embed) a status monitor that can provide the user
	/// with an estimate of when the request will be fufilled.
	Accepted,
	/// 203 - Non-Authoritative Information (http://tools.ietf.org/html/rfc7231#section-6.3.4)
	///
	/// The 203 (Non-Authoritative Information) status code indicates that the request was
	/// successful but the enclosed payload has been modified from that of the origin server's
	/// 200 (OK) response by a transforming proxy
	/// ([Section 5.7.2 of \[RFC7230\]](http://tools.ietf.org/html/rfc7230#section-5.7.2)).
	/// This status code allows the proxy to notify the recipients when a transformation has been
	/// applied, since that knowledge might impact later decisions regarding the content. For
	/// example, future cache validation requests for the conent might only be applicable along the
	/// same request path (through the same proxies).
	///
	/// The 203 response is similiar to the Warning code of 214 Transformation Applied
	/// ([Section 5.5 of \[RFC7234\]](http://tools.ietf.org/html/rfc7234#section-5.5)), which has
	/// the advantage of being appliable to responses with any status code.
	///
	/// A 203 resposne is cacheable by default; i.e., unless otherwise indicated by the method
	/// definition or explicit cache controls (see
	/// [Section 4.2.2 of \[RFC7234\]](http://tools.ietf.org/html/rfc7234#section-4.2.2)).
	NonAuthoritativeInformation,
	/// 204 - No Content (http://tools.ietf.org/html/rfc7231#section-6.3.5)
	///
	/// The 204 (No Content) status code indicates that the server has successfully fufilled the
	/// request and that there is no additional content to send in the response payload body.
	/// Metadata in the response header fields refer to the target resource and its selected
	/// representation after the requested action was applied.
	///
	/// For example, if a 204 status code is received in response to a PUT request and the response
	/// contains the ETag header field, then the PUT was successful and the ETag field-value
	/// contains the entity-tag for the new representation of that target resource.
	///
	/// The 204 response allows a server to indicate that the action has been successfully applied
	/// to the target resource, which implying that the user agent does not need to traverse away
	/// from its current "document view" (if any). The server assumes that the user agent will
	/// provide some indication of the success to its user, in accord with its own interface, and
	/// apply any new or updated metadata in the response to its active representation.
	///
	/// For example, a 204 status code is commonly used with document editing interfaces
	/// corresponding to a "save" action, such that the doucment being saved remains available to
	/// the user for editing. It is also frequently used with interfaces that expect automated data
	/// transfers to be prevalent, such as withing distributed version control systems.
	///
	/// A 204 response is terminated by the first empty line after the header fiels because it
	/// cannot contain a message body.
	///
	/// A 204 response is cacheable by default; i.e., unless otherwise indicated by the method
	/// definition or explicit cache controls (see
	/// [Section 4.2.2 of \[RFC7234\]](http://tools.ietf.org/html/rfc7234#section-4.2.2)).
	NoContent,
	/// 205 - Reset Content (http://tools.ietf.org/html/rfc7231#section-6.3.6)
	///
	/// The 205 (Reset Content) status code indicates that the server has fufilled the request and
	/// desires that the user agent reset the "document view", which caused the request to be sent,
	/// to its original state as received from the origin server.
	///
	/// This response is intended to support a common data entry use case where the user receives
	/// content that supports data entry (a form, notepad, canvas, etc.), enters or manipulates
	/// data in that space, causes the entered data to be submitted in a request, and then the data
	/// entry mechanism is reset for the next entry so that the user can easily initiate another
	/// input aciton.
	///
	/// Since the 205 status code implies that no addtional content will be provided, a server MUST
	/// NOT generate a payload in a 205 response. In other wors, a server MUST do one of the
	/// following for a 205 response: a) indicate a zero-length body for the response by including
	/// a Content-Length header field with a value of 0; b) indicate a zero-length payload for the
	/// response by including a Transfer-Encoding header field with a value of chunked and a message
	/// body consiting of a single chunk of zero-length; or, c) close the connection immediatly
	/// after sending the blank line terminating the header section.
	ResetContent,
	/// 206 - Partial Content (http://tools.ietf.org/html/rfc7233#section-4.1)
	PartialContent,
	/// 300 - Multiple Choices (http://tools.ietf.org/html/rfc7231#section-6.4.1)
	///
	/// The 300 (Multiple Choices) status code indicates that the target resource has more than one
	/// representation, each with its own more specific identifier, and information about
	/// alternatives is being provided so that the user (or user agent) can select a preferred
	/// representation by redirecting its request to one or more of those identifiers. In other
	/// words, the server desires that the user agent engage in reactive negotiation to select the
	/// most appropriate representation(s) for its needs
	/// ([Section 3.4 of \[RFC7231\]](http://tools.ietf.org/html/rfc7231#section-3.4)).
	///
	/// If the server has a preferred choice, the server SHOULD generate a Location header field
	/// containing a preferred choice's URI reference. The user agent MAY use the Location field
	/// value for automatic redirection.
	///
	/// For request methods other than HEAD, the server SHOULD generate a payload in the 300
	/// response containing a list of representation metadata and URI reference(s) from which the
	/// user or user agent can choose the one most preferred. The user agent MAY make a selection
	/// from that list automatically if it understands the provided media type. A specific format
	/// for automatic selection is not defined by this specification because HTTP tries to remain
	/// orthogonal to the definition of its payloads. In practice, the representation is provided
	/// in some easily parsed format believed to be acceptable to the user agent, as determined by
	/// shared design or content negotiation, or in some commonly accepted hypertext format.
	///
	/// A 300 response is cacheable by default; i.e., unless otherwise indicated by the method
	/// definition or explicit cache controls
	/// (see [Section 4..2 of \[RFC7234\]](http://tools.ietf.org/html/rfc7234#section-4.2.2)).
	///
	/// > Note: The original proposal for the 300 status code defined the URI header field as
	/// > providing a list of alternative representations, such that it would be usable for 200,
	/// > 300, and 406 responses and be transferred in response to the HEAD method. However, a
	/// > lack of deployment and disagreement over syntax led to both URI and Alternates (a
	/// > subsequent proposal) being dropped from this specification. It is possible to
	/// > communicate the list using a set of Link header fields
	/// > [ \[RFC5988\] ](http://tools.ietf.org/html/rfc5988), each with a relationship of
	/// > "alternate", though deployment is a chicken-and-egg problem.
	MultipleChoices,
	/// 301 - Moved Permanently (http://tools.ietf.org/html/rfc7231#section-6.4.2)
	///
	/// The 301 (Moved Permanently) status code indicates that the target resource has been
	/// assigned a new permanent URI and any future references to this resource ought to use one of
	/// the enclosed URIs. Clients with link-editing capabilities ought to automatically re-link
	/// references to the effective request URI to one or more of the new references sent by the
	/// server, where possible.
	///
	/// The server SHOULD generate a Location header field in the response containing a preferred
	/// URI reference for the new permanent URI. The user agent MAY use the location field value
	/// for automatic redirection. The server's response payload usually contains a short hypertext
	/// note with a hyperlink to the new URI(s).
	///
	/// > Note: For historical reasons, a user agent MAY change the request method from POST to GET
	/// > for the subsequent request. If this behavior is undesired the 307 (Temporary Redirect)
	/// > status code can be used instead.
	///
	/// A 301 response is cacheable by default; i.e., unless otherwise indicated by the method
	/// definition or explicit cache controls (see
	/// [Section 4.2.2 of \[RFC7234\]](http://tools.ietf.org/html/rfc7234#section-4.2.2)).
	MovedPermanently,
	/// 302 - Found (http://tools.ietf.org/html/rfc7231#section-6.4.3)
	///
	/// The 302 (Found) status code indicates that the target resource resides temporarily under a
	/// different URI. Since the redirection might be altered on occasion, the client ought to
	/// continue to use the effective request URI for future requests.
	///
	/// The server SHOULD generate a Location header field in the response containing a URI
	/// reference for the different URI. The user agent MAY use the Location field value for
	/// automatic redirection. The server's response payload usually contains a short hypertext
	/// note with a hyperlink to the different URI(s).
	///
	/// > Note: For historical reasions, a user agent MAY change the request method from POST to
	/// > GET for the subsequent request. If this behavior is undesired, the 307 (Temporary
	/// > Redirect) status code can be used instead.
	Found,
	/// 303 - See Other (http://tools.ietf.org/html/rfc7231#section-6.4.4)
	SeeOther,
	/// 304 - Not Modified (http://tools.ietf.org/html/rfc7232#section-4.1)
	NotModified,
	/// 305 - Use Proxy (http://tools.ietf.org/html/rfc7231#section-6.4.5)
	UseProxy,
	/// 307 - Temporary Redirect (http://tools.ietf.org/html/rfc7231#section-6.4.7)
	TemporaryRedirect,
	/// 400 - Bad Request (http://tools.ietf.org/html/rfc7231#section-6.5.1)
	BadRequest,
	/// 401 - Unauthorized (http://tools.ietf.org/html/rfc7235#section-3.1)
	Unauthorized,
	/// 402 - Payment Required (http://tools.ietf.org/html/rfc7231#section-6.5.2)
	PaymentRequired,
	/// 403 - Forbidden (http://tools.ietf.org/html/rfc7231#section-6.5.3)
	Forbidden,
	/// 404 - Not Found (http://tools.ietf.org/html/rfc7231#section-6.5.4)
	NotFound,
	/// 405 - Method Not Allowed (http://tools.ietf.org/html/rfc7231#section-6.5.5)
	MethodNotAllowed,
	/// 406 - Not Acceptable (http://tools.ietf.org/html/rfc7231#section-6.5.6)
	NotAcceptable,
	/// 407 - Proxy Authentication Required (http://tools.ietf.org/html/rfc7235#section-3.2)
	ProxyAuthenticationRequired,
	/// 408 - Request Timeout (http://tools.ietf.org/html/rfc7231#section-6.5.7)
	RequestTimeout,
	/// 409 - Conflict (http://tools.ietf.org/html/rfc7231#section-6.5.8)
	Conflict,
	/// 410 - Gone (http://tools.ietf.org/html/rfc7231#section-6.5.9)
	Gone,
	/// 411 - Length Required (http://tools.ietf.org/html/rfc7231#section-6.5.10)
	LengthRequired,
	/// 412 - Precondition Failed (http://tools.ietf.org/html/rfc7232#section-4.2)
	PreconditionFailed,
	/// 413 - Payload Too Large (http://tools.ietf.org/html/rfc7231#section-6.5.11)
	PayloadTooLarge,
	/// 414 - URI Too Long (http://tools.ietf.org/html/rfc7231#section-6.5.12)
	UriTooLong,
	/// 415 - Unsupported Media Type (http://tools.ietf.org/html/rfc7231#section-6.5.13)
	UnsupportedMediaType,
	/// 416 - Range Not Satisfiable (http://tools.ietf.org/html/rfc7233#section-4.4)
	RangeNotSatisfiable,
	/// 417 - Expectation Failed (http://tools.ietf.org/html/rfc7231#section-6.5.14)
	ExpectationFailed,
	/// 426 - Upgrade Required (http://tools.ietf.org/html/rfc7231#section-6.5.15)
	UpgradeRequired,
	/// 500 - Internal Server Error (http://tools.ietf.org/html/rfc7231#section-6.6.1)
	InternalServerError,
	/// 501 - Not Implemented (http://tools.ietf.org/html/rfc7231#section-6.6.2)
	NotImplemented,
	/// 502 - Bad Gateway (http://tools.ietf.org/html/rfc7231#section-6.6.3)
	BadGateway,
	/// 503 - Service Unavailable (http://tools.ietf.org/html/rfc7231#section-6.6.4)
	ServiceUnavailable,
	/// 504 - Gateway Timeout (http://tools.ietf.org/html/rfc7231#section-6.6.5)
	GatewayTimeout,
	/// 505 - HTTP Version Not Supported (http://tools.ietf.org/html/rfc7231#section-6.6.6)
	HttpVersionNotSupported,
	/// HTTP status codes are extensible. HTTP clients are not required to understand the meaning of
	/// all registered status codes, though such understanding is obviously desirable. However, a
	/// client MUST understand the class of any status code, as indicated by the first digit, and treat
	/// an unrecognized status code as being equivalent to the x00 status code of that class, with the
	/// exception that a recipient MUST NOT cache a response with an unrecognized status code.
	///
	/// For example, if an unrecognized status code of 471 is received by a client, the client can
	/// assume that there was something wrong with its request and treat the response as if it had
	/// received a 400 (Bad Request) status code. The response message will usually contain a
	/// representation that explains the status.
	Extension(u16, String)
}

#[derive(PartialEq, Debug)]
pub enum StatusCodeClass {
	/// 1xx - Informational (http://tools.ietf.org/html/rfc7231#section-6.2)
	///
	/// The 1xx (Informational) class of status code indicates than an interim response for
	/// communicating connection status or request progress prior to completing the requested
	/// action and sending a final response. 1xx respones are terminated by the first empty line
	/// after the status-line (the empty line signaling the end of the header section). Since
	/// HTTP/1.0 did not define any 1xx status codes, a server MUST NOT send a 1xx response to an
	/// HTTP/1.0 client.
	///
	/// A client MUST be able to parse one or more 1xx responses received prior to a final
	/// response, even if the client does not expect one. A user agent MAY ignore unexpected 1xx
	/// responses.
	///
	/// A proxy MUST forward 1xx responses unless the proxy itself requested the generation of the
	/// 1xx response. For example, if a proxy adds an "Expect: 100-continue" field when it forwards
	/// a request, then it need not forward the corresponding 100 (Continue) response(s).
	Informational,
	Successful,
	Redirection,
	ClientError,
	ServerError,
}

impl StatusCode {
	pub fn from_u16(n: u16) -> StatusCode {
		use StatusCode::*;

		match n {
			100 => Continue,
			101 => SwitchingProtocols,
			200 => Ok,
			201 => Created,
			202 => Accepted,
			203 => NonAuthoritativeInformation,
			204 => NoContent,
			205 => ResetContent,
			206 => PartialContent,
			300 => MultipleChoices,
			301 => MovedPermanently,
			302 => Found,
			303 => SeeOther,
			304 => NotModified,
			305 => UseProxy,
			307 => TemporaryRedirect,
			400 => BadRequest,
			401 => Unauthorized,
			402 => PaymentRequired,
			403 => Forbidden,
			404 => NotFound,
			405 => MethodNotAllowed,
			406 => NotAcceptable,
			407 => ProxyAuthenticationRequired,
			408 => RequestTimeout,
			409 => Conflict,
			410 => Gone,
			411 => LengthRequired,
			412 => PreconditionFailed,
			413 => PayloadTooLarge,
			414 => UriTooLong,
			415 => UnsupportedMediaType,
			416 => RangeNotSatisfiable,
			417 => ExpectationFailed,
			426 => UpgradeRequired,
			500 => InternalServerError,
			501 => NotImplemented,
			502 => BadGateway,
			503 => ServiceUnavailable,
			504 => GatewayTimeout,
			505 => HttpVersionNotSupported,
			_ => Extension(n, "Unknown extension code".to_string())
		}
	}

	pub fn to_u16(self) -> u16 {
		use StatusCode::*;

		match self {
			Continue => 100,
			SwitchingProtocols => 101,
			Ok => 200,
			Created => 201,
			Accepted => 202,
			NonAuthoritativeInformation => 203,
			NoContent => 204,
			ResetContent => 205,
			PartialContent => 206,
			MultipleChoices => 300,
			MovedPermanently => 301,
			Found => 302,
			SeeOther => 303,
			NotModified => 304,
			UseProxy => 305,
			TemporaryRedirect => 307,
			BadRequest => 400,
			Unauthorized => 401,
			PaymentRequired => 402,
			Forbidden => 403,
			NotFound => 404,
			MethodNotAllowed => 405,
			NotAcceptable => 406,
			ProxyAuthenticationRequired => 407,
			RequestTimeout => 408,
			Conflict => 409,
			Gone => 410,
			LengthRequired => 411,
			PreconditionFailed => 412,
			PayloadTooLarge => 413,
			UriTooLong => 414,
			UnsupportedMediaType => 415,
			RangeNotSatisfiable => 416,
			ExpectationFailed => 417,
			UpgradeRequired => 426,
			InternalServerError => 500,
			NotImplemented => 501,
			BadGateway => 502,
			ServiceUnavailable => 503,
			GatewayTimeout => 504,
			HttpVersionNotSupported => 505,
			Extension(code, _) => code,
		}
	}

	pub fn class(self) -> Option<StatusCodeClass> {
		match self.to_u16() {
			100 ... 199 => Some(StatusCodeClass::Informational),
			200 ... 299 => Some(StatusCodeClass::Successful),
			300 ... 399 => Some(StatusCodeClass::Redirection),
			400 ... 499 => Some(StatusCodeClass::ClientError),
			500 ... 599 => Some(StatusCodeClass::ServerError),
			_ => None,
		}
	}
}

#[cfg(test)]
mod test {

	#[test]
	fn statuscode_from_u16_test() {
		use super::StatusCode;

		let test_cases = vec![
			(100, StatusCode::Continue),
			(101, StatusCode::SwitchingProtocols),
			(200, StatusCode::Ok),
			(201, StatusCode::Created),
			(202, StatusCode::Accepted),
		];

		for (n, expect) in test_cases {
			let actual = StatusCode::from_u16(n);
			assert_eq!(expect, actual);
		}
	}

	#[test]
	fn statuscode_from_u16_is_inverse_of_to_u16() {
		use super::StatusCode;

		for expect in 100..600 {
			let actual = StatusCode::from_u16(expect).to_u16();
			assert_eq!(expect, actual);
		}
	}

	#[test]
	fn statuscode_class_test() {
		use super::{StatusCode, StatusCodeClass};

		for n in 100..200 {
			let expect = Some(StatusCodeClass::Informational);
			let actual = StatusCode::from_u16(n).class();
			assert_eq!(expect, actual);
		}

		for n in 200..300 {
			let expect = Some(StatusCodeClass::Successful);
			let actual = StatusCode::from_u16(n).class();
			assert_eq!(expect, actual);
		}

		for n in 300..400 {
			let expect = Some(StatusCodeClass::Redirection);
			let actual = StatusCode::from_u16(n).class();
			assert_eq!(expect, actual);
		}

		for n in 400..500 {
			let expect = Some(StatusCodeClass::ClientError);
			let actual = StatusCode::from_u16(n).class();
			assert_eq!(expect, actual);
		}

		for n in 500..600 {
			let expect = Some(StatusCodeClass::ServerError);
			let actual = StatusCode::from_u16(n).class();
			assert_eq!(expect, actual);
		}

		for n in vec![0, 10, 600] {
			let expect = None;
			let actual = StatusCode::from_u16(n).class();
			assert_eq!(expect, actual);
		}
	}
}
