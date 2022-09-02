use storytell_diagnostics::diagnostic::{StorytellResult, Diagnostic};
use storytell_fs::file_host::FileDiagnostic;
use storytell_parser::ast::model::ASTHeader;
use crate::{base::*, visitors::MagicVariableCollectorContext, path::{PathCollector}};
use self::compile::JSONCompilable;

pub mod compile;

pub struct JSONCompilerProvider;

impl CompilerProvider for JSONCompilerProvider {
    type Output = String;
    type Context = JSONCompilerContext;

    fn compile_header(file: &ASTHeader, ctx: &mut Self::Context) -> StorytellResult<Self::Output> {
        file.compile(ctx)
    }
}

pub struct JSONCompilerContext {
    pub magic_variables: MagicVariableCollectorContext,
    pub diagnostics: Vec<FileDiagnostic>,
    pub paths: PathCollector,
    pub include_details: bool
}

impl CompilerContext for JSONCompilerContext {
    fn add_diagnostic(&mut self, dia: FileDiagnostic) {
        self.diagnostics.push(dia);
    }

    fn add_main_path(&mut self, ast: &ASTHeader) -> Result<(), Diagnostic> {
        self.paths.add_main_path(ast)
    }
}

impl JSONCompilerContext {

    pub fn new(include_details: bool) -> Self {
        Self { 
            magic_variables: MagicVariableCollectorContext::new(),
            diagnostics: vec![],
            paths: PathCollector::new(),
            include_details
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::json_compiler::*;
    use std::time::{Instant};

    #[test]
    fn compile() {
        let before = Instant::now();
        let (result, diagnostics, ctx) = compile_str::<JSONCompilerProvider>("
# Hello, World!
How's it going on this {a += 1} {b += 5; c.push(123); c.pop(); v = d = 33}? `Test!`

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
", JSONCompilerContext::new(true), 1);
        println!("Parsing took {} nanoseconds", before.elapsed().as_nanos());
        println!("[{}] {:?} {:?}", result.join(","), diagnostics, ctx.magic_variables);
    }
    
    #[test]
    fn path_test() {
        let (_, dia, _) = compile_str::<JSONCompilerProvider>("
# This is a main path

This is some main path content...
Let's go to a sub-path...
-> my_sub_path

## My Sub Path

This is my sub-path.
-> this_is_a_main_path.second_sub_path

## Second Sub Path

This is the second sub-path.
-> first_sub_sub_path

## Second Sub Path

### First sub sub path!!!!

## Wo

", JSONCompilerContext::new(true), 1);
        panic!("{:?}",dia )
    }
}