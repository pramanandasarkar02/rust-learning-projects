use reqwest;


fn scrapper(url: String) -> Option<String>{
    match reqwest::blocking::get(url) {
        Ok(response) => {
            if response.status().is_success() {
                match response.text() {
                    Ok(body) => {
                        println!("Content fetched {}", body);
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


fn main() {
    let website_url = String::from("https://webscraper.io/test-sites/e-commerce/static");
    println!("Scrapping webpage(recussive web-page): {}", website_url);

    if let Some(html) = scrapper(website_url) {
        println!("Html length:{} bytes", html.len())
    } else {
        println!("No content fetched")
    }
}
