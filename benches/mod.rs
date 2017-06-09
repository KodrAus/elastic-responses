#![feature(test)]
#![feature(alloc_system)]

extern crate alloc_system;
extern crate test;
extern crate elastic_responses;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate inlinable_string;
extern crate string_cache;

pub mod bulk;