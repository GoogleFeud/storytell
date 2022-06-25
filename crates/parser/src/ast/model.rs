
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

#[derive(Clone, Debug)]
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

create_nodes!(
    ASTInline {
        kind: ASTInlineKind,
        text: String
    }

    ASTParagraph {
        text: String,
        inline_points: Vec<(usize, ASTInline)>
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
    Paragraph(ASTParagraph),
    CodeBlock(ASTCodeBlock),
    ChoiceGroup(ASTChoiceGroup),
    Match(ASTMatch),
    Header(ASTHeader)
}