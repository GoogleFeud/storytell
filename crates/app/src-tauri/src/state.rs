use std::sync::Mutex;
use crate::{projects::Projects, compiler::CompilerWrapper};

#[derive(Default)]
pub struct InnerStorytellState {
    pub compiler: Option<CompilerWrapper>,
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