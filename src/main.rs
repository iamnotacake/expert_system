use crossterm::style::Colorize;
use expert_system::{parser, Facts, Query, Rule};
use rustyline::error::ReadlineError;
use std::collections::HashSet;

fn run(usable_rules: HashSet<Rule>, mut facts: Facts, level: usize) -> Facts {
    macro_rules! levelprintln {
        ($fmt:literal) => {
            println!(concat!("{}", $fmt), "  ".repeat(level))
        };
        ($fmt:literal, $( $args:expr ),*) => {
            println!(concat!("{}", $fmt), "  ".repeat(level), $( $args ),*)
        };
    }

    // TODO

    return facts;
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut rules = HashSet::new();
    let mut facts = Facts::new(&[], &[], &[]);

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let line = line.trim_end();

                match parser::query(&line) {
                    Ok(query) => match query {
                        Query::Rule(rule) => {
                            println!("Rule: {}", rule);
                            // println!("Mentioned facts: {:?}", rule.iter_facts().collect::<Vec<_>>());
                            // let possible_inputs = rule.possible_inputs();
                            // for input in possible_inputs.iter() {
                            //     println!("Possible input: {}", input);
                            // }
                            // let possible_outputs = rule.possible_outputs();
                            // for output in possible_outputs.iter() {
                            //     println!("Possible output: {}", output);
                            // }
                            rules.insert(rule);
                        }
                        Query::Given(list) => {
                            println!("Have: {}", &list);
                            facts = list;
                        }
                        Query::Find(find) => {
                            println!("Find: {}", find);

                            facts = facts.merge(&find).unwrap();
                            let result = run(rules.clone(), facts.clone(), 0);
                            println!("Result: {}", result);
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
                    Err(e) => {
                        eprintln!("  {}^", " ".repeat(e.location.offset));
                        eprintln!("{}", e);
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
