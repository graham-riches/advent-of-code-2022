use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryInfo {
    pub name: String,
    pub size: usize
}

// Hash directory info only on name
impl Hash for DirectoryInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// Default equals trait
impl Eq for DirectoryInfo {}

// Ordering based only on size, not name
impl Ord for DirectoryInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size.cmp(&other.size)
    }
}

// Partial ordering based on ordering
impl PartialOrd for DirectoryInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

