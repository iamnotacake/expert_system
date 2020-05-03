use expert_system::{parser, Facts, Query, Rule};
use rustyline::error::ReadlineError;
use std::collections::HashSet;

fn run(usable_rules: HashSet<Rule>, given: Facts, mut find: Facts, level: usize) -> Facts {
    find.remove_contained(&given);

    if find.is_empty() {
        println!("{}search list empty, returning {}", "  ".repeat(level), given);
        return given;
    }

    if usable_rules.is_empty() {
        println!("{}no more rules to use, returning {}", "  ".repeat(level), given);
        return given;
    }

    for rule in usable_rules.iter() {
        if rule.can_give(&find) {
            // recurse into
            let mut usable_rules = usable_rules.clone();
            usable_rules.remove(&rule);

            println!("{}try {} with {}", "  ".repeat(level), rule, given);
            run(usable_rules, given.clone(), find.clone(), level + 1);
        } else {
            println!("{}{} can't give {}", "  ".repeat(level), rule, find);
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
