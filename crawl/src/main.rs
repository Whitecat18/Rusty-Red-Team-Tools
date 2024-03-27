// use std::collections::HashSet;
// use std::sync::Arc;
// use select::document;
// use tokio::sync::{mpsc,Semaphore};
// use tokio::time::{sleep, Duration};
// use reqwest::Client;
// use url::{Url, ParseError};
// use html5ever::parse_document;
// use colored::*;
// use std::io::{stdin, stdout, Write};

// const MAX_DEPTH: u32 = 3;
// const CONCURRENT_REQUESTS: usize = 10;
// const CRAWL_DELAY: u64 = 1; // In seconds

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;


// async fn fetch(url: &str, client: &Client) -> Result<String>{
//     let response = client.get(url).send().await?;
//     let html = response.text().await?;
//     Ok(html)
// }

// fn get_internal_links(url: &str, html_content: &str) -> Result<HashSet<String>>{
//     let base_url = Url::parse(url);
//     let document = 
// }

// fn main(){
    
// }


// use std::collections::{HashSet, VecDeque};
// use std::time::Instant;
// use reqwest::Url;
// use select::document::Document;
// use select::predicate::Name;


// const MAX_DEPTH: usize = 3;
// const CONCURRENT_REQUESTS: usize = 10;
// const CRAWL_DELAY: u64 = 1;

// fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
//     println!("{}", prompt);
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input)?;
//     Ok(input.trim().to_string())
// }

// async fn fetch(url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
//     let response = reqwest::get(url).await?;
//     if response.status().is_success() {
//         let html_content = response.text().await?;
//         Ok(Some(html_content))
//     } else {
//         Ok(None)
//     }
// }

// fn find_forms(html_content: &str) -> Option<()> {
//     let document = Document::from(html_content);
//     if document.find(Name("form")).next().is_some() {
//         Some(())
//     } else {
//         let has_input_text = document.find(Name("input"))
//             .filter_map(|n| n.attr("type"))
//             .any(|t| t == "text");
//         if has_input_text {
//             Some(())
//         } else {
//             None
//         }
//     }
// }

// fn get_internal_links(url: &str, html_content: &str) -> Vec<String> {
//     let base_url = Url::parse(url).unwrap();
//     let document = Document::from(html_content);
//     let mut internal_links = Vec::new();
//     for node in document.find(Name("a")).filter_map(|n| n.attr("href")) {
//         if let Ok(link) = base_url.join(node) {
//             if link.scheme() == "https" && link.host_str() == base_url.host_str() {
//                 internal_links.push(link.to_string());
//             }
//         }
//     }
//     internal_links
// }


// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>>{
//     let start_time = Instant::now();
//     let mut visited_urls = HashSet::new();
//     let mut pending_urls = VecDeque::new();

//     let domain = get_user_input("Enter the domain you want to crawl (e.g., example.com): ");
//     let starting_url = format!("https://{:?}",domain);
//     pending_urls.push_back((starting_url.clone(), 0));
    
//     while let Some((url, depth)) = pending_urls.pop_front() {
//         if depth > MAX_DEPTH || visited_urls.contains(&url) {
//             continue;
//         }

//         visited_urls.insert(url.clone());
//         let html_content = fetch(&url).await?;
//         if let Some(html_content) = html_content {
//             if let Some(forms) = find_forms(&html_content) {
//                 println!("\x1B[92mForm Found\x1B[0m: {}", url); // Green color for "Form Found" message
//             }
//             let internal_links = get_internal_links(&url, &html_content);
//             for link in internal_links {
//                 pending_urls.push_back((link, depth + 1));
//             }
//         }
//         tokio::time::sleep(tokio::time::Duration::from_secs(CRAWL_DELAY)).await;
//     }
//     println!("Execution time: {:.2} seconds", start_time.elapsed().as_secs_f64());
//     Ok(())
// }

// Code Test -> Slow case 
// use std::collections::{HashSet, VecDeque};
// use std::time::Instant;
// use reqwest::Url;
// use select::document::Document;
// use select::predicate::Name;

// const MAX_DEPTH: usize = 3;
// const CONCURRENT_REQUESTS: usize = 10;
// const CRAWL_DELAY: u64 = 1;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let start_time = Instant::now();

//     let mut visited_urls = HashSet::new();
//     let mut pending_urls = VecDeque::new();

//     let domain = get_user_input("Enter the domain you want to crawl (e.g., example.com): ")?;
//     let starting_url = format!("https://{}", domain);
//     pending_urls.push_back((starting_url.clone(), 0));

