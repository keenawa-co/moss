mod project_service;

#[derive(Serialize)]
pub struct Project {
    name: String,
}

pub struct ProjectService {
    db: String,
}

impl ProjectService {
    pub fn new(db: String) -> Self {
        Self { db }
    }

    pub fn create_project(&self, name: String) -> Result<Project, String> {
        println!("{name}, {}", self.db);
        Ok(Project { name })
    }

    pub fn delete_project(&self, name: String) -> Result<(), String> {
        Ok(())
    }
}
