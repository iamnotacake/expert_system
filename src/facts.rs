use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Facts {
    yes: [bool; 26],
    no: [bool; 26],
}

impl Facts {
    pub fn new(chars: &str) -> Facts {
        let mut yes = [false; 26];
        let mut no = [false; 26];

        for c in chars.chars().map(|c| c.to_ascii_uppercase()) {
            yes[(c as usize) - ('A' as usize)] = true;
        }

        Facts { yes, no }
    }
}

impl fmt::Display for Facts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( ")?;

        if self.yes.iter().any(|&c| c) {
            write!(f, "true: ")?;

            for (idx, &val) in self.yes.iter().enumerate() {
                if val {
                    write!(f, "{}", (idx + 'A' as usize) as u8 as char)?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        if self.no.iter().any(|&c| c) {
            write!(f, " false: ")?;

            for (idx, &val) in self.no.iter().enumerate() {
                if val {
                    write!(f, "{}", (idx + 'A' as usize) as u8 as char)?;
                } else {
                    write!(f, ".")?;
                }
            }
        }

        write!(f, " )")
    }
}
