use crate::fs::file::File;

// Represents a file system directory
#[derive(Debug, Clone, PartialEq)]
pub struct Directory {
    pub parent: Option<usize>,
    pub name: String,
    pub directories: Vec<usize>,
    pub files: Vec<File>
}

impl Directory {
    // Create a new directory with no parent
    pub fn new(name: &str) -> Self {
        Self {
            parent: None,
            name: name.to_string(),
            directories: Vec::new(),
            files: Vec::new()
        }
    }

    // Create a directory with a parent node
    pub fn with_parent(parent: usize, name: &str) -> Self {
        Self {
            parent: Some(parent),
            name: name.to_string(),
            directories: Vec::new(),
            files: Vec::new()
        }
    }

    // Check if the current directory name matches another
    pub fn has_name(&self, name: &str) -> bool {
        self.name.eq(name)
    }
}