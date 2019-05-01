use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Rule {
    Char(char),
    Not(Box<Rule>),
    And(Box<Rule>, Box<Rule>),
    Or(Box<Rule>, Box<Rule>),
    Xor(Box<Rule>, Box<Rule>),
    IfThen(Box<Rule>, Box<Rule>),
    IfAndOnlyIf(Box<Rule>, Box<Rule>),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Rule::*;

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
