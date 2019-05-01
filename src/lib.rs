#![feature(box_syntax)]

use std::fmt;

pub mod facts;
pub use facts::Facts;

pub mod rule;
pub use rule::Rule;

#[derive(Debug, PartialEq)]
pub enum Query {
    Rule(Rule),
    Given(Facts),
    Find(Facts),
    Dump,
    Delete(Rule),
}

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}
