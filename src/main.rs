extern crate expert_system;
extern crate failure;
extern crate rustyline;

use expert_system::{parser, Facts, Query, Rule};
use rustyline::error::ReadlineError;
use std::collections::HashSet;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut rules = HashSet::new();
    let mut facts = Facts::new("");

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                match parser::query(&line) {
                    Ok(query) => match query {
                        Query::Rule(rule) => {
                            println!("Rule: {}", rule);
                            rules.insert(rule);
                        }
                        Query::Given(list) => {
                            println!("Have: {}", &list);
                            facts = list;
                        }
                        Query::Find(find) => {
                            println!("Find: {}", find);

                            // TODO
                        }
                        Query::Dump => {
                            println!("*** Rules:");
                            for rule in rules.iter() {
                                println!("***   {}", rule);
                            }

                            println!("*** Facts: {}", facts);
                        }
                        Query::Delete(rule) => {
                            if !rules.remove(&rule) {
                                eprintln!("Rule not found");
                            }
                        }
                    },
                    Err(expert_system::parser::ParseError {
                        offset, expected, ..
                    }) => {
                        eprintln!("  {}^", " ".repeat(offset));
                        eprintln!("Expected: {:?}", expected);
                    }
                }

                rl.add_history_entry(line);
            }
            Err(ReadlineError::Interrupted) => {}
            Err(ReadlineError::Eof) => break,
            Err(err) => panic!("{}", err),
        }
    }
}
