pub mod compile;

use compile::{JSCompilable};
use storytell_diagnostics::diagnostic::{StorytellResult};
use storytell_parser::{ast::{model::{ASTHeader}}};
use storytell_fs::file_host::{FileDiagnostic};
use crate::visitors::{MagicVariableCollectorContext};
use crate::path::Path;
use crate::base::*;

/// The compiler just compiles everything to javascript
/// It doesn't provide a "runtime" which actually keeps
/// track of the current path, etc.
/// The compiler itself also doesn't provide any tools for analyzing.
#[derive(Clone, Debug)]
pub struct JSBootstrapVars {
    /// Name of a funtion which moves the current path,
    /// (path: string[]) => any
    pub divert_fn: &'static str,
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

pub struct JSCompilerProvider;

impl CompilerProvider for JSCompilerProvider {
    type Output = String;
    type Context = JSCompilerContext;

    fn compile_header(file: &ASTHeader, ctx: &mut Self::Context) -> StorytellResult<Self::Output> {
        file.compile(ctx)
    }
}

pub struct JSCompilerContext {
    pub magic_variables: MagicVariableCollectorContext,
    pub diagnostics: Vec<FileDiagnostic>,
    pub paths: Path,
    pub bootstrap: JSBootstrapVars
}

impl CompilerContext for JSCompilerContext {
    fn add_diagnostic(&mut self, dia: FileDiagnostic) {
        self.diagnostics.push(dia);
    }

    fn get_global_path(&mut self) -> &mut Path {
        &mut self.paths
    }
}

impl JSCompilerContext {

    pub fn new(bootstrap: JSBootstrapVars) -> Self {
        Self { 
            magic_variables: MagicVariableCollectorContext::new(), 
            diagnostics: vec![], 
            paths: Path::new("global"),
            bootstrap 
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::js_compiler::compile::JSSafeCompilable;

    use super::*;
    use std::time::{Instant};

    const BOOTSTRAP_VARS: JSBootstrapVars = JSBootstrapVars {
        divert_fn: "divert",
        paragraph_fn: "Paragraph",
        codeblock_fn: "Codeblock",
        match_fn: "Match",
        choice_group_fn: "ChoiceGroup",
        inline_js_fn: "Js",
        path_fn: "Path"
    };

    #[test]
    fn compile() {
        let before = Instant::now();
        let (result, diagnostics, ctx) = compile_str::<JSCompilerProvider>("
# Hello, World!
How's it going on this {a += 1} {b += 5; c += 'Hello World!'; v = d = 33}? `Test!`

```js
console.log(`Some code ${123}`);
```
Hello!

## This is a subpath...

#[SomeAttribute(123, 456, 789)]
- This is a choice group!
    {killed += n}
    {saved -= 1}
- Second choice 
    -> hello_world
    @{`Some magic string $(my_var + 1) and $(my_var + 2)...`}
    - {true}
        This condition is true...
    - {false}
        This condition is false...

{killed = c}
{e.b.c.d += 1}
{e.b.c.d}
", JSCompilerContext::new(BOOTSTRAP_VARS.clone()), 1);
        println!("Parsing took {} nanoseconds", before.elapsed().as_nanos());
        println!("{} {:?} {:?}", result.safe_compile(), diagnostics, ctx.magic_variables);
        panic!("AAA");
    }

}
