
use crate::{tokenizer::{Tokenizer, TokenKind}, input::InputPresenter};
use storytell_diagnostics::{diagnostic::*, *};
use self::ast::*;
pub mod ast;

make_diagnostics!(define [
    UNKNOWN_TOKEN,
    JSP2001,
    "Unknown token $."
], [
    EXPECTED,
    JSP2002,
    "Expected $."
]);

pub struct JsParser<'a> {
    pub tokens: Tokenizer<'a>
}

impl<'a> JsParser<'a> {

    pub fn new(content: &'a str) -> Self {
        Self { 
            tokens: Tokenizer::new(content)
        }
    }

    pub fn resolve_prec(token: &TokenKind) -> u8 {
        match token {
            TokenKind::StarStarOp => 13,
            TokenKind::StarOp | TokenKind::SlashOp | TokenKind::PercentOp => 12,
            TokenKind::PlusOp | TokenKind::MinusOp => 11,
            TokenKind::LessThanOp | TokenKind::LessThanEqualsOp | TokenKind::GreaterThanOp | TokenKind::GreaterThanEqualsOp => 9,
            TokenKind::EqualsEqualsOp | TokenKind::EqualsEqualsEqualsOp | TokenKind::NotEqualsOp | TokenKind::NotEqualsEqualsOp => 8,
            TokenKind::AmpersandAmpersandOp => 4,
            TokenKind::BarBarOp | TokenKind::QuestionQuestionOp => 3,
            TokenKind::PlusEqualsOp | TokenKind::MinusEqualsOp | TokenKind::SlashEqualsOp | TokenKind::StarEqualsOp | TokenKind::EqualsOp => 2,
            _ => 0
        }
    }

    fn parse_binary(&mut self, left: ASTExpression, left_prec: u8) -> Option<ASTExpression> {
        let start = left.range().start;
        if let Some(next) = self.tokens.peek() {
            let right_prec = Self::resolve_prec(&next.kind);
            if right_prec == 0 {
                return Some(left);
            };
            if right_prec > left_prec {
                let op_token = self.tokens.consume().unwrap();
                let exp = if let Some(exp) = self.parse_single_expression(true) {
                    exp
                } else { 
                    return Some(left)
                };
                let right = self.parse_binary(exp, right_prec)?;
                self.parse_binary(ASTExpression::Binary(Box::from(ASTBinary {
                    left,
                    right,
                    operator: op_token.kind,
                    range: self.tokens.range(start)
                })), left_prec)
            } else {
                Some(left)
            }
        } else {
            Some(left)
        }
    }

    fn parse_list<T>(&mut self, separator: TokenKind, end_token: TokenKind, end_token_str: &str, parse_fn: fn(thing: &mut Self) -> Option<T>) -> Vec<T> {
        let mut expressions: Vec<T> = vec![];
        let mut is_first = true;
        while !self.tokens.is_next(end_token.clone()) {
            if !is_first {
                self.tokens.expect(separator.clone(), ",");
            };
            if let Some(exp) = parse_fn(self) {
                expressions.push(exp);
            } else {
                break;
            }
            is_first = false;
        };
        self.tokens.expect(end_token, end_token_str);
        expressions
    }
    
    fn parse_suffix(&mut self, tok: ASTExpression, start: usize) -> Option<ASTExpression> {
        let token = if let Some(token) = self.tokens.peek() { token } else {
            return Some(tok)
        };
        match token.kind {
            TokenKind::ParanthesisOpenPunc => {
                self.tokens.consume();
                let arguments = self.parse_list(TokenKind::CommaPunc, TokenKind::ParanthesisClosePunc, ")", |parser| parser.parse_full_expression());            
                self.parse_suffix(ASTExpression::Call(Box::from(ASTCall {
                    expression: tok,
                    arguments,
                    range: self.tokens.range(start)
                })), start)
            },
            TokenKind::DotOp => {
                self.tokens.consume();
                let ident = self.tokens.expect(TokenKind::Identifier, "identifier")?;
                self.parse_suffix(ASTExpression::Access(Box::from(ASTAccess {
                    accessor: ASTAccessContent::Identifier(ASTIdentifier { range: ident.range }),
                    expression: tok,
                    range: self.tokens.range(start)
                })), start)
            },
            TokenKind::SquareBracketOpenPunc => {
                self.tokens.consume();
                let exp = self.parse_full_expression()?;
                self.tokens.expect(TokenKind::SquareBracketClosePunc, "]")?;
                self.parse_suffix(ASTExpression::Access(Box::from(ASTAccess {
                    accessor: ASTAccessContent::Expression(exp),
                    expression: tok,
                    range: self.tokens.range(start)
                })), start)
            },
            TokenKind::QuestionOp => {
                self.tokens.consume();
                let left = self.parse_full_expression()?;
                self.tokens.expect(TokenKind::ColonOp, "colon")?;
                Some(ASTExpression::Ternary(Box::from(ASTTernary {
                    condition: tok,
                    left,
                    right: self.parse_full_expression()?,
                    range: self.tokens.range(start)
                })))
            },
            _ => Some(tok)
        }
    }

