pub mod automata;
pub mod dfa;
pub mod graph;
pub mod nfa;
pub mod postfixer;
pub mod regex;

use std::{env, error::Error, fs};

use automata::AutomataState;
use dfa::build_dfa;
use graph::{Graph, NodeIndex};
use nfa::build_nfa;
use regex::get_alphabet;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = search(&config.query, &contents, config.ignore_case);

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let mut results = Vec::new();

    let postfix_regex = postfixer::transform(query.to_string()).unwrap();

    let alphabet = get_alphabet(&postfix_regex);

    let (handle, nfa) = build_nfa(postfix_regex);

    let (start, dfa) = build_dfa(handle, nfa, alphabet);

    for line in contents.lines() {
        if check_line_matches(start, &dfa, line, ignore_case) {
            results.push(line);
        }
    }

    return results;
}

fn check_line_matches(
    start_of_dfa: NodeIndex,
    dfa: &Graph<AutomataState, char>,
    line: &str,
    ignore_case: bool,
) -> bool {
    for i in 0..line.len() {
        let sub_line = &line[i..];

        let automata_has_accepted = run_automata(start_of_dfa, dfa, sub_line, ignore_case);

        if automata_has_accepted {
            return true;
        }
    }

    return false;
}

fn run_automata(
    start_of_dfa: NodeIndex,
    dfa: &Graph<AutomataState, char>,
    sub_line: &str,
    ignore_case: bool,
) -> bool {
    let mut current_node = start_of_dfa;

    for c in sub_line.chars() {
        let mut can_progress = false;
        let outgoing_edges = dfa.outgoing_edges(current_node).unwrap();

        for edge in outgoing_edges {
            let data = dfa.get_edge_data(&edge).unwrap();
            let label = *data.borrow();

            if ignore_case {
                if label.to_lowercase().to_string() == c.to_lowercase().to_string() {
                    current_node = dfa.traverse(edge).unwrap();
                    can_progress = true;
                }
            } else {
                if label == c {
                    current_node = dfa.traverse(edge).unwrap();
                    can_progress = true;
                }
            }
        }

        if !can_progress {
            break;
        }
    }

    let data = dfa.get_node_data(&current_node).unwrap();
    let label = data.borrow();

    return label.is_accepting();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_basic_input_when_searching_in_case_sensitive_mode_should_return_answers_and_respect_case(
    ) {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, false)
        );
    }

    #[test]
    fn given_basic_input_when_searching_in_case_insensitive_mode_should_return_answers_and_not_respect_case(
    ) {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }

    #[test]
    fn given_test_input_when_searching_with_alternation_operator_should_correctly_return_answers() {
        let query = "(safe)|(three)";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
            search(query, contents, false)
        );
    }

    #[test]
    fn given_test_input_when_searching_with_star_operator_should_correctly_return_answers() {
        let query = "thre*";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Pick three."], search(query, contents, false));
    }

    #[test]
    fn given_test_input_when_searching_with_plus_operator_should_correctly_return_answers() {
        let query = ",+";
        let contents = "\
Rust:
safe, fast, productive.
Pick three,
Trust me.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three,"],
            search(query, contents, false)
        );
    }

    #[test]
    fn given_test_input_when_searching_with_optional_operator_should_correctly_return_answers() {
        let query = "e,?";
        let contents = "\
Rust:
safe, fast, productive.
Pick three,
Trust me.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three,", "Trust me."],
            search(query, contents, false)
        );
    }
}
