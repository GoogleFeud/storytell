
use storytell_diagnostics::location::Range;
use crate::tokenizer::TokenKind;

pub trait Visitable {
    fn visit<T: Visitor + ?Sized>(&self, visitor: &mut T);
    fn visit_each_child<T: Visitor + ?Sized>(&self, visitor: &mut T);
}

pub trait MutVisitable<Item>: Clone {
    fn visit_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Item;
    fn visit_each_child_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Item;
}

macro_rules! create_nodes {
    ($([$name: ident, $type: ident {$($field_name: ident: $field_type: ty),*}, {$($child_name: ident: $child_type: ty [$child_type_name: ident]),*}]),+) => {
        pub trait Visitor {
            fn expression(&mut self, exp: &ASTExpression) {
                exp.visit_inner(self)
            }
            fn optional_expression(&mut self, exp: &Option<ASTExpression>) {
                if let Some(exp) = exp { exp.visit_inner(self) }
            }
            fn list<T: Visitable>(&mut self, _exp: &Vec<T>) {}
            $(fn $type(&mut self, _exp: &$name) {})+
        }

        pub trait MutVisitor {
            fn expression(&mut self, exp: &ASTExpression) -> ASTExpression {
               exp.visit_inner_mut(self)
            }
            fn optional_expression(&mut self, exp: &Option<ASTExpression>) -> Option<ASTExpression> {
                exp.as_ref().map(|exp| exp.visit_inner_mut(self))
            }
            fn list<M, T: MutVisitable<M> + Clone>(&mut self, exp: &Vec<T>) -> Vec<T> { exp.to_vec() }
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
    ASTArray, array {}, {
        elements: Vec<ASTExpression> [list]
    }
], [
    ASTCall, call {}, {
        expression: ASTExpression [expression],
        arguments: Vec<ASTExpression> [list]
    }
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
], [
    ASTAccess, access {
        accessor: ASTAccessContent
    }, {
        expression: ASTExpression [expression]
    }
], [
    ASTNew, new_exp {}, {
        arguments: Vec<ASTExpression> [list],
        expression: ASTExpression [expression]
    }
], [
    ASTTernary, ternary {}, {
        condition: ASTExpression [expression],
        left: ASTExpression [expression],
        right: ASTExpression [expression]
    }
], [
    ASTStringLiteralPart, string_literal_part {
        before: String
    }, {
        expression: ASTExpression [expression]
    }
], [
    ASTStringLiteral, string_literal {
        start: String
    }, {
        spans: Vec<ASTStringLiteralPart> [list]
    }
]);

#[derive(Clone, Debug)]
pub enum ASTAccessContent {
    Identifier(ASTIdentifier),
    Expression(ASTExpression)
}

impl ASTAccessContent {
    pub fn range(&self) -> &Range<usize> {
        match self {
            Self::Identifier(ident) => &ident.range,
            Self::Expression(exp) => exp.range()
        }
    }
}

impl Visitable for ASTAccessContent {
    fn visit<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        match self {
            Self::Identifier(ident) => ident.visit(visitor),
            Self::Expression(exp) => exp.visit(visitor)
        }
    }

    fn visit_each_child<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        match self {
            Self::Identifier(ident) => ident.visit_each_child(visitor),
            Self::Expression(exp) => exp.visit_each_child(visitor)
        }
    }
}

impl MutVisitable<ASTAccessContent> for ASTAccessContent {
    fn visit_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Self {
        match self {
            Self::Identifier(ident) => Self::Identifier(ident.visit_mut(visitor)),
            Self::Expression(exp) => Self::Expression(exp.visit_mut(visitor))
        }
    }

    fn visit_each_child_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Self {
        match self {
            Self::Identifier(ident) => Self::Identifier(ident.visit_each_child_mut(visitor)),
            Self::Expression(exp) => Self::Expression(exp.visit_each_child_mut(visitor))
        }
    }
}

#[derive(Clone, Debug)]
pub enum ASTExpression {
    String(ASTString),
    Number(ASTNumber),
    Boolean(ASTBoolean),
    Identifier(ASTIdentifier),
    Binary(Box<ASTBinary>),
    Unary(Box<ASTUnary>),
    Call(Box<ASTCall>),
    ArrayLit(ASTArray),
    Access(Box<ASTAccess>),
    New(Box<ASTNew>),
    Ternary(Box<ASTTernary>)
}

