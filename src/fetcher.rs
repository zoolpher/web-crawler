use scraper::{Html, Selector};
use url::Url;

// This function takes a URL, fetches the HTML, and returns a list of absolute links found on that page.
// The "pub" keyword makes this function public so other files can use it.
pub async fn fetch_links(current_url: &str) -> Result<Vec<String>, String> {
    
    // We use reqwest to download the page and map any error into a simple string
    let response = reqwest::get(current_url)
        .await
        .map_err(|e| format!("Failed to fetch: {}", e))?;
        
    let html = response.text()
        .await
        .map_err(|e| format!("Failed to read HTML: {}", e))?;
        
    let document = Html::parse_document(&html);
    
    // Find all anchor tags (<a>) in the document
    let link_selector = Selector::parse("a").unwrap();
    
    // Parse the current URL so we can use it to resolve any relative links later
    let base_url = Url::parse(current_url)
        .map_err(|e| format!("Invalid URL: {}", e))?;
        
    let mut found_links = Vec::new();
    
    // Iterate over all anchor tags found in the HTML
    for element in document.select(&link_selector) {
        if let Some(href) = element.value().attr("href") {
            
            // Convert relative links (like /about) into absolute links using the base URL
            if let Ok(absolute_url) = base_url.join(href) {
                found_links.push(absolute_url.to_string());
            }
        }
    }
    
    Ok(found_links)
}
