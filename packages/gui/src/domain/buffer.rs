use std::path::PathBuf;

pub(crate) struct Buffer {
    path: PathBuf,
    contents: Vec<u8>,
}

impl Buffer {
    pub fn get_file_name(&self) -> String {
        let name = self.path.file_name().expect("Failed to get file name");
        name.to_string_lossy().to_string()
    }

    pub fn new(path: PathBuf, contents: Vec<u8>) -> Self {
        Self { path, contents }
    }
}
