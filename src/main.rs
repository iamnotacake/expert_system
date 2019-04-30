extern crate expert_system;
extern crate failure;
extern crate rustyline;

use expert_system::parser;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                match parser::fact(&line) {
                    Ok(fact) => {
                        dbg!(&fact);

                        // TODO
                    }
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
