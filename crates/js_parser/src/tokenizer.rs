use storytell_diagnostics::{location::Range, diagnostic::*, make_diagnostics, dia};
use crate::input::{InputConsumer, InputPresenter};

make_diagnostics!(define [
    END_OF_STR,
    JSP1001,
    "Unexpected end of string."
], [
    INVALID_DIGIT,
    JSP1002,
    "Invalid digit '$'."
], [
    DECIMAL_POINT,
    JSP1003,
    "Number already has a decimal point."
]);

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    String,
    Number,
    Identifier,
    CommaPunc, //,
    SemicolonPunc, //;
    SquareBracketOpenPunc, // [
    SquareBracketClosePunc, // ]
    PranthesisOpenPunc, // (
    ParanthesisClosePunc, // )
    PlusOp, // +
    MinusOp, // -
    StarOp, // *
    SlashOp, // /
    PercentOp, // %
    EqualsEqualsEqualsOp, // ===
    EqualsEqualsOp, // ==
    EqualsOp,
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
    StringLitStart,
    VoidKeyword,
    TrueKeyword,
    FalseKeyword,
    NewKeyword
}

pub struct Token {
    pub kind: TokenKind,
    pub range: Range<usize>
}

#[derive(PartialEq)]
pub enum NumberType {
    Binary, // 0b
    Octal, // 0o
    Hex, // 0x
    None
}

pub struct Tokenizer<'a> {
    pub input: InputConsumer<'a>,
    pub last_token: Option<Token>,
    pub errors: Vec<Diagnostic>
}

impl<'a> Tokenizer<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            input: InputConsumer::new(content),
            last_token: None,
            errors: vec![]
        }
    }

    pub fn parse_string(&mut self, end_char: char) -> Option<Token> {
        let start = self.input.pos - 1;
        loop {
            match self.input.next() {
                Some(character) if character == end_char => break,
                None => {
                    self.errors.push(dia!(END_OF_STR, self.input.range(start)))
                },
                _ => {}
            }
        }
        Some(Token {
            kind: TokenKind::String,
            range: self.input.range(start)
        })
    }

    pub fn parse_number(&mut self) -> Option<Token> {
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
                            self.errors.push(dia!(INVALID_DIGIT, self.input.range_here(), &ch.to_string()));
                            number_type = NumberType::None;
                        },
                        NumberType::Octal if ch > '7' => {
                            self.errors.push(dia!(INVALID_DIGIT, self.input.range_here(), &ch.to_string()));
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
                        self.errors.push(dia!(DECIMAL_POINT, self.input.range_here()));
                        break;
                    };
                    has_dot = true;
                },
                '_' => self.input.skip_chars(1),
                _ => break
            }
        };
        Some(Token {
            kind: TokenKind::Number,
            range: self.input.range(start)
        })
    }

    fn consume(&mut self) -> Option<Token> {
        if self.input.is_eof() {
            return None;
        }
        let start = self.input.pos;
        let kind = 
        match self.input.next()? {
            '`' => TokenKind::StringLitStart,
            ',' => TokenKind::CommaPunc,
            ';' => TokenKind::SemicolonPunc,
            '[' => TokenKind::SquareBracketOpenPunc,
            ']' => TokenKind::SquareBracketClosePunc,
            '(' => TokenKind::PranthesisOpenPunc,
            ')' => TokenKind::ParanthesisClosePunc,
            '+' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::PlusEqualsOp },
            '+' => TokenKind::PlusOp,
            '-' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::MinusEqualsOp},
            '-' => TokenKind::MinusOp,
            '*' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::StarEqualsOp},
            '*' => TokenKind::StarOp,
            '/' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::SlashEqualsOp},
            '/' => TokenKind::SlashOp,
            '%' => TokenKind::PercentOp,
            '=' if self.input.is_next(b'=', 0) && self.input.is_next(b'=', 1) => { self.input.skip_chars(2); TokenKind::EqualsEqualsEqualsOp },
            '=' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::EqualsEqualsOp },
            '=' => TokenKind::EqualsOp,
            '!' => TokenKind::ExclamationOp,
            '.' if self.input.is_next(b'.', 0) && self.input.is_next(b'.', 1) => { self.input.skip_chars(2); TokenKind::DotDotDotOp },
            '.' => TokenKind::DotOp,
            '>' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::GreaterThanEqualsOp },
            '>' => TokenKind::GreaterThanOp,
            '<' if self.input.is_next(b'=', 0) => { self.input.skip_chars(1); TokenKind::LessThanEqualsOp },
            '<' => TokenKind::LessThanOp,
            '"' => return self.parse_string('"'),
            '\'' => return self.parse_string('\''),
            '0'..='9' => return self.parse_number(),
            _ => return None
        };
        Some(Token {
            kind,
            range: self.input.range(start)
        })
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.last_token.is_some() {
            self.last_token.take()
        } else {
            self.consume()
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.last_token.is_some() {
            self.last_token.as_ref()
        } else {
            self.last_token = self.consume();
            self.last_token.as_ref()
        }
    }

    pub fn is_eof(&self) -> bool {
        self.input.is_eof()
    }

    pub fn parse_full<'b>(content: &'b str) -> (Vec<Token>, InputPresenter<'b>, Vec<Diagnostic>) {
        let mut parser = Tokenizer::new(content);
        let mut res: Vec<Token> = vec![];
        while !parser.is_eof() {
            if let Some(tok) = parser.next() {
                res.push(tok);
            }
        }
        (res, parser.input.data, parser.errors)
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

}