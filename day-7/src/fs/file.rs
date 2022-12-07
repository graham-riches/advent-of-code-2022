#[derive(Debug, Clone, PartialEq)]
pub struct File {
    name: String,
    size: usize
}

impl File{
    // Create a new file object
    pub fn new(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: size
        }
    }

    // Get the size of the current file
    pub fn get_size(&self) -> usize {
        self.size
    }
}