impl ASTExpression {
    pub fn visit_inner<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        match self {
            Self::String(str) => str.visit(visitor),
            Self::Number(num) => num.visit(visitor),
            Self::Boolean(bool) => bool.visit(visitor),
            Self::Identifier(ident) => ident.visit(visitor),
            Self::Binary(binary) => binary.visit(visitor),
            Self::Unary(unary) => unary.visit(visitor),
            Self::Call(call) => call.visit(visitor),
            Self::ArrayLit(arr) => arr.visit(visitor),
            Self::Access(access) => access.visit(visitor),
            Self::New(new) => new.visit(visitor),
            Self::Ternary(ternary) => ternary.visit(visitor)
        }
    }

    pub fn visit_inner_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> ASTExpression {
        match self {
            Self::String(str) => ASTExpression::String(str.visit_mut(visitor)),
            Self::Number(num) => ASTExpression::Number(num.visit_mut(visitor)),
            Self::Boolean(bool) => ASTExpression::Boolean(bool.visit_mut(visitor)),
            Self::Identifier(ident) => ASTExpression::Identifier(ident.visit_mut(visitor)),
            Self::Binary(binary) => ASTExpression::Binary(Box::from(binary.visit_mut(visitor))),
            Self::Unary(unary) => ASTExpression::Unary(Box::from(unary.visit_mut(visitor))),
            Self::Call(call) => ASTExpression::Call(Box::from(call.visit_mut(visitor))),
            Self::ArrayLit(arr) => ASTExpression::ArrayLit(arr.visit_mut(visitor)),
            Self::Access(access) => ASTExpression::Access(Box::from(access.visit_mut(visitor))),
            Self::New(new) => ASTExpression::New(Box::from(new.visit_mut(visitor))),
            Self::Ternary(ternary) => ASTExpression::Ternary(Box::from(ternary.visit_mut(visitor)))
        }
    }
}

impl Visitable for ASTExpression {
    fn visit<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        visitor.expression(self);
    }

    fn visit_each_child<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        match self {
            Self::String(str) => str.visit_each_child(visitor),
            Self::Number(num) => num.visit_each_child(visitor),
            Self::Boolean(bool) => bool.visit_each_child(visitor),
            Self::Identifier(ident) => ident.visit_each_child(visitor),
            Self::Binary(binary) => binary.visit_each_child(visitor),
            Self::Unary(unary) => unary.visit_each_child(visitor),
            Self::Call(call) => call.visit_each_child(visitor),
            Self::ArrayLit(arr) => arr.visit_each_child(visitor),
            Self::Access(access) => access.visit_each_child(visitor),
            Self::New(new) => new.visit_each_child(visitor),
            Self::Ternary(ternary) => ternary.visit_each_child(visitor)
        }
    }
    
}

impl MutVisitable<ASTExpression> for ASTExpression {
    fn visit_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> ASTExpression {
        visitor.expression(self)
    }

    fn visit_each_child_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> ASTExpression {
        match self {
            Self::String(str) => ASTExpression::String(str.visit_each_child_mut(visitor)),
            Self::Number(num) => ASTExpression::Number(num.visit_each_child_mut(visitor)),
            Self::Boolean(bool) => ASTExpression::Boolean(bool.visit_each_child_mut(visitor)),
            Self::Identifier(ident) => ASTExpression::Identifier(ident.visit_each_child_mut(visitor)),
            Self::Binary(binary) => ASTExpression::Binary(Box::from(binary.visit_each_child_mut(visitor))),
            Self::Unary(unary) => ASTExpression::Unary(Box::from(unary.visit_each_child_mut(visitor))),
            Self::Call(call) => ASTExpression::Call(Box::from(call.visit_each_child_mut(visitor))),
            Self::ArrayLit(arr) => ASTExpression::ArrayLit(arr.visit_each_child_mut(visitor)),
            Self::Access(access) => ASTExpression::Access(Box::from(access.visit_each_child_mut(visitor))),
            Self::New(new) => ASTExpression::New(Box::from(new.visit_each_child_mut(visitor))),
            Self::Ternary(ternary) => ASTExpression::Ternary(Box::from(ternary.visit_each_child_mut(visitor)))
        }
    }
}

impl<Item: Visitable> Visitable for Vec<Item> {
    fn visit<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        visitor.list(self)
    }

    fn visit_each_child<T: Visitor + ?Sized>(&self, visitor: &mut T) {
        for element in self {
            element.visit(visitor)
        }
    }
}

impl<Item: MutVisitable<Item>> MutVisitable<Vec<Item>> for Vec<Item> {
    fn visit_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Vec<Item> {
        visitor.list(self)
    }

    fn visit_each_child_mut<T: MutVisitor + ?Sized>(&self, visitor: &mut T) -> Vec<Item> {
        self.iter().map(|e| e.visit_mut(visitor)).collect()
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
            Self::Unary(unary) => &unary.range,
            Self::Call(thing) => &thing.range,
            Self::ArrayLit(thing) => &thing.range,
            Self::Access(thing) => &thing.range,
            Self::New(thing) => &thing.range,
            Self::Ternary(thing) => &thing.range,
        }
    }
}

