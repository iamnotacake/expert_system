use std::fmt;

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

impl fmt::Display for Fact {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Fact::*;

        match self {
            Char(ref c) => write!(f, "{}", c),
            Not(ref l) => write!(f, "!({})", l),
            And(ref l, ref r) => write!(f, "({} + {})", l, r),
            Or(ref l, ref r) => write!(f, "({} | {})", l, r),
            Xor(ref l, ref r) => write!(f, "({} ^ {})", l, r),
            IfThen(ref l, ref r) => write!(f, "{} => {}", l, r),
            IfAndOnlyIf(ref l, ref r) => write!(f, "{} <=> {}", l, r),
        }
    }
}
