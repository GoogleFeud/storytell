
use tauri::State;
use crate::{state::StorytellState, projects::Project};
use serde_json::{to_string};

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