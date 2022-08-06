
use storytell_diagnostics::location::Range;
use crate::tokenizer::TokenKind;

pub trait Visitable {
    fn visit<T: Visitor + ?Sized>(&self, visitor: &mut T);
    fn visit_each_child<T: Visitor + ?Sized>(&self, visitor: &mut T);
}

pub trait MutVisitable<Item> {
    fn visit_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Item;
    fn visit_each_child_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Item;
}

macro_rules! create_nodes {
    ($([$name: ident, $type: ident {$($field_name: ident: $field_type: ty),*}, {$($child_name: ident: $child_type: ty [$child_type_name: ident]),*}]),+) => {
        pub trait Visitor {
            fn expression(&mut self, exp: &ASTExpression) {
                exp.visit(self)
            }
            fn optional_expression(&mut self, exp: &Option<ASTExpression>) {
                if let Some(exp) = exp { self.expression(exp) }
            }
            $(fn $type(&mut self, _exp: &$name) {})+
        }

        pub trait MutVisitor {
            fn expression(&mut self, exp: &ASTExpression) -> ASTExpression {
               exp.visit_mut(self)
            }
            fn optional_expression(&mut self, exp: &Option<ASTExpression>) -> Option<ASTExpression> {
                exp.as_ref().map(|exp| self.expression(exp))
            }
            $(fn $type(&mut self, exp: &$name) -> $name { exp.clone() })+
        }

        $(
            #[derive(Clone, Debug)]
            pub struct $name {
                $(pub $field_name: $field_type,)*
                $(pub $child_name: $child_type,)*
                pub range: Range<usize>
            }

            impl Visitable for $name {
                fn visit<T: Visitor + ?Sized>(&self, visitor: &mut T) {
                    visitor.$type(self);
                }
                
                fn visit_each_child<T: Visitor + ?Sized>(&self, _visitor: &mut T) {
                    $(
                        _visitor.$child_type_name(&self.$child_name);   
                    )*
                }
            }

            impl MutVisitable<$name> for $name {
                fn visit_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Self {
                    visitor.$type(self)
                }

                fn visit_each_child_mut<T: MutVisitor + ?Sized>(&self, _visitor: &mut T) -> Self {
                        $name {
                            $($field_name: self.$field_name.clone(),)*
                            $($child_name: _visitor.$child_type_name(&self.$child_name),)*
                            range: self.range.clone()
                        }
                }
            }

        )+
    }
}

create_nodes!([
    ASTString, string {}, {}
], [
    ASTNumber, number {}, {}
], [
    ASTBoolean, boolean {}, {}
], [
    ASTIdentifier, identifier {}, {}
], [
    ASTBinary, binary {
        operator: TokenKind
    }, {
        left: ASTExpression [expression],
        right: ASTExpression [expression]
    }
], [
    ASTUnary, unary {
        operator: TokenKind
    }, {
        expression: ASTExpression [expression]
    }
]);

#[derive(Clone, Debug)]
pub enum ASTExpression {
    String(ASTString),
    Number(ASTNumber),
    Boolean(ASTBoolean),
    Identifier(ASTIdentifier),
    Binary(Box<ASTBinary>),
    Unary(Box<ASTUnary>)
}

impl Visitable for ASTExpression {
    fn visit<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        match self {
            Self::String(str) => visitor.string(str),
            Self::Number(num) => visitor.number(num),
            Self::Boolean(bool) => visitor.boolean(bool),
            Self::Identifier(ident) => visitor.identifier(ident),
            Self::Binary(binary) => visitor.binary(binary),
            Self::Unary(unary) => visitor.unary(unary),
        }
    }

    fn visit_each_child<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        Visitable::visit(self, visitor)
    }
    
}

impl MutVisitable<ASTExpression> for ASTExpression {
    fn visit_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> ASTExpression {
        match self {
            Self::String(str) => ASTExpression::String(visitor.string(str)),
            Self::Number(num) => ASTExpression::Number(visitor.number(num)),
            Self::Boolean(bool) => ASTExpression::Boolean(visitor.boolean(bool)),
            Self::Identifier(ident) => ASTExpression::Identifier(visitor.identifier(ident)),
            Self::Binary(binary) => ASTExpression::Binary(Box::from(visitor.binary(binary))),
            Self::Unary(unary) => ASTExpression::Unary(Box::from(visitor.unary(unary))),
        }
    }

    fn visit_each_child_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> ASTExpression {
        self.visit_mut(visitor)
    }
}

impl ASTExpression {
    pub fn range(&self) -> &Range<usize> {
        match self {
            Self::String(str) => &str.range,
            Self::Number(num) => &num.range,
            Self::Boolean(bool) => &bool.range,
            Self::Identifier(ident) => &ident.range,
            Self::Binary(binary) => &binary.range,
            Self::Unary(unary) => &unary.range
        }
    }
}

