//! An implemention of the HTTP protocol for both clients and servers.

#![feature(convert)]
#![feature(slice_patterns)]

mod statuscodes;
mod message;
pub mod client;

pub use statuscodes::*;

#[derive(PartialEq, Debug)]
pub enum HttpVersion {
    Version10,
    Version11,
}
