use std::fmt;

pub type OperatorPrecedence = usize;

#[derive(PartialEq, Eq)]
pub enum OperatorType {
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
    Open,
    Close,
    Char(char)
}

impl RegexSymbol {
    pub fn from_char(c: char) -> RegexSymbol {
        return match c {
            '?' => RegexSymbol::Optional,
            '+' => RegexSymbol::Plus,
            '*' => RegexSymbol::Star,
            '|' => RegexSymbol::Alternation,
            '(' => RegexSymbol::Open,
            ')' => RegexSymbol::Close,
            c => RegexSymbol::Char(c)
        }
    }

    pub fn get_escaped(c: char) -> Result<RegexSymbol, String> {
        return match c {
            '?' => Ok(RegexSymbol::Char('?')),
            '+' => Ok(RegexSymbol::Char('+')),
            '*' => Ok(RegexSymbol::Char('*')),
            '|' => Ok(RegexSymbol::Char('|')),
            '(' => Ok(RegexSymbol::Char('(')),
            ')' => Ok(RegexSymbol::Char(')')),
            't' => Ok(RegexSymbol::Char('\t')),
            'b' => Ok(RegexSymbol::Char('\u{0008}')),
            'n' => Ok(RegexSymbol::Char('\n')),
            'r' => Ok(RegexSymbol::Char('\r')),
            'f' => Ok(RegexSymbol::Char('\u{000A}')),
            '\\' => Ok(RegexSymbol::Char('\\')),
            c => Err(format!("Error - Invalid escaped character: \\{}", c))
        }
    }

    pub fn get_precedence(&self) -> OperatorPrecedence {
        return match self {
            RegexSymbol::Optional => 3,
            RegexSymbol::Plus => 3,
            RegexSymbol::Star => 3,
            RegexSymbol::Concat => 2,
            RegexSymbol::Alternation => 1,
            _ => 0
        }
    }

    pub fn get_type(&self) -> OperatorType {
        return match self {
            RegexSymbol::Optional => OperatorType::Unary,
            RegexSymbol::Plus => OperatorType::Unary,
            RegexSymbol::Star => OperatorType::Unary,
            RegexSymbol::Concat => OperatorType::Binary,
            RegexSymbol::Alternation => OperatorType::Binary,
            _ => OperatorType::None
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
            RegexSymbol::Open => write!(f, "("),
            RegexSymbol::Close => write!(f, ")"),
            RegexSymbol::Char(c) => write!(f, "{}", c),
        }
    }
}
