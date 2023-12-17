use std::{fmt, collections::VecDeque};

type OperatorPrecedence = usize;

#[derive(PartialEq, Eq)]
enum OperatorType {
    None,
    Unary,
    Binary
}

#[derive(PartialEq, Eq)]
pub enum RegexSymbol {
    Optional,
    Plus,
    Star,
    Concat,
    Alternation,
    Char(char)
}


pub fn transform(regex: String) -> Result<VecDeque<RegexSymbol>, String> {
    let regex = check_start_and_end_chars(regex)?;
    let regex = check_for_illegal_operator_sequences(regex)?;

    let formatted = format(regex);

    return convert(formatted);
}

fn format(regex: String) -> Vec<RegexSymbol> {
    let mut formatted: Vec<RegexSymbol> = Vec::new();
    let mut iter = regex.chars().peekable();

    // Loop through and add concat symbols inbetween valid slots
    while let Some(c) = iter.next() {
        let current: char = c;
        let next = match iter.peek() {
            Some(c) => *c,
            None => {
                formatted.push(RegexSymbol::from_char(current)); // Add the final character to the formatted string
                continue;
            }
        };

        formatted.push(RegexSymbol::from_char(current));

        // Concatenation can't occur after an opening bracket e.g. '(' 
        // Concatenation can't occur after a binary operator e.g. '|'
        // Concatenation can't occur before a closing bracket e.g. ')'
        // Concatenation can't occur before an operator e.g. '+'          
        if current != '(' 
                && !RegexSymbol::is_binary_operator(current) 
                && next != ')' 
                && !RegexSymbol::is_operator(next) {
            formatted.push(RegexSymbol::Concat);
        }
    }

    return formatted;
}

/*
Uses the shunting yard algorithm to convert infix regex to postfix regex.
The algorithm works by keeping an output queue as the final result and taking advantage
of a stack's LIFO structure to arrange the operators correctly onto the queue.

It iterates over each character and parks them according to the following rules

1. If it's just a character, put it straight onto the output queue

2. If it's an open parenthesis, push it onto the operator stack

3. If it's a close parenthesis
    a. Pop the remaining operators off the stack onto the output queue until you get an open parenthesis
    b. Pop the open parenthesis to effectively finish converting that sub scope

4. If it's an operator before we push it onto the stack, we'll keep popping off until
    a. You've run out operators to pop or equivalently you've hit a scope boundary with an open parenthesis
    b. The operator on the top of the stack has a lower precedence
        - This is because we want to evaluate higher precedence operators first and therefore we push them
          to the output queue before we push lower priority ones.

5. Pop the remaining operators off the stack onto the output queue until there's no more
    - This last step is to just clean things up and finalise the postfix notation by utilising the LIFO output
      of any remaining operators on the stack.
*/
fn convert(formatted: Vec<RegexSymbol>) -> Result<VecDeque<RegexSymbol>, String>{
    let mut output_queue: VecDeque<RegexSymbol> = VecDeque::new();
    let mut operator_stack: Vec<RegexSymbol> = Vec::new();

    for symbol in formatted {
        if let RegexSymbol::Char('(') = symbol {
            operator_stack.push(symbol)
        }
        else if let RegexSymbol::Char(')') = symbol {
            // If the stack runs out without finding a left parenthesis, then there are mismatched parentheses.
            let mut found_corresponding_bracket = false;

            while !found_corresponding_bracket {
                match operator_stack.last() {
                    Some(top) => {
                        if *top != RegexSymbol::Char('(') {
                            output_queue.push_back(operator_stack.pop().unwrap());
                        }
                        else {
                            found_corresponding_bracket = true;
                        }
                    },
                    None => return Err("Error - Unbalanced brackets".to_string())
                }
            }
            // Pop the corresponding parenthesis we just encountered off the stack
            operator_stack.pop().unwrap();
        }
        else if RegexSymbol::get_type(&symbol) != OperatorType::None {
            if RegexSymbol::get_type(&symbol) == OperatorType::Binary { 
                // All binary operators are left associative in RegEx, so <= is used to respect the grouping.
                // i.e. we want it to be evaluated from left to right
                while !operator_stack.is_empty() 
                        && *operator_stack.last().unwrap() != RegexSymbol::Char('(') 
                        && RegexSymbol::get_precedence(&symbol) <= RegexSymbol::get_precedence(operator_stack.last().unwrap()) {
                    output_queue.push_back(operator_stack.pop().unwrap());
                }
            }

            // Don't bother popping unary ops, they're all of the same precedence and always follows the operand.
            operator_stack.push(symbol);
        }
        else {
            output_queue.push_back(symbol);
        }
    }

    // After the main loop, pop the remaining items from the operator stack into the output queue.
    while !operator_stack.is_empty() {
        // If the operator token on the top of the stack is a parenthesis, then there are mismatched parentheses.
        if *operator_stack.last().unwrap() == RegexSymbol::Char('(') {
            return Err("Error - Unbalanced brackets".to_string());
        }

        output_queue.push_back(operator_stack.pop().unwrap());
    }

    return Ok(output_queue);
}

fn check_start_and_end_chars(regex: String) -> Result<String, String> {
    if regex.starts_with(|c| RegexSymbol::is_operator(c)) {
        return Err("Error - Illegal operator usage at start of string".to_string());
    }

    if regex.ends_with(|c| RegexSymbol::is_binary_operator(c)) {
        return Err("Error - Illegal operator usage at end of string".to_string());
    }

    return Ok(regex);
}

