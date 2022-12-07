use std::collections::HashSet;

pub use self::file::File;
pub use self::directory::Directory;
pub use self::directory_info::DirectoryInfo;
mod file;
mod directory;
mod directory_info;

// Actions that can be taken in the simplified file system
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ListAll,
    GotoRoot,
    GoUpOne,
    GoDownTo(String)
}

// Represents a filesystem hierarchy
pub struct FileSystem {
    directories: Vec<Directory>,
    current: usize
}

impl FileSystem {
    // Create a new filesystem
    pub fn new() -> Self {
        let root = Directory::new("/");
        Self {
            directories: vec![root],
            current: 0
        }
    }

    // Move up one directory from current
    pub fn go_up_one(&mut self) -> () {
        match self.directories[self.current].parent {
            Some(i) => self.current = i,
            None    => self.current = 0
        };
    }

    // Change to root directory
    pub fn go_to_root(&mut self) -> () {
        self.current = 0
    }

    // Move down a directory - assumes directory actually exists**
    pub fn go_down_to(&mut self, dir: &str) -> () {        
        for i in &self.directories[self.current].directories {
            if self.directories[*i].has_name(dir) {
                self.current = *i;
                break;
            }
        }
    }

    // Add a new directory to the current directory
    pub fn add_directory(&mut self, name: &str) -> () {
        self.directories.push(Directory::with_parent(self.current, name));
        let newest_index = self.directories.len() - 1 as usize;
        self.directories[self.current].directories.push(newest_index);
    }

    // Add a new file to the current directory
    pub fn add_file(&mut self, file: File) -> () {
        self.directories[self.current].files.push(file);
    }

    // Get size of the current directory
    fn get_directory_size(&self, index: usize) -> usize {
        let mut size = self.directories[index].files.iter()
         .map(|x| x.get_size())
         .fold(0, |sum, x| sum + x);
        
        for child in &self.directories[index].directories {
            size += self.get_directory_size(*child);
        }
        return size;
    }

    // Get all directories with size equal to or below the limit
    pub fn get_directories_with_size_below(&self, limit: usize) -> HashSet<DirectoryInfo> {
        let mut set = HashSet::new();
        for i in 0..self.directories.len() {
            let d_size = self.get_directory_size(i);
            if d_size <= limit {
                set.insert(DirectoryInfo{ name: self.directories[i].name.clone(), size: d_size });
            }
        }
        set
    }

    // Get total used size
    pub fn get_used_size(&self) -> usize {
        self.get_directory_size(0)
    }

    // Get first directory bigger than some value
    pub fn get_first_directory_bigger_than(&self, limit: usize) -> DirectoryInfo {
        let mut dirs = Vec::new();
        for i in 0..self.directories.len() {
            let d_size = self.get_directory_size(i);
            if d_size >= limit {
                dirs.push(DirectoryInfo{ name: self.directories[i].name.clone(), size: d_size });
            }
        }
        dirs.sort();
        dirs[0].clone()
    }
}