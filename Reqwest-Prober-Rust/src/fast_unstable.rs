// Using rayon to make multiple request , but its unstable for large scans like above 200 Sites in 200.txt files .. etc 

use reqwest::StatusCode;
use std::env;
use rayon::prelude::*;

fn handle_request(url: &str) -> Result<(String, StatusCode), String> {
    let client = reqwest::blocking::Client::new();
    match client.get(url).send() {
        Ok(resp) => {
            let status = resp.status();
            let final_url = resp.url().to_string();
            match status {
                StatusCode::OK => Ok((url.to_string(), status)),
                StatusCode::FOUND | StatusCode::MOVED_PERMANENTLY => {
                    Ok((format!("{} is redirected to {}", url, final_url), status))
                }
                _ => Err(format!("{} -> {:?}", url, status)),
            }
        }
        Err(_) => Err(format!("{} -> No such site", url)),
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let mut urls = Vec::new();
    if let Ok(file) = std::fs::read_to_string(file_path) {
        for line in file.lines() {
            if !line.trim().is_empty() {
                urls.push(line.trim().to_string());
            }
        }
    } else {
        eprintln!("Failed to read file: {}", file_path);
        std::process::exit(1);
    }

    urls.par_iter().for_each(|url| {
        match handle_request(url) {
            Ok((msg, status)) => {
                if status.is_redirection() {
                    println!("{}", msg);
                } else {
                    println!("{} -> {:?}", url, status);
                }
            }
            Err(err) => println!("{}", err),
        }
    });
}
