use std::fmt;

use super::Facts;

#[derive(Debug, PartialEq, Eq, Hash)]
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
            IfThen(ref l, _) => l.contains_facts_recursive(facts),
            IfAndOnlyIf(ref l, ref r) => {
                l.contains_facts_recursive(facts) || r.contains_facts_recursive(facts)
            }
            _ => unreachable!(),
        }
    }

    pub fn can_give(&self, facts: &Facts) -> bool {
        use Rule::*;

        match self {
            IfThen(_, ref r) => r.contains_facts_recursive(facts),
            IfAndOnlyIf(ref l, ref r) => {
                l.contains_facts_recursive(facts) || r.contains_facts_recursive(facts)
            }
            _ => unreachable!(),
        }
    }

    fn contains_facts_recursive(&self, facts: &Facts) -> bool {
        use Rule::*;

        match self {
            Char(ref c) => {
                let idx = (*c as usize) - ('A' as usize);
                facts.yes[idx] || facts.no[idx]
            }
            Not(ref l) => l.contains_facts_recursive(facts),
            And(ref l, ref r) => {
                l.contains_facts_recursive(facts) || r.contains_facts_recursive(facts)
            }
            Or(ref l, ref r) => {
                l.contains_facts_recursive(facts) || r.contains_facts_recursive(facts)
            }
            Xor(ref l, ref r) => {
                l.contains_facts_recursive(facts) || r.contains_facts_recursive(facts)
            }
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