    fn parse_single_expression(&mut self, parse_suffix: bool) -> Option<ASTExpression> {
        let token = self.tokens.consume()?;
        let tok_start = token.range.start;
        let ast_token = match token.kind {
            TokenKind::String => ASTExpression::String(ASTString { range: token.range }),
            TokenKind::Number => ASTExpression::Number(ASTNumber { range: token.range }),
            TokenKind::Identifier => ASTExpression::Identifier(ASTIdentifier { range: token.range }),
            TokenKind::FalseKeyword | TokenKind::TrueKeyword => ASTExpression::Boolean(ASTBoolean { range: token.range }),
            TokenKind::ExclamationOp => ASTExpression::Unary(Box::from(ASTUnary {
                operator: TokenKind::ExclamationOp,
                expression: self.expect_single_expr("an expression", true)?,
                range: self.tokens.range(tok_start)
            })),
            TokenKind::PlusOp => ASTExpression::Unary(Box::from(ASTUnary {
                operator: TokenKind::PlusOp,
                expression: self.expect_single_expr("an expression", true)?,
                range: self.tokens.range(tok_start)
            })),
            TokenKind::MinusOp => ASTExpression::Unary(Box::from(ASTUnary {
                operator: TokenKind::MinusOp,
                expression: self.expect_single_expr("an expression", true)?,
                range: self.tokens.range(tok_start)
            })),
            TokenKind::DotDotDotOp => ASTExpression::Unary(Box::from(ASTUnary {
                operator: TokenKind::DotDotDotOp,
                expression: self.expect_single_expr("an expression", true)?,
                range: self.tokens.range(tok_start)
            })),
            TokenKind::VoidKeyword => ASTExpression::Unary(Box::from(ASTUnary {
                operator: TokenKind::VoidKeyword,
                expression: self.expect_single_expr("an expression", true)?,
                range: self.tokens.range(tok_start)
            })),
            TokenKind::SquareBracketOpenPunc => {
                let elements = self.parse_list(TokenKind::CommaPunc, TokenKind::SquareBracketClosePunc, "]", |parser| parser.parse_full_expression());
                ASTExpression::ArrayLit(ASTArray {
                    elements,
                    range: self.tokens.range(tok_start)
                })
            },
            TokenKind::ParanthesisOpenPunc => {
                let exp = self.parse_full_expression()?;
                self.tokens.expect(TokenKind::ParanthesisClosePunc, ")")?;
                exp
            },
            TokenKind::NewKeyword => {
                let expression = self.parse_single_expression(false)?;
                let arguments = if self.tokens.is_next(TokenKind::ParanthesisOpenPunc) {
                    self.tokens.consume();
                    self.parse_list(TokenKind::CommaPunc, TokenKind::ParanthesisClosePunc, ")", |parser| parser.parse_full_expression())
                } else {
                    vec![]
                };
                ASTExpression::New(Box::from(ASTNew {
                    expression,
                    arguments,
                    range: self.tokens.range(tok_start)
                }))
            },
            _ => {
                self.tokens.diagnostics.push(dia!(UNKNOWN_TOKEN, token.range, self.tokens.input.data.from_range(&token.range)));
                return None
            }
        };
        if parse_suffix {
            self.parse_suffix(ast_token, tok_start)
        } else {
            Some(ast_token)
        }
    }

