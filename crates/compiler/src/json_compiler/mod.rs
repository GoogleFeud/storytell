use storytell_diagnostics::diagnostic::StorytellResult;
use storytell_parser::ast::model::ASTHeader;
use crate::{base::{*, files::BlobId}, visitors::variable::VariableStore};
use self::compile::JSONCompilable;

pub mod compile;

pub struct JSONCompilerProvider;

impl CompilerProvider for JSONCompilerProvider {
    type Output = String;
    type Context = JSONCompilerContext;

    fn compile_header(file: &ASTHeader, ctx: &mut Self::Context, file_id: BlobId) -> StorytellResult<Self::Output> {
        file.compile(ctx, file_id)
    }
}

#[derive(Default)]
pub struct JSONCompilerContext {
    pub variables: VariableStore<BlobId>,
    pub prefix_js_idents: Option<String>
}

impl CompilerContext for JSONCompilerContext {

    fn process_path(&mut self, _path: &ASTHeader) {
        // Not needed, for now the front-end handles this.
    }

}

impl JSONCompilerContext {

    pub fn new(prefix_js_idents: Option<String>) -> Self {
        Self { 
            variables: VariableStore::default(),
            prefix_js_idents
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
", JSONCompilerContext::new(None), 1);
        println!("Parsing took {} nanoseconds", before.elapsed().as_nanos());
        println!("[{}] {:?} {:?}", result.join(","), diagnostics, ctx.variables);
    }

}