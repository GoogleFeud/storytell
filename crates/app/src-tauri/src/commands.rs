use storytell_compiler::{base::Compiler, json_compiler::{JSONCompilerProvider}, json};
use storytell_fs::file_host::{SysFileHost, FileHost};
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
    inner_state.projects.update_project(id, name, description.unwrap_or(String::new()));
}

#[tauri::command]
pub fn delete_project(state: State<StorytellState>, id: String) {
    let mut inner_state = state.lock().unwrap();
    inner_state.projects.delete_project(&id);
}

#[tauri::command]
pub fn rename_file(state: State<StorytellState>, id: u16, name: String) -> Option<String> {
    let mut inner_state = state.lock().unwrap();
    if let Some(compiler) = inner_state.compiler.as_mut() {
        compiler.host.rename_file(id, name)
    } else {
        None
    }
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
    let mut compiler = Compiler::<JSONCompilerProvider, SysFileHost>::new(SysFileHost::new(line_endings), project.files_directory.to_str().unwrap());
    let global_files = compiler.host.load_directory(project.files_directory.to_str().unwrap());
    let json_str = json!({
        fileExplorer: json!({
            dirs: compiler.host.directories.compile(),
            files: compiler.host.files.compile(),
            global: global_files.compile(),
            lastId: compiler.host.counter
        })
    });
    inner_state.compiler = Some(compiler);
    Some(json_str)
}