    fn parse_full_expression(&mut self) -> Option<ASTExpression> {
        if let Some(exp) = self.parse_single_expression(true) {
            self.parse_binary(exp, 0)
        } else {
            None
        }
    }

    fn expect_single_expr(&mut self, msg: &str, parse_suffix: bool) -> Option<ASTExpression> {
        let expr = self.parse_single_expression(parse_suffix);
        if expr.is_none() {
            self.tokens.diagnostics.push(dia!(EXPECTED, self.tokens.input.range_here(), msg));
        }
        expr
    }

    pub fn parse(content: &'a str) -> (Vec<ASTExpression>, Vec<Diagnostic>, InputPresenter<'a>) {
        let mut parser = JsParser::new(content, );
        let mut result: Vec<ASTExpression> = vec![];
        while let Some(exp) = parser.parse_full_expression() {
            result.push(exp);
            if parser.tokens.is_next(TokenKind::SemicolonPunc) {
                parser.tokens.consume();
            } else {
                break;
            }
        }
        (result, parser.tokens.diagnostics, parser.tokens.input.data)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let (tokens, _, input) = JsParser::parse("\"HelloWorld\"; 123.4; false");
        assert_eq!(input.from_range(tokens[0].range()), "\"HelloWorld\"");
        assert_eq!(input.from_range(tokens[1].range()), "123.4");
        assert_eq!(input.from_range(tokens[2].range()), "false");
    }

