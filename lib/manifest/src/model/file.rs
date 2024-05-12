#[derive(Debug, Serialize, Deserialize)]
pub struct RootFile {
    pub version: f32,
    pub serial: usize,
    pub toolchain: String, // TODO: use "semantic version" type here, instead of String
    pub ignored_list: Vec<String>,
}

impl Default for RootFile {
    fn default() -> Self {
        Self {
            version: 1.0,
            serial: 0,
            toolchain: "v1.0.0".to_string(),
            ignored_list: Vec::new(),
        }
    }
}
