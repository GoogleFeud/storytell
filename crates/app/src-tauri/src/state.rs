use std::sync::Mutex;
use storytell_compiler::{
    json_compiler::{JSONCompilerContext, JSONCompilerProvider},
    base::Compiler
};
use storytell_fs::file_host::SysFileHost;
use crate::projects::Projects;

pub struct InnerStorytellState {
    pub compiler: Option<Compiler<JSONCompilerProvider, SysFileHost>>,
    pub projects: Projects
}

impl InnerStorytellState {
    pub fn new() -> Self {
        Self { 
            compiler: None,
            projects: Projects::new()
        }
    }
}

pub type StorytellState = Mutex<InnerStorytellState>;

pub fn create_storytell_state() -> StorytellState {
    Mutex::from(InnerStorytellState::new())
}