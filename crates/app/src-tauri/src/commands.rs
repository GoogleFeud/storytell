use storytell_compiler::{base::{Compiler, files::BlobId}, json_compiler::{JSONCompilerProvider}, json};
use storytell_fs::SysFileHost;
use tauri::State;
use crate::{state::StorytellState, projects::Project, deserialization::JSONCompilable};
use serde_json::to_string;

#[tauri::command]
pub fn list_projects(state: State<StorytellState>) -> String {
    let inner_state = state.lock().unwrap();
    to_string::<Vec<&Project>>(&inner_state.projects.projects.values().collect::<Vec<&Project>>()).unwrap()
}

#[tauri::command]
pub fn create_project(state: State<StorytellState>, name: String, description: String) -> String {
    let mut inner_state = state.lock().unwrap();
    to_string::<Option<&Project>>(&inner_state.projects.create_project(name, description)).unwrap()
}

#[tauri::command]
pub fn edit_project(state: State<StorytellState>, id: String, name: String, description: Option<String>) {
    let mut inner_state = state.lock().unwrap();
    inner_state.projects.update_project(id, name, description.unwrap_or_default());
}

#[tauri::command]
pub fn delete_project(state: State<StorytellState>, id: String) {
    let mut inner_state = state.lock().unwrap();
    inner_state.projects.delete_project(&id);
}

#[tauri::command]
pub fn rename_blob(state: State<StorytellState>, id: u16, name: String) {
    let mut inner_state = state.lock().unwrap();
    let compiler = inner_state.compiler.as_mut().unwrap();
    compiler.host.rename_blob(&id, name);
}

#[tauri::command]
pub fn delete_blob(state: State<StorytellState>, id: u16) {
    let mut inner_state = state.lock().unwrap();
    let compiler = inner_state.compiler.as_mut().unwrap();
    compiler.host.delete_blob(&id);
}

#[tauri::command]
pub fn create_file(state: State<StorytellState>, name: String, parent: Option<BlobId>) -> String {
    let mut inner_state = state.lock().unwrap();
    let compiler = inner_state.compiler.as_mut().unwrap();
    let file = compiler.host.create_file(name, parent).borrow();
    file.compile()
}

// Returns all the files for the file manager
// Compiles the last opened file if necessary
#[tauri::command]
pub fn init_compiler(state: State<StorytellState>, project_id: String) -> Option<String> {
    let mut inner_state = state.lock().unwrap();
    let project = inner_state.projects.projects.get(&project_id)?;
    #[cfg(windows)]
    let line_endings = 2;
    #[cfg(not(windows))]
    let line_endings = 1;
    let mut compiler = Compiler::<JSONCompilerProvider, SysFileHost>::new(project.files_directory.to_str().unwrap(), line_endings, SysFileHost::default());
    let global_files = compiler.host.load_cwd();
    let json_str = json!({
        fileExplorer: json!({
            blobs: format!("{{{}}}", compiler.host.dirs.iter()
                .map(|i| format!("\"{}\":{}", i.0, i.1.borrow().compile()))
                .chain(compiler.host.files.iter()
                    .map(|i| format!("\"{}\":{}", i.0, i.1.borrow().compile())))
                .collect::<Vec<String>>().join(",")),
            global: global_files.compile(),
            lastId: compiler.host.counter
        })
    });
    inner_state.compiler = Some(compiler);
    Some(json_str)
}