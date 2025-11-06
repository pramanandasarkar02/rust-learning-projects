use std::{collections::{HashSet, VecDeque}, io::{Write, stdin, stdout}};

use reqwest;
use scraper::{Html, Selector};


fn scraper(url: &str) -> Option<String>{
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
    // let website_url = String::from("https://webscraper.io/test-sites/e-commerce/static");
    // let max_depth = 2; 

    let mut websit_url_str = String::new();
    let mut depth_str = String::new();
    print!("Enter website url: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut websit_url_str).unwrap();
    let website_url = websit_url_str.trim();

    print!("Enter depth: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut depth_str).unwrap();
    let max_depth = depth_str.trim().parse().expect("need a Integer");
    


    println!("Scrapping webpage(recussive web-page): {}", website_url);

    let mut scrapped = HashSet::new();
    let mut failed_scraped = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((website_url.to_string(), 0));

    while let Some((current_url, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }
        if scrapped.contains(&current_url) {
            continue;
        }
        println!("Depth: {} | Scrapping: {}", depth, current_url);
        match scraper(&current_url) {
            Some(html) => {
                println!("😃😃😃 fetched {} bytes", html.len());
                if depth < max_depth {
                    let links = extract_links(&html, &current_url);
                    for link in links {
                        if !scrapped.contains(link.as_str()) && !failed_scraped.contains(link.as_str()) {
                            queue.push_back((link.clone(), depth + 1));
                        }
                    }
                }
            }
            None => {
                eprintln!("😰😰😰 Failed to fetch: {}", current_url);
                failed_scraped.insert(current_url.clone());
            }
        }
        scrapped.insert(current_url.clone());
    }

    println!("\t\tScrapping completed. \ntotal succefully of scrapped url count: {}", scrapped.len());
    println!("total failed scrapped url count: {}", failed_scraped.len());
    
}
