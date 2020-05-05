use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct Facts {
    yes: HashSet<char>,
    no: HashSet<char>,
    unknown: HashSet<char>,
}

impl Facts {
    pub fn new_from_chars_str(chars: &str) -> Facts {
        let mut yes = HashSet::with_capacity(32);
        let no = HashSet::new();
        let unknown = HashSet::new();

        for c in chars.chars().map(|c| c.to_ascii_uppercase()) {
            yes.insert(c);
        }

        Facts { yes, no, unknown }
    }

    pub fn new(yes: &[char], no: &[char], unknown: &[char]) -> Facts {
        let yes: HashSet<char> = yes.iter().cloned().collect();
        let no: HashSet<char> = no.iter().cloned().collect();
        let unknown = unknown
            .iter()
            .filter(|fact| !yes.contains(fact) && !no.contains(fact))
            .cloned()
            .collect();

        Facts { yes, no, unknown }
    }

    /// Merge `self` with `other`, returning `None` if facts are controversial
    /// and removing all known facts from unknown list
    pub fn merge(&self, other: &Facts) -> Option<Facts> {
        let yes: HashSet<char> = self.yes.iter().chain(other.yes.iter()).cloned().collect();
        let no: HashSet<char> = self.no.iter().chain(other.no.iter()).cloned().collect();

        for fact in yes.iter() {
            if no.contains(fact) {
                return None;
            }
        }

        let unknown: HashSet<char> = self.unknown
            .iter()
            .chain(other.unknown.iter())
            .filter(|fact| !yes.contains(fact) && !no.contains(fact))
            .cloned()
            .collect();

        Some(Facts { yes, no, unknown })
    }

    /// Make true facts false and false facts true
    pub fn invert(&self) -> Facts {
        Facts { yes: self.no.clone(), no: self.yes.clone(), unknown: self.unknown.clone() }
    }

    pub fn is_empty(&self) -> bool {
        self.yes.is_empty() && self.no.is_empty()
    }

    pub fn yes(&self, c: char) -> bool {
        self.yes.get(&c).is_some()
    }

    pub fn no(&self, c: char) -> bool {
        self.no.get(&c).is_some()
    }

    pub fn unknown(&self, c: char) -> bool {
        self.unknown.get(&c).is_some()
    }

    /// Remove facts that are known in `other` from self
    pub fn remove_contained(&mut self, other: &Facts) {
        for fact in other.yes.iter() {
            self.yes.remove(fact);
        }

        for fact in other.no.iter() {
            self.yes.remove(fact);
        }
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

        if !self.unknown.is_empty() {
            write!(f, " false: ")?;

            let mut keys = self.unknown.iter().collect::<Vec<_>>();
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
