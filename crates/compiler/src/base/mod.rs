use std::{marker::PhantomData, fs::DirEntry};
use rustc_hash::FxHashSet;
use storytell_diagnostics::{diagnostic::{StorytellResult, Diagnostic, DiagnosticMessage}, make_diagnostics};
use storytell_parser::{ast::{model::{ASTHeader, ASTBlock}, Parser}};
use storytell_fs::FileHost;
pub mod files;
use files::CompilerFileHost;

use self::files::{BlobId, Directory, File, CompiledFileData};

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
    pub host: CompilerFileHost<F>,
    pub ctx: P::Context,
    _provider: PhantomData<P>
}

impl<P: CompilerProvider, F: FileHost> Compiler<P, F> {

    pub fn new(cwd: &str, line_endings: usize, host: F, ctx: P::Context) -> Self {
        Self {
            host: CompilerFileHost::new(cwd, line_endings, host),
            ctx,
            _provider: PhantomData::default()
        }
    }

    pub fn reset(&mut self) -> (FxHashSet<BlobId>, Vec<CompiledFileData<P::Output>>) {
        self.host.counter = 1;
        self.host.files.clear();
        self.host.dirs.clear();
        self.init_fs()
    }

    pub fn init_fs(&mut self) -> (FxHashSet<BlobId>, Vec<CompiledFileData<P::Output>>) {
        let mut parsed_files: Vec<CompiledFileData<P::Output>> = vec![];
        let line_endings = self.host.line_endings;
        let cwd = self.host.cwd.clone();
        let global = self.host.register_dir(cwd, vec![], &mut |entry, path, children, id| {
            Directory {
                name: entry.file_name().to_str().unwrap().to_string(),
                path: path.clone(),
                id,
                parent: path.last().cloned(),
                children
            }
        }, &mut |c: &CompilerFileHost<F>, entry: DirEntry, path: Vec<BlobId>, id: BlobId| {
            let file_contents = c.raw.read_file(entry.path()).unwrap();
            let (parsed_content, mut dias) = Parser::parse(&file_contents, line_endings);
            if let Some(ASTBlock::Header(header))  = parsed_content.get(0) {
                match P::compile_header(header, &mut self.ctx) {
                    Ok(compiled) => parsed_files.push(CompiledFileData {
                        id,
                        compiled_content: Some(compiled),
                        content: file_contents,
                        diagnostics: dias
                    }),
                    Err(mut error) => {
                        dias.append(&mut error);
                        parsed_files.push(CompiledFileData { 
                            id,
                            compiled_content: None,
                            content: file_contents, 
                            diagnostics: dias 
                        });
                    }
                }
            }
            File {
                parsed_content,
                name: entry.file_name().to_str().unwrap().to_string(),
                path: path.clone(),
                parent: path.last().cloned(),
                id
            }
        });
        (global, parsed_files)
    }

    pub fn compile_file(&mut self, file_id: BlobId) -> (Option<P::Output>, String, Vec<Diagnostic>) {
        let (file, text, mut dia) = self.host.parse_file(file_id).unwrap();
        if let Some(ASTBlock::Header(header))  = file.parsed_content.get(0) {
            match P::compile_header(header, &mut self.ctx) {
                Ok(compiled) => (Some(compiled), text, dia),
                Err(mut error) => {
                    dia.append(&mut error);
                    (None, text, dia)
                }
            }
        } else {
            (None, text, dia)
        }
    }

    pub fn compile_file_with_content(&mut self, file_id: BlobId, content: &str) -> (Option<P::Output>, Vec<Diagnostic>) {
        let (content, mut dia) = Parser::parse(content, self.host.line_endings);
        let mut file = self.host.files.get(&file_id).unwrap().borrow_mut();
        file.parsed_content = content;
        if let Some(ASTBlock::Header(header))  = file.parsed_content.get(0) {
            match P::compile_header(header, &mut self.ctx) {
                Ok(compiled) => (Some(compiled), dia),
                Err(mut error) => {
                    dia.append(&mut error);
                    (None, dia)
                }
            }
        } else {
            (None, dia)
        }
    }


}

pub fn compile_str<P: CompilerProvider>(string: &str, mut ctx: P::Context, line_endings: usize) -> (Vec<P::Output>, Vec<Diagnostic>, P::Context) {
    let (parsed, mut total_errors) = Parser::parse(string, line_endings);
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