    pub struct MyVisitor<'a> {
        occurance: u8,
        input: InputPresenter<'a>
    }

    impl<'a> Visitor for MyVisitor<'a> {
        fn binary(&mut self, exp: &ASTBinary) {
            println!("THING: {:?}", self.input.from_range(&exp.range));
            if self.occurance == 0 {
                assert_eq!(exp.operator, TokenKind::MinusOp);
            }
            if self.occurance == 1 {
                assert_eq!(self.input.from_range(exp.left.range()), "a");
                assert_eq!(self.input.from_range(exp.right.range()), "b");
            }
            if self.occurance == 2 {
                assert_eq!(self.input.from_range(exp.right.range()), "c");
                assert_eq!(exp.operator, TokenKind::StarOp);
            }
            if self.occurance == 3 {
                assert_eq!(self.input.from_range(exp.left.range()), "c");
                assert_eq!(self.input.from_range(exp.right.range()), "d");
            }
            self.occurance += 1;
            exp.visit_each_child(self);
        }

    }

    #[test]
    fn test_binary_prec() {
        let (tokens, errors, input) = JsParser::parse("
            a + b - c / d * c
        ");
        assert_eq!(errors.len(), 0);
        let mut visitor = MyVisitor { input, occurance: 0 };
        tokens[0].visit(&mut visitor);
    }

    #[test]
    fn test_unary() {
        let (tokens, errors, input) = JsParser::parse("
            !a && b
       ");
        assert_eq!(errors.len(), 0);
        if let ASTExpression::Binary(expr) = &tokens[0] {
            if let ASTExpression::Unary(unary) = &expr.left {
                assert_eq!(input.from_range(unary.expression.range()), "a");
            } else {
                panic!("Expected unary expression.");
            }
        } else {
            panic!("Expected binary expression.")
        }
    }

    #[test]
    fn test_call() {
        let (tokens, errors, input) = JsParser::parse("
            test(test(), test123(3 + 2))
       ");
        assert_eq!(errors.len(), 0);
        if let ASTExpression::Call(expr) = &tokens[0] {
            assert_eq!(expr.arguments.len(), 2);
            assert_eq!(input.from_range(expr.expression.range()), "test");
            if let ASTExpression::Call(expr) = &expr.arguments[0] {
                assert_eq!(expr.arguments.len(), 0);
                assert_eq!(input.from_range(expr.expression.range()), "test");
            } else {
                panic!("Expected call.")
            }
            if let ASTExpression::Call(expr) = &expr.arguments[1] {
                assert_eq!(expr.arguments.len(), 1);
                assert_eq!(input.from_range(expr.expression.range()), "test123");
                if let ASTExpression::Binary(expr) = &expr.arguments[0] {
                    assert_eq!(input.from_range(&expr.range), "3 + 2");
                } else {
                    panic!("Expected binary.")
                }
            } else {
                panic!("Expected call.")
            }
        } else {
            panic!("Expected call.")
        }
    }

    #[test]
    fn test_array() {
        let (tokens, errors, input) = JsParser::parse("
            [1, 2, 3, [4, 5], [6], []]
       ");
        assert_eq!(errors.len(), 0);
        if let ASTExpression::ArrayLit(expr) = &tokens[0] {
            println!("{:?}", expr.elements);
            assert_eq!(expr.elements.len(), 6);
            assert_eq!(input.from_range(&expr.elements[0].range()), "1");
            assert_eq!(input.from_range(&expr.elements[1].range()), "2");
            assert_eq!(input.from_range(&expr.elements[2].range()), "3");
            if let ASTExpression::ArrayLit(expr) = &expr.elements[3] {
                assert_eq!(expr.elements.len(), 2);
            } else {
                panic!("Expected array.")
            }
        } else {
            panic!("Expected array.")
        }
    }

    #[test]
    fn test_dot_access() {
        let (tokens, errors, input) = JsParser::parse("
            a().b.c.d;
       ");
        assert_eq!(errors.len(), 0);
        if let ASTExpression::Access(acc) = &tokens[0] {
            assert_eq!(input.from_range(acc.accessor.range()), "d");
            if let ASTExpression::Access(acc) = &acc.expression {
                assert_eq!(input.from_range(acc.accessor.range()), "c");
                if let ASTExpression::Access(acc) = &acc.expression {
                    assert_eq!(input.from_range(acc.accessor.range()), "b");
                    assert!(matches!(acc.expression, ASTExpression::Call(_)));
                } else {
                    panic!("Expected access.");
                }
            } else {
                panic!("Expected access.");
            }
        } else {
            panic!("Expected access.")
        }
    }

    #[test]
    fn test_exp_access() {
        let (tokens, errors, input) = JsParser::parse("
            (1 + 2).c[d][e].z;
       ");
        assert_eq!(errors.len(), 0);
        if let ASTExpression::Access(acc) = &tokens[0] {
            assert_eq!(input.from_range(acc.accessor.range()), "z");
            if let ASTExpression::Access(acc) = &acc.expression {
                assert_eq!(input.from_range(acc.accessor.range()), "e");
                if let ASTExpression::Access(acc) = &acc.expression {
                    assert_eq!(input.from_range(acc.accessor.range()), "d");
                    if let ASTExpression::Access(acc) = &acc.expression {
                        assert_eq!(input.from_range(acc.accessor.range()), "c");
                        assert!(matches!(acc.expression, ASTExpression::Binary(_)));
                    } else {
                        panic!("Expected access.");
                    }
                } else {
                    panic!("Expected access.");
                }
            } else {
                panic!("Expected access.");
            }
        } else {
            panic!("Expected access.")
        }
    }

    #[test]
    fn test_new() {
        let (tokens, errors, input) = JsParser::parse("
            new Something(new Other)
       ");
        assert_eq!(errors.len(), 0);
        if let ASTExpression::New(new) = &tokens[0] {
            assert_eq!(input.from_range(new.expression.range()), "Something");
            if let ASTExpression::New(new) = &new.arguments[0] {
                assert_eq!(new.arguments.len(), 0);
                assert_eq!(input.from_range(new.expression.range()), "Other")
            } else {
                panic!("Expected new.")
            }
        } else {
            panic!("Expected new.")
        }
    }

    #[test]
    fn test_ternary() {
        let (tokens, errors, input) = JsParser::parse("
            a ? b : (c + k) ? d : e;
       ");
        assert_eq!(errors.len(), 0);
        if let ASTExpression::Ternary(exp) = &tokens[0] {
            assert_eq!(input.from_range(exp.condition.range()), "a");
            assert_eq!(input.from_range(exp.left.range()), "b");
            if let ASTExpression::Ternary(exp) = &exp.right {
                assert_eq!(input.from_range(exp.condition.range()), "c + k");
                assert_eq!(input.from_range(exp.left.range()), "d");
                assert_eq!(input.from_range(exp.right.range()), "e");
            } else {
                panic!("Expected ternary.")
            }
        } else {
            panic!("Expected ternary.")
        }
    }
}