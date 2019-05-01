extern crate expert_system;
extern crate failure;
extern crate rustyline;

use expert_system::{parser, Fact, Facts, Query};
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                match parser::query(&line) {
                    Ok(query) => match query {
                        Query::Fact(fact) => {
                            println!("Fact: {}", fact);

                            // TODO
                        }
                        Query::Given(facts) => {
                            println!("Have: {}", facts);

                            // TODO
                        }
                        Query::Find(find) => {
                            println!("Find: {}", find);

                            // TODO
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
