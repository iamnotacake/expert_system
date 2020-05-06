use crossterm::style::Colorize;
use expert_system::{parser, Facts, Query, Rule};
use rustyline::error::ReadlineError;
use std::collections::HashSet;

fn run(
    rules: &HashSet<Rule>,
    used_rules: HashSet<Rule>,
    mut facts: Facts,
    level: usize
) -> Facts {
    macro_rules! levelprintln {
        ($fmt:literal) => {
            println!(concat!("{}", $fmt), "  ".repeat(level))
        };
        ($fmt:literal, $( $args:expr ),*) => {
            println!(concat!("{}", $fmt), "  ".repeat(level), $( $args ),*)
        };
    }

    if level > 20 {
        panic!("recursion level too big");
    }

    levelprintln!("{}", facts.to_string().green());

    if facts.is_empty(false, false, true) {
        levelprintln!("Unknown list is empty, returning");
        return facts;
    }

    for rule in rules.iter() {
        if let Rule::IfThen(ref l, ref r) = rule {
            let (l_facts, r_facts) =
                (l.iter_facts().collect::<Vec<_>>(), r.iter_facts().collect::<Vec<_>>());

            // If we already tried this rule
            if used_rules.contains(rule) {
                // And we don't have all the facts it needs
                if l_facts.iter().any(|&fact| facts.is_unknown(fact)) {
                    // levelprintln!("Rule {} already used, skipping", rule.to_string().cyan());
                    // Then just skip it
                    continue;
                } else {
                    // levelprintln!("Rule {} already used", rule.to_string().cyan());
                }
            }

            if r_facts.iter().any(|&fact| facts.is_unknown(fact)) {
                levelprintln!("Using {}", rule.to_string().blue());

                if let Some(outcomes) = rule.try_match(&facts) {
                    levelprintln!("{} possible outcome{}",
                                  outcomes.len(),
                                  if outcomes.len() > 1 { "s" } else { "" });

                    for outcome in outcomes.iter() {
                        levelprintln!("Trying with {}", outcome);

                        if let Some(merged_facts) = facts.merge(outcome) {
                            // levelprintln!("Merged {}", merged_facts.to_string().cyan());
                            if merged_facts.unknown.len() == facts.unknown.len() {
                                levelprintln!("{}", "Does not give needed facts".to_string().yellow());
                                continue;
                            }

                            let mut used_rules = used_rules.clone();
                            used_rules.insert(rule.clone());

                            let new_facts = run(rules, used_rules, merged_facts, level + 1);
                            if new_facts.unknown.is_empty() {
                                return new_facts;
                            }
                        } else {
                            levelprintln!("{}", "Conflict".to_string().red());
                        }
                    }
                } else {
                    if used_rules.contains(rule) {
                        levelprintln!("{}", "No match".to_string().yellow());
                        continue;
                    }

                    let mut used_rules = used_rules.clone();
                    used_rules.insert(rule.clone());

                    let mut facts = facts.clone();
                    for l_fact in l_facts.iter() {
                        if !facts.is_yes(*l_fact) && !facts.is_no(*l_fact) {
                            facts.unknown.insert(*l_fact);
                        }
                    }

                    let new_facts = run(rules, used_rules, facts, level + 1);
                    if new_facts.unknown.is_empty() {
                        return new_facts;
                    }
                }
            }
        }
    }

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
                            let result = run(&rules, HashSet::new(), facts.clone(), 0);
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
