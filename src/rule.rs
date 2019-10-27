use std::fmt;

use super::Facts;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Rule {
    Char(char),
    Not(Box<Rule>),
    And(Box<Rule>, Box<Rule>),
    Or(Box<Rule>, Box<Rule>),
    Xor(Box<Rule>, Box<Rule>),
    IfThen(Box<Rule>, Box<Rule>),
    IfAndOnlyIf(Box<Rule>, Box<Rule>),
}

impl Rule {
    pub fn can_take(&self, facts: &Facts) -> bool {
        use Rule::*;

        match self {
            IfThen(ref l, _) => l.can_take_recursive(facts),
            IfAndOnlyIf(ref l, ref r) => l.can_take_recursive(facts) || r.can_take_recursive(facts),
            _ => unreachable!(),
        }
    }

    fn can_take_recursive(&self, facts: &Facts) -> bool {
        use Rule::*;

        match self {
            Char(ref c) => facts.yes(*c),
            Not(ref l) => !l.can_take_recursive(facts),
            And(ref l, ref r) => l.can_take_recursive(facts) && r.can_take_recursive(facts),
            Or(ref l, ref r) => l.can_take_recursive(facts) || r.can_take_recursive(facts),
            Xor(ref l, ref r) => l.can_take_recursive(facts) ^ r.can_take_recursive(facts),
            _ => unreachable!(),
        }
    }

    pub fn can_give(&self, facts: &Facts) -> bool {
        use Rule::*;

        match self {
            IfThen(_, ref r) => r.can_give_recursive(facts),
            IfAndOnlyIf(ref l, ref r) => l.can_give_recursive(facts) || r.can_give_recursive(facts),
            _ => unreachable!(),
        }
    }

    fn can_give_recursive(&self, facts: &Facts) -> bool {
        use Rule::*;

        match self {
            Char(ref c) => facts.no(*c) || facts.yes(*c),
            Not(ref l) => l.can_give_recursive(facts),
            And(ref l, ref r) => l.can_give_recursive(facts) || r.can_give_recursive(facts),
            Or(ref l, ref r) => l.can_give_recursive(facts) || r.can_give_recursive(facts),
            Xor(ref l, ref r) => l.can_give_recursive(facts) || r.can_give_recursive(facts),
            _ => unreachable!(),
        }
    }
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
