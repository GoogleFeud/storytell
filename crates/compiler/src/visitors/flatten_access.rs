use storytell_js_parser::{ast::{ASTAccess, ASTExpression, ASTAccessContent}, input::InputPresenter};

pub fn get_string_from_accessor<'a>(input: &InputPresenter<'a>, accessor: &ASTAccessContent) -> Option<&str> {
    match accessor {
        ASTAccessContent::Identifier(ident) => Some(input.from_range(&ident.range)),
        ASTAccessContent::Expression(exp) => {
            match exp {
                ASTExpression::String(str) => Some(input.from_range(&str.range)),
                ASTExpression::Number(num) => Some(input.from_range(&num.range)),
                _ => None
            }
        }
    }
}

pub fn flatten_access<'a>(input: &InputPresenter<'a>, access: &ASTAccess) -> Option<Vec<String>> {
    let mut result = vec![];
    let mut left = access;
    while let ASTExpression::Access(acc) = left {
        left = &acc.expression;
        result.push(get_string_from_accessor(input, &acc.accessor)?);
    }
    result
}