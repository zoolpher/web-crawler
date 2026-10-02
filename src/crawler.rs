use std::collections::{HashSet, VecDeque};
use tokio::task::JoinSet;

// Import our fetcher module so we can use its functions here
use crate::fetcher::fetch_links;

// We define a struct to hold the "state" of our crawler.
pub struct Crawler {
    queue: VecDeque<String>,
    visited: HashSet<String>,
    max_pages: usize,
}

impl Crawler {
    
    // This function creates a brand new Crawler instance
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

    // This is the main engine of our crawler
    pub async fn run(&mut self) {
        let mut pages_crawled = 0;
        println!("Starting crawl...");

        // Keep going until the queue is empty or we hit our page limit
        while !self.queue.is_empty() && pages_crawled < self.max_pages {
            
            // JoinSet manages our concurrent background tasks
            let mut join_set = JoinSet::new();
            
            let batch_size = 5;
            let mut launched = 0;

            // Pull URLs from the front of the queue to create a batch
            while let Some(url) = self.queue.pop_front() {
                
                // Spawn a background task for each URL
                join_set.spawn(async move {
                    fetch_links(&url).await
                });
                
                launched += 1;
                pages_crawled += 1;
                
                if launched >= batch_size || pages_crawled >= self.max_pages {
                    break;
                }
            }

            // Wait for all tasks in the batch to finish
            while let Some(result) = join_set.join_next().await {
                
                // If the task didn't crash, and the fetcher didn't return an error
                if let Ok(Ok(new_links)) = result {
                    
                    // Check each new link we discovered
                    for link in new_links {
                        
                        // If we haven't visited it yet, remember it and queue it up
                        if !self.visited.contains(&link) {
                            self.visited.insert(link.clone());
                            self.queue.push_back(link);
                        }
                    }
                }
            }
        }

        println!("\n--- Crawl Finished ---");
        println!("Total pages visited: {}", pages_crawled);
        println!("Total unique links discovered: {}", self.visited.len());
    }
}
