use std::{marker::PhantomData, fs::DirEntry};
use rustc_hash::FxHashSet;
use storytell_diagnostics::{diagnostic::*, make_diagnostics, dia, location::Range};
use storytell_parser::ast::{model::{ASTHeader, ASTBlock}, Parser};
use storytell_fs::FileHost;
pub mod files;
use files::CompilerFileHost;

use self::files::{BlobId, Directory, File, CompiledFileData};

make_diagnostics!(define [
    MISSING_HEADER,
    "File must contain just one top-level (#) path."
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

    pub fn compile_string(ctx: &mut P::Context, line_endings: usize, text: &str) -> (Option<P::Output>, Vec<ASTBlock>, Vec<Diagnostic>) {
        let (parsed_content, mut dias) = Parser::parse(text, line_endings);
        match parsed_content.get(0) {
            Some(ASTBlock::Header(header)) if header.depth == 1 => {
                match P::compile_header(header, ctx) {
                    Ok(compiled) => (Some(compiled), parsed_content, dias),
                    Err(mut error) => {
                        dias.append(&mut error);
                        (None, parsed_content, dias)
                    }
                }
            },
            _ => {
                dias.push(dia!(MISSING_HEADER, Range::new(0, text.len())));
                (None, parsed_content, dias)
            }
        }
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
            let (compiled_content, parsed_content,  diagnostics) = Self::compile_string(&mut self.ctx, line_endings, &file_contents);
            parsed_files.push(CompiledFileData {
                id,
                compiled_content,
                diagnostics,
                content: file_contents,
            });
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
        let mut file = self.host.files.get(&file_id).unwrap().borrow_mut();
        let file_contents = self.host.raw.read_file(self.host.build_path(&file.path, &file.name)).unwrap();
        let (output, parsed, diagnostics) = Self::compile_string(&mut self.ctx, self.host.line_endings, &file_contents);
        file.parsed_content = parsed;
        (output, file_contents, diagnostics)
    }

    pub fn compile_file_with_content(&mut self, file_id: BlobId, content: &str) -> (Option<P::Output>, Vec<Diagnostic>) {
        let (compiled, parsed, diagnostics) = Self::compile_string(&mut self.ctx, self.host.line_endings, content);
        let mut file = self.host.files.get(&file_id).unwrap().borrow_mut();
        file.parsed_content = parsed;
        (compiled, diagnostics)
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