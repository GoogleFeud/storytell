use std::marker::PhantomData;
use storytell_diagnostics::{diagnostic::{StorytellResult, Diagnostic, DiagnosticMessage}, make_diagnostics};
use storytell_parser::{ast::{model::{ASTHeader, ASTBlock}, Parser}, input::ParsingContext};
use storytell_fs::FileHost;
pub mod files;
use files::{CompilerFileHost, FileDiagnostic};

make_diagnostics!(define [
    UNKNOWN_CHILD_PATH,
    C1001,
    "\"$\" is not a sub-path of \"$\"."
], [
    UNKNOWN_PATH,
    C1002,
    "\"$\" is not a path."
]);

pub trait CompilerContext {
    fn process_path(&mut self, path: &ASTHeader);
}

pub trait CompilerProvider {
    type Output;
    type Context: CompilerContext;
    fn compile_header(file: &ASTHeader, ctx: &mut Self::Context) -> StorytellResult<Self::Output>;
}

pub struct Compiler<P: CompilerProvider, F: FileHost> {
    pub cwd: String,
    pub host: CompilerFileHost<F>,
    pub ctx: P::Context,
    _provider: PhantomData<P>
}

impl<P: CompilerProvider, F: FileHost> Compiler<P, F> {

    pub fn new(cwd: &str, line_endings: usize, host: F, ctx: P::Context) -> Self {
        Self {
            host: CompilerFileHost::new(cwd, line_endings, host),
            cwd: cwd.to_string(),
            ctx,
            _provider: PhantomData::default()
        }
    }

    pub fn compile_file(&mut self, file_id: u16) -> (Option<P::Output>, FileDiagnostic) {
        let mut file = self.host.files.get(&file_id).unwrap().borrow_mut();
        let text_content = self.host.raw.read_file(self.host.build_path(&file.path, &file.name)).unwrap();
        let parsed = Parser::new(&text_content, ParsingContext::new(self.host.line_endings)).parse();
        file.text_content = text_content;
        let mut dia = FileDiagnostic {
            file_id,
            diagnostics: parsed.1.diagnostics
        };
        let val = if let Some(ASTBlock::Header(header))  = parsed.0.get(0) {
            match P::compile_header(header, &mut self.ctx) {
                Ok(compiled) => (Some(compiled), dia),
                Err(mut error) => {
                    dia.diagnostics.append(&mut error);
                    (None, dia)
                }
            }
        } else {
            (None, dia)
        };
        file.parsed_content = parsed.0;
        val
    }

    /*
    pub fn compile_file_with_content(&mut self, file_id: u16, content: &str, ctx: &mut P::Context) -> (Option<P::Output>, FileDiagnostic) {
        let mut file = self.host.files.get(&file_id).unwrap().borrow_mut();
        let (res, mut parsing_ctx) = Parser::new(content, ParsingContext::new(self.host.line_endings)).parse();
        let result = match P::compile_blocks(&res, ctx) {
            Ok(compiled) => (Some(compiled), FileDiagnostic {
                file_id,
                diagnostics: parsing_ctx.diagnostics
            }),
            Err(mut dias) => (None, FileDiagnostic {
                file_id,
                diagnostics: {
                    dias.append(&mut parsing_ctx.diagnostics);
                    dias
                }
            })
        };
        file.parsed_content = res;
        file.text_content = content.to_string();
        result
    }
    */


}

pub fn compile_str<P: CompilerProvider>(string: &str, mut ctx: P::Context, line_endings: usize) -> (Vec<P::Output>, Vec<Diagnostic>, P::Context) {
    let (parsed, parsing_ctx) = Parser::new(string, ParsingContext::new(line_endings)).parse();
    let mut total_errors = parsing_ctx.diagnostics;
    let mut result: Vec<P::Output> = vec![];
    let mut headers: Vec<ASTHeader> = vec![];
    for thing in parsed {
        if let ASTBlock::Header(header) = thing {
            ctx.process_path(&header);
            headers.push(header);
        }
    }
    for header in headers {
        match P::compile_header(&header, &mut ctx) {
            Ok(compiled) => result.push(compiled),
            Err(mut err) => total_errors.append(&mut err)
        }
    }
    (result, total_errors, ctx)
}