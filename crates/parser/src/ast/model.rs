
use storytell_diagnostics::location::*;
use std::fmt;

macro_rules! create_nodes {
    ($($name: ident {$($field_name: ident: $field_type: ty),*})+) => {
        $(
            #[derive(Clone)]
            pub struct $name {
                $(pub $field_name: $field_type,)*
                pub attributes: Vec<ASTAttribute>,
                pub range: Range<usize>
            }

            impl fmt::Debug for $name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_struct(stringify!($name))
                     $(.field(stringify!($field_name), &self.$field_name))*
                     .finish()
                }
            }
        )+
    };
}

#[derive(Clone, Debug, PartialEq)]
pub enum ASTInlineKind {
    // **...**
    Bold,
    // { ... }
    Javascript,
    // *...*
    Italics,
    // __...__
    Underline,
    // `...`
    Code,
    // ->
    Divert,
    // <->
    TempDivert
}

#[derive(Clone, Debug)]
pub enum MatchKind {
    If,
    Not,
    Default
}

#[derive(Clone, Debug)]
pub enum ASTAttributeKind {
    Once,
    Exaust
}

#[derive(Clone, Debug)]
pub struct TextPart {
    pub before: String,
    pub text: ASTInline
}

create_nodes!(
    ASTInline {
        kind: ASTInlineKind,
        text: ASTText
    }

    ASTText {
        parts: Vec<TextPart>,
        tail: String
    }

    ASTAttribute {
        kind: ASTAttributeKind
    }

    ASTCodeBlock {
        language: String,
        text: String
    }

    ASTChoice {
        text: String,
        divert: Option<String>,
        children: Option<ASTBlock>,
        is_temp: bool
    }

    ASTChoiceGroup {
        choices: Vec<ASTChoice>
    }

    ASTMatch {
        matched: String,
        kind: MatchKind,
        children: Vec<(String, ASTBlock)>
    }

    ASTHeader {
        title: String,
        depth: u8
    }

);

#[derive(Clone, Debug)]
pub enum ASTBlock {
    Paragraph(ASTText),
    CodeBlock(ASTCodeBlock),
    ChoiceGroup(ASTChoiceGroup),
    Match(ASTMatch),
    Header(ASTHeader)
}

impl ASTText {

    pub fn to_raw(&self) -> String {
        if self.parts.is_empty() {
            return self.tail.clone()
        }
        let mut result = String::new();
        for part in &self.parts {
            result.push_str(&part.before);
            result.push_str(&part.text.to_raw())
        }
        result.push_str(&self.tail);
        result
    }
}

impl ASTInline {

    pub fn to_raw(&self) -> String {
        self.text.to_raw()
    }
}