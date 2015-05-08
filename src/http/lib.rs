//! An implemention of the HTTP protocol for both clients and servers.

#![feature(convert)]

mod statuscodes;
pub mod client;

pub use statuscodes::*;

#[derive(PartialEq, Debug)]
pub enum HttpVersion {
    Version11,
}
