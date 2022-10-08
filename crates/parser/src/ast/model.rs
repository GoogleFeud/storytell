use std::cmp::PartialEq;
use storytell_diagnostics::location::*;
use std::fmt;

pub trait WithAttributes {
    fn get_attribute_n(&self, att: &str, ind: usize) -> Option<&str>;
}

macro_rules! create_nodes {
    ($($name: ident {$($field_name: ident: $field_type: ty),*})+) => {
        $(
            #[derive(Clone, Eq)]
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

            impl PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    self.attributes == other.attributes && $(self.$field_name == other.$field_name)&&*
                }
            }

            impl WithAttributes for $name {
                fn get_attribute_n(&self, att: &str, ind: usize) -> Option<&str> {
                    for item in &self.attributes {
                        if item.name == att {
                            return Some(&item.parameters[ind]);
                        }
                    }
                    return None;
                }
            }
        )+
    };
    (NoAttribute $($name: ident {$($field_name: ident: $field_type: ty),*})+) => {
        $(
            #[derive(Clone, Eq)]
            pub struct $name {
                $(pub $field_name: $field_type,)*
                pub range: Range<usize>
            }

            impl PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    $(self.$field_name == other.$field_name)&&*
                }
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

#[derive(Clone, Debug,  PartialEq, Eq)]
pub enum ASTInlineKind {
    // **...**
    Bold(ASTText),
    // { ... }
    Javascript(String),
    // *...*
    Italics(ASTText),
    // _..._
    Underline(ASTText),
    // `...`
    Code(ASTText),
    Join
}

#[derive(Clone, Debug, Eq)]
pub struct TextPart {
    pub before: String,
    pub text: ASTInline
}

impl PartialEq for TextPart {
    fn eq(&self, other: &Self) -> bool {
        self.before == other.before && self.text == other.text
    }
}

create_nodes!(NoAttribute

    ASTInline {
        kind: ASTInlineKind
    }

    ASTPlainText {
        text: String
    }

    ASTAttribute {
        name: String,
        parameters: Vec<String>
    }

    ASTText {
        parts: Vec<TextPart>,
        tail: String
    }

);

create_nodes!(

    ASTParagraph {
        parts: Vec<TextPart>,
        tail: String
    }

    ASTCodeBlock {
        language: String,
        text: String
    }

    ASTChoice {
        text: ASTText,
        children: Vec<ASTBlock>,
        condition: Option<(String, String)>
    }

    ASTChoiceGroup {
        choices: Vec<ASTChoice>
    }

    ASTMatch {
        matched: String,
        kind: Option<String>,
        choices: Vec<ASTChoice>,
        direct_children: Vec<ASTBlock>
    }

    ASTHeader {
        title: ASTPlainText,
        children: Vec<ASTBlock>,
        depth: u8
    }

    ASTDivert {
        path: Vec<String>
    }

);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ASTBlock {
    Paragraph(ASTParagraph),
    CodeBlock(ASTCodeBlock),
    ChoiceGroup(ASTChoiceGroup),
    Divert(ASTDivert),
    Match(ASTMatch),
    Header(ASTHeader)
}

impl WithAttributes for ASTBlock {
    fn get_attribute_n(&self, att: &str, ind: usize) -> Option<&str> {
        match self {
            Self::ChoiceGroup(ch) => ch.get_attribute_n(att, ind),
            Self::CodeBlock(code) => code.get_attribute_n(att, ind),
            Self::Divert(div) => div.get_attribute_n(att, ind),
            Self::Header(h) => h.get_attribute_n(att, ind),
            Self::Match(m) => m.get_attribute_n(att, ind),
            Self::Paragraph(p) => p.get_attribute_n(att, ind)
        }
    }
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

impl ASTParagraph {

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
        self.kind.to_raw()
    }
}

impl ASTInlineKind {
    pub fn to_raw(&self) -> String {
        match self {
            Self::Bold(text) => text.to_raw(),
            Self::Code(text) => text.to_raw(),
            Self::Italics(text) => text.to_raw(),
            Self::Javascript(text) => text.clone(),
            Self::Underline(text) => text.to_raw(),
            Self::Join => "++".to_string()
        }
    }
}