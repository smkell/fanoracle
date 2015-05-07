//! An implemention of the HTTP protocol for both clients and servers.

mod statuscodes;
pub mod client;

pub use statuscodes::*;

#[derive(PartialEq, Debug)]
pub enum HttpVersion {
    Version11,
}
