#![feature(box_syntax, box_patterns)]

pub mod facts;
pub use facts::Facts;

pub mod rule;
pub use rule::Rule;

#[derive(Debug, PartialEq)]
pub enum Query {
    Rule(Rule),
    Given(Facts),
    Find(Facts),
    Dump,
    Delete(Rule),
}

peg::parser! {
    pub grammar parser() for str {
        pub rule query() -> Query
            = l:rul() { Query::Rule(l) }
            / l:given() { Query::Given(l) }
            / l:find() { Query::Find(l) }
            / "dump" { Query::Dump }
            / "delete" whitespace()+ l:rul() { Query::Delete(l) }

        pub rule rul() -> Rule
            = IfThen()
            / IfAndOnlyIf()

        rule whitespace()
            = quiet!{[' ' | '\t']+}

        rule Atom() -> Rule
            = whitespace()? "(" whitespace()? l:Expr() whitespace()? ")" whitespace()? { l }
            / whitespace()? l:Char() whitespace()? { l }
            / whitespace()? l:Not() whitespace()? { l }

        rule Char() -> Rule
            = c:$(['a'..='z' | 'A'..='Z']) { Rule::Char(c.chars().nth(0).unwrap().to_ascii_uppercase()) }

        rule Not() -> Rule
            = "!" l:Atom() { Rule::Not(box l) }

        rule Expr() -> Rule = precedence!{
            l:(@) "^" r:@ { Rule::Xor(box l, box r) }
            l:(@) "|" r:@ { Rule::Or(box l, box r) }
            l:(@) "+" r:@ { Rule::And(box l, box r) }
            --
            l:Atom() { l }
        }

        rule IfThen() -> Rule
            = l:Expr() "=>" r:Expr() { Rule::IfThen(box l, box r) }

        rule IfAndOnlyIf() -> Rule
            = l:Expr() "<=>" r:Expr() { Rule::IfAndOnlyIf(box l, box r) }

        pub rule given() -> Facts
            = "=" c:$(['a'..='z' | 'A'..='Z']*) { Facts::new(c) }

        pub rule find() -> Facts
            = "?" c:$(['a'..='z' | 'A'..='Z']+) { Facts::new(c) }
    }
}
