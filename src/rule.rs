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

    pub fn possible_inputs(&self) -> Vec<Facts> {
        use Rule::*;

        match self {
            IfThen(ref l, _) => l.possible_combinations_recursive(),
            IfAndOnlyIf(ref l, ref r) => {
                let mut res = l.possible_combinations_recursive();
                res.extend(r.possible_combinations_recursive());
                res
            },
            _ => unreachable!(),
        }
    }

    pub fn possible_outputs(&self) -> Vec<Facts> {
        use Rule::*;

        match self {
            IfThen(_, ref r) => r.possible_combinations_recursive(),
            IfAndOnlyIf(ref l, ref r) => {
                let mut res = l.possible_combinations_recursive();
                res.extend(r.possible_combinations_recursive());
                res
            },
            _ => unreachable!(),
        }
    }

    pub fn possible_combinations_recursive(&self) -> Vec<Facts> {
        use Rule::*;

        let mut res = Vec::new();

        match self {
            Char(ref c) => {
                let facts = Facts::new_yes_no(&[*c], &[]);
                res.push(facts);
            }
            Not(ref l) => {
                for fact in l.possible_combinations_recursive() {
                    res.push(fact.invert());
                }
            }
            And(ref l, ref r) => {
                let possible_l = l.possible_combinations_recursive();
                let possible_r = r.possible_combinations_recursive();

                for p_l in possible_l.iter() {
                    for p_r in possible_r.iter() {
                        if let Some(merged) = p_l.merge(p_r) {
                            res.push(merged);
                        }
                    }
                }
            }
            Or(ref l, ref r) => {
                let possible_l = l.possible_combinations_recursive();
                let possible_r = r.possible_combinations_recursive();

                for p_l in possible_l.iter() {
                    res.push(p_l.clone());
                }

                for p_r in possible_r.iter() {
                    res.push(p_r.clone());
                }

                for p_l in possible_l.iter() {
                    for p_r in possible_r.iter() {
                        if let Some(merged) = p_l.merge(p_r) {
                            res.push(merged);
                        }
                    }
                }
            }
            Xor(ref l, ref r) => {
                let possible_l = l.possible_combinations_recursive();
                let possible_r = r.possible_combinations_recursive();

                for p_l in possible_l.iter() {
                    for p_r in possible_r.iter().map(|f| f.invert()) {
                        if let Some(merged) = p_l.merge(&p_r) {
                            res.push(merged);
                        }
                    }
                }

                for p_l in possible_l.iter().map(|f| f.invert()) {
                    for p_r in possible_r.iter() {
                        if let Some(merged) = p_l.merge(p_r) {
                            res.push(merged);
                        }
                    }
                }
            }
            _ => unreachable!(),
        }

        res
    }
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Rule::*;

        match self {
            Char(ref c) => write!(f, "{}", c),
            Not(box Char(ref c)) => write!(f, "!{}", c),
            Not(ref l) => write!(f, "!({})", l),
            And(ref l, ref r) => write!(f, "({} + {})", l, r),
            Or(ref l, ref r) => write!(f, "({} | {})", l, r),
            Xor(ref l, ref r) => write!(f, "({} ^ {})", l, r),
            IfThen(ref l, ref r) => {
                match l {
                    box And(ref l, ref r) => write!(f, "{} + {}", l, r)?,
                    box Or(ref l, ref r) => write!(f, "{} | {}", l, r)?,
                    box Xor(ref l, ref r) => write!(f, "{} ^ {}", l, r)?,
                    box x => write!(f, "{}", x)?,
                }
                write!(f, " => ")?;
                match r {
                    box And(ref l, ref r) => write!(f, "{} + {}", l, r),
                    box Or(ref l, ref r) => write!(f, "{} | {}", l, r),
                    box Xor(ref l, ref r) => write!(f, "{} ^ {}", l, r),
                    box x => write!(f, "{}", x),
                }
            }
            IfAndOnlyIf(ref l, ref r) => {
                match l {
                    box And(ref l, ref r) => write!(f, "{} + {}", l, r)?,
                    box Or(ref l, ref r) => write!(f, "{} | {}", l, r)?,
                    box Xor(ref l, ref r) => write!(f, "{} ^ {}", l, r)?,
                    box x => write!(f, "{}", x)?,
                }
                write!(f, " <=> ")?;
                match r {
                    box And(ref l, ref r) => write!(f, "{} + {}", l, r),
                    box Or(ref l, ref r) => write!(f, "{} | {}", l, r),
                    box Xor(ref l, ref r) => write!(f, "{} ^ {}", l, r),
                    box x => write!(f, "{}", x),
                }
            }
        }
    }
}
