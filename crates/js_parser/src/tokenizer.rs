use std::fmt::Display;

use storytell_diagnostics::{location::Range, diagnostic::*, make_diagnostics, dia};
use crate::input::{InputConsumer, InputPresenter};

make_diagnostics!(define [
    END_OF_STR,
    "Unexpected end of string."
], [
    INVALID_DIGIT,
    "Invalid digit '$'."
], [
    DECIMAL_POINT,
    "Number already has a decimal point."
], [
    NUMERIC_SEPARATOR_AT_END,
    "Numeric separators are not allowed at the end of numeric literals."
], [
    EXPECTED_TOKEN,
    "Expected token '$'."
]);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    String,
    Number,
    Identifier,
    CommaPunc, //,
    SemicolonPunc, //;
    SquareBracketOpenPunc, // [
    SquareBracketClosePunc, // ]
    ParanthesisOpenPunc, // (
    ParanthesisClosePunc, // )
    PlusOp, // +
    MinusOp, // -
    StarOp, // *
    StarStarOp, // **
    SlashOp, // /
    PercentOp, // %
    EqualsEqualsEqualsOp, // ===
    EqualsEqualsOp, // ==
    NotEqualsOp, // !=
    NotEqualsEqualsOp, // !==
    AmpersandAmpersandOp, // &&
    BarBarOp, // ||
    QuestionQuestionOp, // ??
    EqualsOp, // =
    PlusEqualsOp, // +=
    MinusEqualsOp, // -=
    StarEqualsOp, // *=
    SlashEqualsOp, // /=
    ExclamationOp, // !
    DotOp, // .
    DotDotDotOp, // ...
    LessThanOp, // <
    GreaterThanOp, // >
    LessThanEqualsOp, // <=
    GreaterThanEqualsOp, // >=
    QuestionOp, // ?
    ColonOp, // :
    StringLitStart,
    VoidKeyword,
    TrueKeyword,
    FalseKeyword,
    NewKeyword
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>
}

#[derive(PartialEq, Eq)]
pub enum NumberType {
    Binary, // 0b
    Octal, // 0o
    Hex, // 0x
    None
}

pub struct Tokenizer<'a> {
    pub input: InputConsumer<'a>,
    pub last_token: Option<Token>,
    pub last_pos_before_spacing: Option<usize>,
    pub diagnostics: Vec<Diagnostic>
}

