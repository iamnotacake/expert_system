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
