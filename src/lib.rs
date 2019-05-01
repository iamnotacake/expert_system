#![feature(box_syntax)]

use std::fmt;

pub mod facts;
pub use facts::Facts;

pub mod parser {
    include!(concat!(env!("OUT_DIR"), "/parser.rs"));
}

#[derive(Debug)]
pub enum Query {
    Fact(Fact),
    Given(Facts),
    Find(Facts),
}

#[derive(Debug)]
pub enum Fact {
    Char(char),
    Not(Box<Fact>),
    And(Box<Fact>, Box<Fact>),
    Or(Box<Fact>, Box<Fact>),
    Xor(Box<Fact>, Box<Fact>),
    IfThen(Box<Fact>, Box<Fact>),
    IfAndOnlyIf(Box<Fact>, Box<Fact>),
}
