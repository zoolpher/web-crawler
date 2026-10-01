use std::collections::{HashSet, VecDeque};

// We define a struct to hold the "state" of our crawler.
// This remembers where we have been and where we need to go.
pub struct Crawler {
    queue: VecDeque<String>,
    visited: HashSet<String>,
    max_pages: usize,
}

impl Crawler {
    // This function creates a brand new Crawler instance to get us started
    pub fn new(start_url: &str, max_pages: usize) -> Self {
        
        let mut queue = VecDeque::new();
        queue.push_back(start_url.to_string());
        
        let mut visited = HashSet::new();
        visited.insert(start_url.to_string());
        
        Crawler {
            queue,
            visited,
            max_pages,
        }
    }
}