fn check_for_illegal_operator_sequences(regex: String) -> Result<String, String> {
    let mut i = 0;
    let mut iter = regex.chars().peekable();

    while let Some(c) = iter.next() {
        let current: char = c;
        let next = match iter.peek() {
            Some(c) => *c,
            None => continue
        };

        if RegexSymbol::is_binary_operator(current) && RegexSymbol::is_operator(next) 
        || RegexSymbol::is_unary_operator(current) && RegexSymbol::is_unary_operator(next) {
            return Err(format!("Error - Illegal operator sequence: {}{}, starting at position: {}", current, next, i))
        }

        i += 1;
    }

    return Ok(regex);
}


impl RegexSymbol {
    fn from_char(c: char) -> RegexSymbol {
        return match c {
            '?' => RegexSymbol::Optional,
            '+' => RegexSymbol::Plus,
            '*' => RegexSymbol::Star,
            '|' => RegexSymbol::Alternation,
            c => RegexSymbol::Char(c)
        }
    }

    fn get_precedence(&self) -> OperatorPrecedence {
        return match self {
            RegexSymbol::Optional => 3,
            RegexSymbol::Plus => 3,
            RegexSymbol::Star => 3,
            RegexSymbol::Concat => 2,
            RegexSymbol::Alternation => 1,
            RegexSymbol::Char(_) => 0
        }
    }

    fn get_type(&self) -> OperatorType {
        return match self {
            RegexSymbol::Optional => OperatorType::Unary,
            RegexSymbol::Plus => OperatorType::Unary,
            RegexSymbol::Star => OperatorType::Unary,
            RegexSymbol::Concat => OperatorType::Binary,
            RegexSymbol::Alternation => OperatorType::Binary,
            RegexSymbol::Char(_) => OperatorType::None

        }
    }

    pub fn is_operator(c: char) -> bool {
        return RegexSymbol::is_unary_operator(c) || RegexSymbol::is_binary_operator(c);
    }

    pub fn is_unary_operator(c: char) -> bool {
        return c == '?' || c == '+' || c == '*';
    }

    pub fn is_binary_operator(c: char) -> bool {
        return c == '|';
    }
}

impl fmt::Display for RegexSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RegexSymbol::Optional =>  write!(f, "?"),
            RegexSymbol::Plus => write!(f, "+"),
            RegexSymbol::Star => write!(f, "*"),
            RegexSymbol::Concat => write!(f, "."),
            RegexSymbol::Alternation => write!(f, "|"),
            RegexSymbol::Char(c) => write!(f, "{}", c),
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn given_valid_basic_examples_when_formatting_it_should_correctly_do_so() {
        let examples = ["aaron", "(a)(a)", "(aa)"];
        let answers = ["a.a.r.o.n", "(a).(a)", "(a.a)"];

        for i in 0..examples.len() {
            let result: String = format(examples[i].to_string())
                .iter()
                .map(|x| x.to_string())
                .collect();
            
            let answer = answers[i];

            assert_eq!(result, answer);
        }
    }

    #[test]
    fn given_valid_examples_with_unary_operators_when_formatting_it_should_correctly_do_so() {
        let examples = ["aa*", "a*a", "a*a*", "(a)*a", "a*(a)"];
        let answers = ["a.a*", "a*.a", "a*.a*", "(a)*.a", "a*.(a)"];

        for i in 0..examples.len() {
            let result: String = format(examples[i].to_string())
                .iter()
                .map(|x| x.to_string())
                .collect();
            
            let answer = answers[i];

            assert_eq!(result, answer);
        }
    }

    #[test]
    fn given_valid_examples_when_formatting_it_should_not_concatenate_operators_together() {
        let examples = ["(a)", "a|a", "a*", "((a))"];
        let answers = ["(a)", "a|a", "a*", "((a))"];

        for i in 0..examples.len() {
            let result: String = format(examples[i].to_string())
                .iter()
                .map(|x| x.to_string())
                .collect();
            
            let answer = answers[i];

            assert_eq!(result, answer);
        }
    }

    #[test]
    fn given_valid_complicated_examples_when_formatting_it_should_correctly_do_so() {
        let examples = ["a?a?a?aaa", "a(bb)+a", "ab|bc"];
        let answers = ["a?.a?.a?.a.a.a", "a.(b.b)+.a", "a.b|b.c"];

        for i in 0..examples.len() {
            let result: String = format(examples[i].to_string())
                .iter()
                .map(|x| x.to_string())
                .collect();
            
            let answer = answers[i];

            assert_eq!(result, answer);
        }
    }

    #[test]
    fn given_valid_complicated_examples_when_transforming_it_should_correctly_output_postfix() {
        let examples = ["a", "a(bb)+a", "abcdefg", "(a|b)*a", "a(b|c)*d", "a*(b+|(a|b))?(c|d)"];
        let answers = ["a", "abb.+.a.", "ab.c.d.e.f.g.", "ab|*a.", "abc|*.d.", "a*b+ab||?.cd|."];

        for i in 0..examples.len() {
            let result: String = transform(examples[i].to_string())
                .unwrap()
                .iter()
                .map(|x| x.to_string())
                .collect();
            
            let answer = answers[i];

            assert_eq!(result, answer);
        }
    }

    #[test]
    fn given_invalid_examples_when_transforming_it_should_reject_them() {
        let examples = ["*a", "|a", "(a))", "((a)", "a|", "a||a", "a**a"];

        for example in examples {
            let result = transform(example.to_string());

            assert!(result.is_err());
        }
    }
}