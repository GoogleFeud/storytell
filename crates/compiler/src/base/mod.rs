use std::marker::PhantomData;

use storytell_diagnostics::{diagnostic::{StorytellResult, Diagnostic, DiagnosticMessage}, make_diagnostics};
use storytell_fs::{file_host::{FileDiagnostic, FileHost, GetFindResult}};
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
    fn add_diagnostic(&mut self, dia: FileDiagnostic);
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

    pub fn prepare_file<C: CompilerContext>(&mut self, file_name: &str, ctx: &mut C) -> Option<Vec<&ASTHeader>> {
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
                ctx.process_path(header);
                paths.push(header);
            }
        }
        Some(paths)
    }

    pub fn compile_file<R>(&mut self, file_name: &str, ctx: &mut P::Context) -> Option<Vec<P::Output>> {
        let mut result: Vec<P::Output> = vec![];
        for header in self.prepare_file(file_name, ctx)? {
            match P::compile_header(header, ctx) {
                Ok(compiled) => result.push(compiled),
                Err(error) => {
                    ctx.add_diagnostic(FileDiagnostic {
                        diagnostics: error,
                        filename: file_name.to_string()
                    });
                    return None
                }
            }
        }
        Some(result)
    }

    pub fn compile(&mut self, mut ctx: P::Context) -> (Vec<P::Output>, P::Context) {
        let mut results: Vec<P::Output> = vec![];
        for file_path in self.host.get_files_from_directory(&self.cwd) {
            for header in self.prepare_file(&file_path, &mut ctx).unwrap() {
                match P::compile_header(header, &mut ctx) {
                    Ok(compiled) => results.push(compiled),
                    Err(error) => ctx.add_diagnostic(FileDiagnostic {
                        diagnostics: error,
                        filename: file_path.clone()
                    })
                }
            }
        }
        (results, ctx)
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