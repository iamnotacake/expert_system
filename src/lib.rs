#![feature(box_syntax)]

use std::fmt;

pub mod facts;
pub use facts::Facts;

pub mod fact;
pub use fact::Fact;

#[derive(Debug)]
pub enum Query {
    Fact(Fact),
    Given(Facts),
    Find(Facts),
}

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}
