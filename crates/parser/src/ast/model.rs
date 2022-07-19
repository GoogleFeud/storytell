
use storytell_diagnostics::location::*;
use std::fmt;

pub trait WithAttributes {
    fn has_attribute(&self, attribute: &str) -> bool;
    fn get_attribute_params(&self, attribute: &str) -> Option<&Vec<String>>;
}

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

            impl WithAttributes for $name {
                fn has_attribute(&self, attribute: &str) -> bool {
                    for item in &self.attributes {
                        if item.name == attribute {
                            return true;
                        }
                    }
                    return false;
                }

                fn get_attribute_params(&self, attribute: &str) -> Option<&Vec<String>> {
                    for item in &self.attributes {
                        if item.name == attribute {
                            return Some(&item.parameters);
                        }
                    }
                    return None;
                }
            }
        )+
    };
    (NoAttribute $($name: ident {$($field_name: ident: $field_type: ty),*})+) => {
        $(
            #[derive(Clone)]
            pub struct $name {
                $(pub $field_name: $field_type,)*
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
    Bold(ASTText),
    // { ... }
    Javascript(String),
    // *...*
    Italics(ASTText),
    // __...__
    Underline(ASTText),
    // `...`
    Code(ASTText),
    // -> / <->
    Divert(Vec<String>, bool)
}

#[derive(Clone, Debug)]
pub struct TextPart {
    pub before: String,
    pub text: ASTInline
}

create_nodes!(NoAttribute

    ASTInline {
        kind: ASTInlineKind
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
        children: Vec<ASTBlock>
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
            Self::Divert(paths, _) => paths.join(".")
        } 
    }
}