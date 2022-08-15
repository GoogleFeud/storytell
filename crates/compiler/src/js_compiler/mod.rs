pub mod compile;

use compile::{JSCompilable};
use std::collections::{HashMap};
use storytell_diagnostics::diagnostic::{Diagnostic};
use storytell_parser::{ast::{model::{ASTHeader, ASTBlock}, Parser}, input::ParsingContext};
use storytell_fs::file_host::{FileHost, FileDiagnostic, GetFindResult};

use self::compile::JSSafeCompilable;

/// The compiler just compiles everything to javascript
/// It doesn't provide a "runtime" which actually keeps
/// track of the current path, etc.
/// The compiler itself also doesn't provide any tools for analyzing.
#[derive(Clone, Debug)]
pub struct JSBootstrapVars {
    /// Name of a funtion which moves the current path,
    /// (path: string[]) => any
    pub divert_fn: &'static str,
    pub temp_divert_fn: &'static str,
    /// Responsible for creating paragraphs
    /// (text: string, attribues: Array<{name: string, params: string[]}>) => any
    pub paragraph_fn: &'static str,
    /// Responsible for creating code blocks
    /// (code: string, language: string, attribues: Array<{name: string, params: string[]}>) => any
    pub codeblock_fn: &'static str,
    /// Responsible for creating match blocks
    /// (matched: string, choices: Array<{text: string, children: Children[]}>, directChildren: Children[], kind?: string) => any
    pub match_fn: &'static str,
    /// Responsible for creating choice groups
    /// (choices: Array<{text: string, children: Children[]}>, attribues: Array<{name: string, params: string[]}>) => any
    pub choice_group_fn: &'static str,
    // Responsible for handling inline js
    /// (codes: Array<string>, collected_variables: Array<{ name: string, value_type: number }>) => any
    pub inline_js_fn: &'static str,
    /// Responsible for handling paths
    /// (path: {
    ///     title: string,
    ///     canonicalTitle: string,
    ///     childPaths: Array<This>
    ///     children: []
    /// }) => any
    pub path_fn: &'static str
}

#[derive(Copy, Clone, Debug)]
pub enum MagicVariableType {
    String = 0,
    Number,
    Bool,
    Array,
    Map,
    Unknown
}

pub struct JSCompiler<T: FileHost> {
    pub cwd: String,
    pub host: T
}

impl<T: FileHost> JSCompiler<T> {

    pub fn new(cwd: &str, host: T) -> Self {
        Self {
            cwd: cwd.to_string(),
            host
        }
    }

    fn prepare(&mut self, file_name: &str, ctx: &mut CompilerContext) -> Option<Vec<&ASTHeader>> {
        let file = match self.host.get_or_find(file_name) {
            GetFindResult::FromCache(file) => file,
            GetFindResult::Parsed(file, diagnostic) => {
                if let Some(dia) = diagnostic {
                    ctx.add_diagnostic(dia);
                }
                file
            }
            GetFindResult::NotFound => return None
        };
        let mut paths: Vec<&ASTHeader> = vec![];
        for thing in &file.content {
            if let ASTBlock::Header(header) = thing {
                ctx.paths.add_child_ast(header);
                paths.push(header);
            }
        }
        Some(paths)
    }

    pub fn compile_file(&mut self, ctx: &mut CompilerContext, file_name: &str) -> Option<String> {
        let mut result: Vec<String> = vec![];
        for header in self.prepare(file_name, ctx)? {
            match header.compile(ctx) {
                Ok(compiled) => result.push(compiled),
                Err(error) => {
                    ctx.diagnostics.push(FileDiagnostic {
                        diagnostics: error,
                        filename: file_name.to_string()
                    });
                    return None
                }
            }
        }
        Some(result.safe_compile())
    }

    pub fn compile(&mut self, bootstrap: JSBootstrapVars) -> (String, CompilerContext) {
        let mut ctx = CompilerContext::new(bootstrap);
        let mut results: Vec<String> = vec![];
        for file_path in self.host.get_files_from_directory(&self.cwd) {
            for header in self.prepare(&file_path, &mut ctx).unwrap() {
                match header.compile(&mut ctx) {
                    Ok(compiled) => results.push(compiled),
                    Err(error) => ctx.diagnostics.push(FileDiagnostic {
                        diagnostics: error,
                        filename: file_path.clone()
                    })
                }
            }
        }
        (results.safe_compile(), ctx)
    }

}

