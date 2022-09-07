use directories::UserDirs;
use std::collections::HashMap;
use std::path::{PathBuf, Path};
use uuid::Uuid;
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json::{from_slice, to_string};

#[derive(Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: String,
    pub id: String
}

#[derive(Serialize, Deserialize)]
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
                    projects.insert(project_info.id.clone(), Project {
                        metadata: project_info,
                        files_directory: content_dir,
                        directory: project_dir
                    });
                }
            }
        }
        Self { 
            storytell_dir,
            projects
        }
    }

    pub fn create_project(&mut self, name: String, description: String) -> Option<&Project> {
        if self.projects.contains_key(&name) {
            None
        } else {
            let project_id = Uuid::new_v4().to_string();
            let project_dir = self.storytell_dir.join(project_id.clone());
            fs::create_dir(&project_dir).expect("Failed to create directory.");
            let files_dir = project_dir.join("./content");
            fs::create_dir(&files_dir).expect("Failed to create directory.");
            let project_info = ProjectMetadata {
                id: project_id.clone(),
                name: name.clone(),
                description
            };
            fs::write(project_dir.join("./metadata.json"), to_string(&project_info).unwrap()).expect("Couldn't create file.");
            self.projects.insert(project_id, Project {
                metadata: project_info,
                files_directory: files_dir,
                directory: project_dir
            });
            self.projects.get(&name)
        }
    }

    pub fn update_project(&mut self, id: String, name: String, description: String) -> Option<()> {
        let project = self.projects.get_mut(&id)?;
        project.metadata.name = name;
        project.metadata.description = description;
        fs::write(project.directory.join("./metadata.json"), to_string(&project.metadata).unwrap()).expect("Couldn't write to file.");
        Some(())
    }

    pub fn delete_project(&mut self, id: &str) -> bool {
        if let Some(project) = self.projects.remove(id) {
            fs::remove_dir_all(project.directory).expect("Failed to delete directory.");
            true
        } else {
            false
        }
    }

}