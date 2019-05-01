#![feature(box_syntax)]

use std::fmt;

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

pub struct Facts {
    yes: [bool; 26],
    no: [bool; 26],
}

impl Facts {
    pub fn new(chars: &str) -> Facts {
        let mut yes = [false; 26];
        let mut no = [false; 26];

        for c in chars.chars() {
            yes[(c as usize) - ('A' as usize)] = true;
        }

        Facts { yes, no }
    }
}

impl fmt::Debug for Facts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Facts {{ true: ")?;

        for (idx, &val) in self.yes.iter().enumerate() {
            if val {
                write!(f, "{}", (idx + 'A' as usize) as u8 as char)?;
            } else {
                write!(f, ".")?;
            }
        }

        write!(f, " false: ")?;

        for (idx, &val) in self.no.iter().enumerate() {
            if val {
                write!(f, "{}", (idx + 'A' as usize) as u8 as char)?;
            } else {
                write!(f, ".")?;
            }
        }

        write!(f, " }}")?;

        Ok(())
    }
}
