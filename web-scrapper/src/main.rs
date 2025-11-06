use reqwest;
use scraper::{Html, Selector};


fn scrapper(url: &str) -> Option<String>{
    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.text() {
                    Ok(body) => {
                        Some(body)
                    }
                    Err(e) => {
                        eprintln!("Failed to read from body, err: {}", e);
                        None 
                    }
                }
            } else {
                eprintln!("Server response with error: {}", response.status());
                None
            }
        }
        Err(e) => {
            eprintln!("Request failed for error {}", e);
            None
        }
    }
}



fn extract_links(html: &str, base_url: &str) -> Vec<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();

    document
        .select(&selector)
        .filter_map(|element|element.value().attr("href"))
        .map(|href| {
            if href.starts_with("http") {
                href.to_string()
            } else if href.starts_with('/') {
                format!("{}{}", base_url.trim_end_matches('/'), href)
            } else {
                format!("{}/{}", base_url.trim_end_matches('/'), href)
            }
        })
        .collect()
        
}

fn main() {
    let website_url = String::from("https://webscraper.io/test-sites/e-commerce/static");
    println!("Scrapping webpage(recussive web-page): {}", website_url);

    if let Some(html) = scrapper(website_url.as_str()) {
        println!("Html length:{} bytes", html.len());
        let links = extract_links(html.as_str(), website_url.as_str());
        for link in links{
            println!("link: {}",link);
        }
    } else {
        println!("No content fetched")
    }
}