//     while let Some((url, depth)) = pending_urls.pop_front() {
//         if depth > MAX_DEPTH || visited_urls.contains(&url) {
//             continue;
//         }

//         visited_urls.insert(url.clone());
//         let html_content = fetch(&url).await?;
//         if let Some(html_content) = html_content {
//             if let Some(_) = find_forms(&html_content) {
//                 println!("\x1B[92mForm Found\x1B[0m: {}", url); // Green color for "Form Found" message
//             }
//             let internal_links = get_internal_links(&url, &html_content);
//             for link in internal_links {
//                 pending_urls.push_back((link, depth + 1));
//             }
//         }
//         tokio::time::sleep(tokio::time::Duration::from_secs(CRAWL_DELAY)).await;
//     }

//     println!("Execution time: {:.2} seconds", start_time.elapsed().as_secs_f64());
//     Ok(())
// }

// fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
//     println!("{}", prompt);
//     let mut input = String::new();
//     std::io::stdin().read_line(&mut input)?;
//     Ok(input.trim().to_string())
// }

// async fn fetch(url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
//     let response = reqwest::get(url).await?;
//     if response.status().is_success() {
//         let html_content = response.text().await?;
//         Ok(Some(html_content))
//     } else {
//         Ok(None)
//     }
// }

// fn find_forms(html_content: &str) -> Option<()> {
//     let document = Document::from(html_content);
//     if document.find(Name("form")).next().is_some() {
//         Some(())
//     } else {
//         let has_input_text = document.find(Name("input"))
//             .filter_map(|n| n.attr("type"))
//             .any(|t| t == "text");
//         if has_input_text {
//             Some(())
//         } else {
//             None
//         }
//     }
// }

// fn get_internal_links(url: &str, html_content: &str) -> Vec<String> {
//     let base_url = Url::parse(url).unwrap();
//     let document = Document::from(html_content);
//     let mut internal_links = Vec::new();
//     for node in document.find(Name("a")).filter_map(|n| n.attr("href")) {
//         if let Ok(link) = base_url.join(node) {
//             if link.scheme() == "https" && link.host_str() == base_url.host_str() {
//                 internal_links.push(link.to_string());
//             }
//         }
//     }
//     internal_links
// }

/*
Crawls HTML forms and input text fields on the visited pages
By 5mukx -> https://github.com/Whitecat18
*/

use std::collections::{HashSet, VecDeque};
use std::io::Write;
use std::time::Instant;
use reqwest::Url;
use select::document::Document;
use select::predicate::Name;

const MAX_DEPTH: usize = 3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let mut visited_urls = HashSet::new();
    let mut pending_urls = VecDeque::new();

    let domain = get_user_input("Enter the domain you want to crawl [Eg: example.com] :")?;
    let starting_url = format!("https://{}", domain);
    pending_urls.push_back((starting_url.clone(), 0));

    while let Some((url, depth)) = pending_urls.pop_front() {
        if depth > MAX_DEPTH || visited_urls.contains(&url) {
            continue;
        }

        visited_urls.insert(url.clone());
        let html_content = fetch(&url).await?;
        if let Some(html_content) = html_content {
            if let Some(_) = find_forms(&html_content) {
                println!("\x1B[92mForm Found\x1B[0m: {}", url); // Green color for "Form Found" message
            }
            let internal_links = get_internal_links(&url, &html_content);
            for link in internal_links {
                pending_urls.push_back((link, depth + 1));
            }
        }
    }

    println!("Execution time: {:.2} seconds", start_time.elapsed().as_secs_f64());
    Ok(())
}

fn get_user_input(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

async fn fetch(url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let html_content = response.text().await?;
        Ok(Some(html_content))
    } else {
        Ok(None)
    }
}

fn find_forms(html_content: &str) -> Option<()> {
    let document = Document::from(html_content);
    if document.find(Name("form")).next().is_some() {
        Some(())
    } else {
        let has_input_text = document.find(Name("input"))
            .filter_map(|n| n.attr("type"))
            .any(|t| t == "text");
        if has_input_text {
            Some(())
        } else {
            None
        }
    }
}

fn get_internal_links(url: &str, html_content: &str) -> Vec<String> {
    let base_url = Url::parse(url).unwrap();
    let document = Document::from(html_content);
    let mut internal_links = Vec::new();
    for node in document.find(Name("a")).filter_map(|n| n.attr("href")) {
        if let Ok(link) = base_url.join(node) {
            if link.scheme() == "https" && link.host_str() == base_url.host_str() {
                internal_links.push(link.to_string());
            }
        }
    }
    internal_links
}
