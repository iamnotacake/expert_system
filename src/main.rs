use crossterm::style::Colorize;
use expert_system::{parser, Facts, Query, Rule};
use rustyline::error::ReadlineError;
use std::collections::HashSet;

fn run(usable_rules: HashSet<Rule>, given: Facts, mut find: Facts, level: usize) -> Facts {
    macro_rules! levelprintln {
        ($fmt:expr, $($args:expr),*) => {
            println!(concat!("{}", $fmt), "  ".repeat(level), $($args),*)
        }
    }

    find.remove_contained(&given);

    if find.is_empty() {
        levelprintln!("search list empty, returning {}", given.to_string().yellow());
        return given;
    }

    if usable_rules.is_empty() {
        levelprintln!("no more rules to use, returning {}", given.to_string().green());
        return given;
    }

    for rule in usable_rules.iter() {
        if rule.can_give(&find) {
            // recurse into
            levelprintln!("try {} with {}", rule.to_string().blue(), given.to_string().green());

            let mut usable_rules = usable_rules.clone();
            usable_rules.remove(&rule);
            run(usable_rules, given.clone(), find.clone(), level + 1);
        } else {
            levelprintln!("{} can't give {}", rule.to_string().blue(), find.to_string().yellow());
        }
    }

    unimplemented!()
}

fn main() {
    let mut rl = rustyline::Editor::<()>::new();
    let mut rules = HashSet::new();
    let mut facts = Facts::new("");

    loop {
        match rl.readline("> ") {
            Ok(line) => {
                let line = line.trim_end();

                match parser::query(&line) {
                    Ok(query) => match query {
                        Query::Rule(rule) => {
                            println!("Rule: {}", rule);
                            let possible_inputs = rule.possible_inputs();
                            for input in possible_inputs.iter() {
                                println!("Possible input: {}", input);
                            }
                            let possible_outputs = rule.possible_outputs();
                            for output in possible_outputs.iter() {
                                println!("Possible output: {}", output);
                            }
                            rules.insert(rule);
                        }
                        Query::Given(list) => {
                            println!("Have: {}", &list);
                            facts = list;
                        }
                        Query::Find(find) => {
                            println!("Find: {}", find);

                            let result = run(rules.clone(), facts.clone(), find.clone(), 0);
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
