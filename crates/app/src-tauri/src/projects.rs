use directories::UserDirs;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_string};

#[derive(Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: String
}

pub struct Project {
    pub metadata: ProjectMetadata,
    pub directory: PathBuf,
    pub files_directory: PathBuf
}

pub struct Projects {
    pub storytell_dir: PathBuf,
    pub projects: HashMap<String, Project>
}

impl Projects {
    pub fn new() -> Self {
        let docs_dir = UserDirs::new().expect("Couldn't load file system.").document_dir().expect("Couldn't find a documents directory.").to_path_buf();
        let storytell_dir = docs_dir.join("./Storytell");
        if !Path::exists(storytell_dir.as_path()) {
            fs::create_dir(&storytell_dir).expect("Couldn't create Storytell folder.");
        }

        let mut projects: HashMap<String, Project> = HashMap::new();
        for file in fs::read_dir(&storytell_dir).unwrap().flatten() {
            if file.file_type().unwrap().is_dir() {
                let project_dir = file.path();
                let content_dir = project_dir.join("./content");
                if !Path::is_dir(&content_dir) {
                    continue;
                }
                if let Ok(content) = fs::read(project_dir.join("./metadata.json")) {
                    let project_info = from_slice::<ProjectMetadata>(content.as_slice()).expect("Invalid JSON.");
                    projects.insert(project_info.name.clone(), Project {
                        metadata: project_info,
                        files_directory: content_dir,
                        directory: project_dir
                    });
                }
            }
        }
        Self { 
            storytell_dir,
            projects: HashMap::new()
        }
    }

    pub fn create_project(&mut self, name: String, description: String) -> bool {
        if self.projects.contains_key(&name) {
            false
        } else {
            let project_dir = self.storytell_dir.join(name.clone());
            fs::create_dir(&project_dir).expect("Failed to create directory.");
            let files_dir = project_dir.join("./content");
            fs::create_dir(&files_dir).expect("Failed to create directory.");
            let project_info = ProjectMetadata {
                name: name.clone(),
                description
            };
            fs::write(project_dir.join("./metadata.json"), to_string(&project_info).unwrap()).expect("Couldn't create file.");
            self.projects.insert(name, Project {
                metadata: project_info,
                files_directory: files_dir,
                directory: project_dir
            });
            true
        }
    }

    pub fn delete_project(&mut self, name: &str) -> bool {
        if let Some(project) = self.projects.remove(name) {
            fs::remove_dir(project.directory).expect("Failed to delete directory.");
            true
        } else {
            false
        }
    }

}