pub fn compile_str(string: &str, booststrap: JSBootstrapVars, line_endings: usize) -> (String, Vec<Diagnostic>, CompilerContext) {
    let (parsed, parsing_ctx) = Parser::new(string, ParsingContext::new(line_endings)).parse();
    let mut total_errors = parsing_ctx.diagnostics;
    let mut ctx = CompilerContext::new(booststrap);
    let mut result: Vec<String> = vec![];
    let mut headers: Vec<ASTHeader> = vec![];
    for thing in parsed {
        if let ASTBlock::Header(header) = thing {
            ctx.paths.add_child_ast(&header);
            headers.push(header);
        }
    }
    for header in headers {
        match header.compile(&mut ctx) {
            Ok(compiled) => result.push(compiled),
            Err(mut err) => total_errors.append(&mut err)
        }
    }
    (result.safe_compile(), total_errors, ctx)
}

pub struct Path {
    pub name: String,
    pub depth: u8,
    pub children: HashMap<String, Path>
}

impl Path {
    pub fn new(name: &str) -> Self {
        Path { name: name.to_string(), depth: 0, children: HashMap::new() }
    }

    pub fn add_child_ast(&mut self, ast: &ASTHeader) {
        let path_name = Self::canonicalize_name(&ast.title.text);
        let mut path = Self {
            name: path_name.clone(),
            depth: ast.depth,
            children: HashMap::new()
        };
        for child in &ast.children {
            if let ASTBlock::Header(block) = child {
                path.add_child_ast(block);
            }
        }
        self.children.insert(path_name, path);
    }

    pub fn get_child_by_path(&self, path: &[String]) -> Option<&Path> {
        let mut found_path = self.children.get(&path[0])?;
        for p in path.iter().skip(1) {
            found_path = found_path.children.get(p)?;
        }
        Some(found_path)
    }

    pub fn try_get_child_by_path(&self, path: &Vec<String>) -> Result<&Path, usize> {
        let mut found_path = if let Some(path) = self.children.get(&path[0]) {
            path
        } else {
            return Err(0);
        };
        for ind in 1..path.len() {
            found_path = if let Some(path) = found_path.children.get(&path[ind]) {
                path
            } else {
                return Err(ind);
            }
        }
        Ok(found_path)
    }

    /// Path names can only contain lowercase letters, digits and underscores.
    /// Empty spaces are replaced with underscores.
    /// Capital letters are replaced with their lowercase variants.
    /// Any other character gets erased.
    pub fn canonicalize_name(name: &str) -> String {
        let mut canonical = String::new();
        for character in name.chars() {
            match character {
                ' ' => canonical.push('_'),
                '_' | 'a'..='z' | '0'..='9' => canonical.push(character),
                'A'..='Z' => canonical.push(character.to_lowercase().next().unwrap()),
                _ => {}
            }
        }
        canonical
    }

}

pub struct CompilerContext {
    pub magic_variables: HashMap<String, MagicVariableType>,
    pub diagnostics: Vec<FileDiagnostic>,
    pub paths: Path,
    pub bootstrap: JSBootstrapVars
}

impl CompilerContext {

    pub fn new(bootstrap: JSBootstrapVars) -> Self {
        Self { 
            magic_variables: HashMap::new(), 
            diagnostics: vec![], 
            paths: Path::new("global"),
            bootstrap 
        }
    }

    pub fn add_diagnostic(&mut self, err: FileDiagnostic) {
        self.diagnostics.push(err);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    const BOOTSTRAP_VARS: JSBootstrapVars = JSBootstrapVars {
        divert_fn: "divert",
        temp_divert_fn: "tempDivert",
        paragraph_fn: "Paragraph",
        codeblock_fn: "Codeblock",
        match_fn: "Match",
        choice_group_fn: "ChoiceGroup",
        inline_js_fn: "Js",
        path_fn: "Path"
    };

    #[test]
    fn compile() {
        let (result, diagnostics, _ctx) = compile_str("
# Hello, World!
How's it going on this {a += 1} {b += 5; c += 'Hello World!'; v = d = 33}?

```js
console.log(\"some code...\");
```
Hello!

## This is a subpath...

- This is a choice group!
    {killed += 1}
    {saved -= 1}
- Second choice 
    -> hello_world
", BOOTSTRAP_VARS.clone(), 1);
        println!("{} {:?}", result, diagnostics);
        panic!("AAA");
    }

}
