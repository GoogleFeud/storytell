use storytell_compiler::{base::{files::{BlobId, FileDiagnostic}}, json};
use storytell_diagnostics::{make_diagnostics, dia, diagnostic::*};
use storytell_fs::{SysFileHost, FileHost};
use tauri::State;
use crate::{state::StorytellState, projects::Project, deserialization::JSONSerializable, compiler::CompilerWrapper};
use serde_json::to_string;

make_diagnostics!(define [
    INCORRECT_VARIABLE_TYPE,
    C1002,
    "Variable has multiple types assigned ()."
]);

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
    compiler.inner.host.rename_blob(&id, name);
}

#[tauri::command]
pub fn delete_blob(state: State<StorytellState>, id: u16) {
    let mut inner_state = state.lock().unwrap();
    let compiler = inner_state.compiler.as_mut().unwrap();
    compiler.inner.host.delete_blob(&id);
}

#[tauri::command]
pub fn create_blob(state: State<StorytellState>, name: String, parent: Option<BlobId>, dir: bool) -> String {
    let mut inner_state = state.lock().unwrap();
    let compiler = inner_state.compiler.as_mut().unwrap();
    let file_id = compiler.inner.host.create_blob(name, parent, dir);
    if dir {
        let dir = compiler.inner.host.dirs.get(&file_id).unwrap().borrow();
        dir.compile()
    } else {
        let file = compiler.inner.host.files.get(&file_id).unwrap().borrow();
        file.compile()
    }
}

#[tauri::command]
pub fn refresh_blobs(state: State<StorytellState>) -> String {
    let mut inner_state = state.lock().unwrap();
    let compiler = inner_state.compiler.as_mut().unwrap();
    let (global_files, compiled_files) = compiler.inner.reset(&mut CompilerWrapper::create_ctx());
    json!({
        blobs: format!("{{{}}}", compiler.inner.host.dirs.iter()
            .map(|i| format!("\"{}\":{}", i.0, i.1.borrow().compile()))
            .chain(compiler.inner.host.files.iter()
                .map(|i| format!("\"{}\":{}", i.0, i.1.borrow().compile())))
            .collect::<Vec<String>>().join(",")),
        global: global_files.compile(),
        contents: compiled_files.compile()
    })
}

#[tauri::command]
pub fn recompile_file(state: State<StorytellState>, file_id: BlobId, content: String) -> String {
    let mut inner_state = state.lock().unwrap();
    let compiler = inner_state.compiler.as_mut().unwrap();
    compiler.variables.remove(&file_id);
    let mut context = CompilerWrapper::create_ctx();
    let (compiled, diagnostics) = compiler.inner.compile_file_with_content(file_id, &content, &mut context);
    compiler.variables.insert(file_id.clone(), context.variables);
    let file = compiler.inner.host.files.get(&file_id).unwrap().borrow();
    compiler.inner.host.raw.write_file(&compiler.inner.host.build_path(&file.path, &file.name), &content).expect("Couldn't write to file.");
    let other_diagnostics: Vec<FileDiagnostic> = vec![];
    for (name, variable) in compiler.variables.0.iter() {
        if variable.get_common_kind().is_none() {
            for (file_id, assignments) in variable.assignments {
                other_diagnostics.push(FileDia)
            }
        }
    }
    println!("{:?}", compiler.variables);
    json!({
        parsedContent: compiled.compile(),
        diagnostics: diagnostics.compile()
    })
}

#[tauri::command]
pub fn save_data(state: State<StorytellState>,
    open_panels: Vec<BlobId>,
    open_folders: Vec<BlobId>,
    pinned_panels: Vec<BlobId>,
    last_open: Option<BlobId>
) {
    let mut inner_state = state.lock().unwrap();
    let current_project = inner_state.projects.get_open_project().unwrap();
    current_project.metadata.open_folders = open_folders;
    current_project.metadata.open_panels = open_panels;
    current_project.metadata.pinned_panels = pinned_panels;
    current_project.metadata.last_open = last_open;
    current_project.save();
}

// Returns all the files for the file manager
// Compiles the last opened file if necessary
#[tauri::command]
pub fn init_compiler(state: State<StorytellState>, project_id: String) -> Option<String> {
    let mut inner_state = state.lock().unwrap();
    inner_state.projects.open_project(&project_id);
    let project = inner_state.projects.projects.get(&project_id)?;
    #[cfg(windows)]
    let line_endings = 2;
    #[cfg(not(windows))]
    let line_endings = 1;
    let mut compiler = CompilerWrapper::new(project.files_directory.to_str().unwrap(), line_endings, SysFileHost::default());
    let (global_files, compiled_files) = compiler.inner.init_fs(&mut CompilerWrapper::create_ctx());
    let json_str = json!({
        fileExplorer: json!({
            blobs: format!("{{{}}}", compiler.inner.host.dirs.iter()
                .map(|i| format!("\"{}\":{}", i.0, i.1.borrow().compile()))
                .chain(compiler.inner.host.files.iter()
                    .map(|i| format!("\"{}\":{}", i.0, i.1.borrow().compile())))
                .collect::<Vec<String>>().join(",")),
            global: global_files.compile()
        }),
        contents: compiled_files.compile(),
        openPanels: project.metadata.open_panels.compile(),
        openFolders: project.metadata.open_folders.compile(),
        pinnedPanels: project.metadata.pinned_panels.compile(),
        lastOpen: project.metadata.last_open.compile()
    });
    inner_state.compiler = Some(compiler);
    Some(json_str)
}