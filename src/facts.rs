use std::collections::HashSet;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Facts {
    yes: HashSet<char>,
    no: HashSet<char>,
}

impl Facts {
    pub fn new(chars: &str) -> Facts {
        let mut yes = HashSet::with_capacity(32);
        let mut no = HashSet::with_capacity(32);

        for c in chars.chars().map(|c| c.to_ascii_uppercase()) {
            yes.insert(c);
        }

        Facts { yes, no }
    }

    pub fn yes(&self, c: char) -> bool {
        self.yes.get(&c).is_some()
    }

    pub fn no(&self, c: char) -> bool {
        self.no.get(&c).is_some()
    }
}

impl fmt::Display for Facts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;

        if !self.yes.is_empty() {
            write!(f, " true: ")?;

            let mut keys = self.yes.iter().collect::<Vec<_>>();
            keys.sort();

            for (idx, key) in keys.iter().enumerate() {
                if idx == 0 {
                    write!(f, "{}", key);
                } else {
                    write!(f, ", {}", key);
                }
            }

            write!(f, " ")?;
        }

        if !self.no.is_empty() {
            write!(f, " false: ")?;

            let mut keys = self.no.iter().collect::<Vec<_>>();
            keys.sort();

            for (idx, key) in keys.iter().enumerate() {
                if idx == 0 {
                    write!(f, "{}", key);
                } else {
                    write!(f, ", {}", key);
                }
            }

            write!(f, " ")?;
        }

        write!(f, ")")
    }
}
