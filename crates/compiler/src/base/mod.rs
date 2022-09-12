use std::marker::PhantomData;

use storytell_diagnostics::{diagnostic::{StorytellResult, Diagnostic, DiagnosticMessage}, make_diagnostics};
use storytell_fs::{file_host::{FileDiagnostic, FileHost}};
use storytell_parser::{ast::{model::{ASTHeader, ASTBlock}, Parser}, input::ParsingContext};

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
    pub host: F,
    _provider: PhantomData<P>
}

impl<P: CompilerProvider, F: FileHost> Compiler<P, F> {

    pub fn new(host: F, cwd: &str) -> Self {
        Self {
            host,
            cwd: cwd.to_string(),
            _provider: PhantomData::default()
        }
    }

    pub fn compile_file<R>(&mut self, file_id: u16, ctx: &mut P::Context) -> (Vec<P::Output>, FileDiagnostic) {
        let mut result: Vec<P::Output> = vec![];
        let file = self.host.parse_file_by_id(file_id).unwrap();
        let mut dia = FileDiagnostic {
            file_id,
            diagnostics: vec![]
        };
        for thing in &file.content {
            if let ASTBlock::Header(header) = thing {
                match P::compile_header(header, ctx) {
                    Ok(compiled) => result.push(compiled),
                    Err(mut error) => {
                        dia.diagnostics.append(&mut error)
                    }
                }
            }
        }
        (result, dia)
    }

    pub fn compile(&mut self, mut ctx: P::Context) -> (Vec<P::Output>, Vec<FileDiagnostic>, P::Context) {
        let mut results: Vec<P::Output> = vec![];
        let mut diagnostics: Vec<FileDiagnostic> = vec![];
        let line_endings = self.host.get_line_endings();
        for file in self.host.get_all_files() {
            let mut dias = file.parse(line_endings);
            for thing in &file.content {
                if let ASTBlock::Header(header) = thing {
                    match P::compile_header(header, &mut ctx) {
                        Ok(compiled) => results.push(compiled),
                        Err(mut error) => dias.append(&mut error)
                    }
                }
            }
            if !dias.is_empty() {
                diagnostics.push(FileDiagnostic { diagnostics: dias, file_id: file.id })
            }
        }
        (results, diagnostics, ctx)
    }


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