impl<'a> Tokenizer<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            input: InputConsumer::new(content),
            last_token: None,
            last_pos_before_spacing: None,
            diagnostics: vec![]
        }
    }

    fn parse_string(&mut self, end_char: char) -> Option<Token> {
        let start = self.input.pos - 1;
        loop {
            match self.input.next() {
                Some(character) if character == end_char => break,
                None => {
                    self.diagnostics.push(dia!(END_OF_STR, self.input.range(start)));
                    break;
                },
                _ => {}
            }
        }
        Some(Token {
            kind: TokenKind::String,
            range: self.input.range(start)
        })
    }

    fn parse_number(&mut self) -> Option<Token> {
        let start = self.input.pos - 1;
        let mut has_dot = false;
        let mut number_type = if let Some('0') = self.input.prev(1) {
            match self.input.peek() {
                Some('x') => {
                    self.input.skip_chars(1);
                    NumberType::Hex
                },
                Some('o') => {
                    self.input.skip_chars(1);
                    NumberType::Octal
                },
                Some('b') => {
                    self.input.skip_chars(1);
                    NumberType::Binary
                },
                _ => NumberType::None
            }
        } else { NumberType::None };
        while let Some(character) = self.input.peek() {
            match character {
                '0' => self.input.skip_chars(1),
                ch @ '1'..='9' => {
                    match number_type {
                        NumberType::Binary if ch > '1' => {
                            self.diagnostics.push(dia!(INVALID_DIGIT, self.input.range_here(), &ch.to_string()));
                            number_type = NumberType::None;
                        },
                        NumberType::Octal if ch > '7' => {
                            self.diagnostics.push(dia!(INVALID_DIGIT, self.input.range_here(), &ch.to_string()));
                            number_type = NumberType::None;
                        },
                        _ => {}
                    }
                    self.input.skip_chars(1);
                },
                'A'..='F' | 'a'..='f' if number_type == NumberType::Hex => self.input.skip_chars(1),
                '.' if number_type == NumberType::None => {
                    self.input.skip_chars(1);
                    if has_dot {
                        self.diagnostics.push(dia!(DECIMAL_POINT, self.input.range_here()));
                        break;
                    };
                    has_dot = true;
                },
                '_' => self.input.skip_chars(1),
                _ => break
            }
        };
        if self.input.prev(1)? == '_' {
            self.diagnostics.push(dia!(NUMERIC_SEPARATOR_AT_END, self.input.range(start)))
        }
        Some(Token {
            kind: TokenKind::Number,
            range: self.input.range(start)
        })
    }

    fn parse_identifier_or_keyword(&mut self) -> Option<Token> {
        let start = self.input.pos - 1;
        while let Some(character) = self.input.peek() {
            match character {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$' => {
                    self.input.skip_chars(1);
                    continue;
                },
                _ => break
            }
        }
        let range = self.input.range(start);
        let kind = match self.input.data.from_range(&range) {
            "false" => TokenKind::FalseKeyword,
            "true" => TokenKind::TrueKeyword,
            "void" => TokenKind::VoidKeyword,
            "new" => TokenKind::NewKeyword,
            _ => TokenKind::Identifier
        };
        Some(Token {
            range,
            kind
        })
    }

    fn parse(&mut self) -> Option<Token> {
        if self.input.is_eof() {
            return None;
        }
        let start = self.input.pos;
        let character = self.input.next()?;
        match character {
            ' ' | '\n' | '\r' => {
                if self.last_pos_before_spacing.is_none() {
                    self.last_pos_before_spacing = Some(start);
                }
                return self.parse()
            },
            _ => self.last_pos_before_spacing = None
        }
        let kind = 
        match character {
            '`' => TokenKind::StringLitStart,
            ',' => TokenKind::CommaPunc,
            ';' => TokenKind::SemicolonPunc,
            '[' => TokenKind::SquareBracketOpenPunc,
            ']' => TokenKind::SquareBracketClosePunc,
            '(' => TokenKind::ParanthesisOpenPunc,
            ')' => TokenKind::ParanthesisClosePunc,
            '+' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::PlusEqualsOp },
            '+' => TokenKind::PlusOp,
            '-' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::MinusEqualsOp},
            '-' => TokenKind::MinusOp,
            '*' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::StarEqualsOp},
            '*' if self.input.is_next(b'*', 0) => { self.input.skip_chars(1); TokenKind::StarStarOp},
            '*' => TokenKind::StarOp,
            '/' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::SlashEqualsOp},
            '/' => TokenKind::SlashOp,
            '%' => TokenKind::PercentOp,
            '=' if self.input.is_next(b'=', 0) && self.input.is_next(b'=', 1) => { self.input.skip_chars(2); TokenKind::EqualsEqualsEqualsOp },
            '=' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::EqualsEqualsOp },
            '=' => TokenKind::EqualsOp,
            '!' if self.input.is_next(b'=', 0) && self.input.is_next(b'=', 1) => { self.input.skip_chars(2); TokenKind::NotEqualsEqualsOp },
            '!' if self.input.is_next(b'=', 0) => { self.input.skip_chars(2); TokenKind::NotEqualsOp },
            '!' => TokenKind::ExclamationOp,
            '&' if self.input.is_next(b'&', 0) => { self.input.skip_chars(1); TokenKind::AmpersandAmpersandOp },
            '|' if self.input.is_next(b'|', 0) => { self.input.skip_chars(1); TokenKind::BarBarOp },
            '?' if self.input.is_next(b'?', 0) => { self.input.skip_chars(1); TokenKind::QuestionQuestionOp },
            '?' => TokenKind::QuestionOp,
            ':' => TokenKind::ColonOp,
            '.' if self.input.is_next(b'.', 0) && self.input.is_next(b'.', 1) => { self.input.skip_chars(2); TokenKind::DotDotDotOp },
            '.' => TokenKind::DotOp,
            '>' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::GreaterThanEqualsOp },
            '>' => TokenKind::GreaterThanOp,
            '<' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::LessThanEqualsOp },
            '<' => TokenKind::LessThanOp,
            '"' => return self.parse_string('"'),
            '\'' => return self.parse_string('\''),
            '0'..='9' => return self.parse_number(),
            'a'..='z' | 'A'..='Z' | '$' | '_' => return self.parse_identifier_or_keyword(),
            _ => return None
        };
        Some(Token {
            kind,
            range: self.input.range(start)
        })
    }

    pub fn consume(&mut self) -> Option<Token> {
        if self.last_token.is_some() {
            self.last_token.take()
        } else {
            self.parse()
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.last_token.is_some() {
            self.last_token.as_ref()
        } else {
            self.last_token = self.parse();
            self.last_token.as_ref()
        }
    }

    pub fn is_next(&mut self, kind: TokenKind) -> bool {
        if let Some(tok) = self.peek() {
            tok.kind == kind
        } else { 
            false
        }
    }

    pub fn expect(&mut self, kind: TokenKind, msg: &str) -> Option<Token> {
        if let Some(token) = self.consume() {
            if token.kind == kind {
                return Some(token);
            }
        }
        self.diagnostics.push(dia!(EXPECTED_TOKEN, self.input.range_here(), msg));
        None
    }

    pub fn pos(&self) -> usize {
        match self.last_token {
            Some(_) => self.input.pos - 1,
            None => self.input.pos
        }
    }

    pub fn range(&self, start: usize) -> Range<usize> {
        Range { start, end: self.last_pos_before_spacing.unwrap_or_else(|| self.pos()) }
    }

    pub fn is_eof(&self) -> bool {
        self.input.is_eof()
    }

    pub fn parse_full(content: &str) -> (Vec<Token>, InputPresenter, Vec<Diagnostic>) {
        let mut parser = Tokenizer::new(content);
        let mut res: Vec<Token> = vec![];
        while !parser.is_eof() {
            if let Some(tok) = parser.consume() {
                res.push(tok);
            }
        }
        (res, parser.input.data, parser.diagnostics)
    }


}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AmpersandAmpersandOp => write!(f, "&&"),
            Self::BarBarOp => write!(f, "||"),
            Self::ColonOp => write!(f, ":"),
            Self::CommaPunc => write!(f, ","),
            Self::DotDotDotOp => write!(f, "..."),
            Self::DotOp => write!(f, "."),
            Self::EqualsEqualsEqualsOp => write!(f, "==="),
            Self::EqualsEqualsOp => write!(f, "=="),
            Self::EqualsOp => write!(f, "="),
            Self::ExclamationOp => write!(f, "!"),
            Self::FalseKeyword => write!(f, "false"),
            Self::GreaterThanEqualsOp => write!(f, ">="),
            Self::GreaterThanOp => write!(f, ">"),
            Self::LessThanEqualsOp => write!(f, "<="),
            Self::LessThanOp => write!(f, "<"),
            Self::MinusEqualsOp => write!(f, "-="),
            Self::MinusOp => write!(f, "-"),
            Self::NewKeyword => write!(f, "new"),
            Self::NotEqualsEqualsOp => write!(f, "!=="),
            Self::NotEqualsOp => write!(f, "!="),
            Self::ParanthesisClosePunc => write!(f, ")"),
            Self::ParanthesisOpenPunc => write!(f, "("),
            Self::PercentOp => write!(f, "%"),
            Self::PlusEqualsOp => write!(f, "+="),
            Self::PlusOp => write!(f, "+"),
            Self::QuestionOp => write!(f, "?"),
            Self::QuestionQuestionOp => write!(f, "??"),
            Self::SemicolonPunc => write!(f, ";"),
            Self::SlashEqualsOp => write!(f, "/="),
            Self::SlashOp => write!(f, "/"),
            Self::SquareBracketClosePunc => write!(f, "]"),
            Self::SquareBracketOpenPunc => write!(f, "["),
            Self::StarEqualsOp => write!(f, "*="),
            Self::StarOp => write!(f, "*"),
            Self::StarStarOp => write!(f, "**"),
            Self::TrueKeyword => write!(f, "true"),
            Self::VoidKeyword => write!(f, "void"),
            _ => panic!("Token kind cannot be turned into a string.")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers() {
        let (result, range_reader, errors) = Tokenizer::parse_full("3.14 123 0x23 0b123 45_566_43");
        assert_eq!(range_reader.from_range(&result[0].range), "3.14");
        assert_eq!(range_reader.from_range(&result[1].range), "123");
        assert_eq!(range_reader.from_range(&result[2].range), "0x23");
        assert_eq!(errors.len(), 1);
        assert_eq!(range_reader.from_range(&result[4].range), "45_566_43");
    }

    #[test]
    fn test_strings() {
        let (result, range_reader, _) = Tokenizer::parse_full("
        \"Hello, World!\"
        'Test...'
        ");
        assert_eq!(range_reader.from_range(&result[0].range), "\"Hello, World!\"");
        assert_eq!(range_reader.from_range(&result[1].range), "'Test...'");
    }

    #[test]
    fn test_ops() {
        let (result, _, _) = Tokenizer::parse_full("
        \"Hello, World!\"
        ...
        +=
        ===
        !
        ");
        assert_eq!(result[0].kind, TokenKind::String);
        assert_eq!(result[1].kind, TokenKind::DotDotDotOp);
        assert_eq!(result[2].kind, TokenKind::PlusEqualsOp);
        assert_eq!(result[3].kind, TokenKind::EqualsEqualsEqualsOp);
        assert_eq!(result[4].kind, TokenKind::ExclamationOp);
    }

    #[test]
    fn test_identifiers_and_keywords() {
        let (result, range_reader, _) = Tokenizer::parse_full("
        hello_world HelloWorld
        helloWorld12d$
        void
        false
        true
        ");
        assert_eq!(result[0].kind, TokenKind::Identifier);
        assert_eq!(range_reader.from_range(&result[0].range), "hello_world");
        assert_eq!(result[1].kind, TokenKind::Identifier);
        assert_eq!(range_reader.from_range(&result[1].range), "HelloWorld");
        assert_eq!(result[2].kind, TokenKind::Identifier);
        assert_eq!(range_reader.from_range(&result[2].range), "helloWorld12d$");
        assert_eq!(result[3].kind, TokenKind::VoidKeyword);
        assert_eq!(result[4].kind, TokenKind::FalseKeyword);
        assert_eq!(result[5].kind, TokenKind::TrueKeyword);
    }

}