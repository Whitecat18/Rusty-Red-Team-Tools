// Using Reqwst and Tokio to reqwest and checks if the website has redirection !! < Fast , and Stable > !! 
// Using Buffers to collect and keeps the threads normal !

// CURRENT REQWEST PROBER MAIN FILE


use reqwest::StatusCode;
use std::env;
use tokio::time::{Duration, timeout};
use futures::stream::StreamExt;

async fn handle_request(url: &str) -> Result<(String, StatusCode), String> {
    let client = reqwest::Client::new();
    let response = timeout(Duration::from_secs(5), client.get(url).send()).await;

    match response {
        Ok(result) => {
            match result {
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
        Err(_) => Err(format!("{} -> Too Long to respond", url)),
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let mut urls = Vec::new();
    if let Ok(file) = tokio::fs::read_to_string(file_path).await {
        for line in file.lines() {
            if !line.trim().is_empty() {
                let trimmed = line.trim();
                if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
                    urls.push(format!("https://{}", trimmed));
                    urls.push(format!("http://{}", trimmed));
                } else {
                    urls.push(trimmed.to_string());
                }
            }
        }
    } else {
        eprintln!("Failed to read file: {}", file_path);
        std::process::exit(1);
    }

    let _results = futures::stream::iter(urls)
        .map(|url| async move {
            match handle_request(&url).await {
                Ok((msg, status)) => {
                    if status.is_redirection() {
                        println!("{}", msg);
                    } else {
                        println!("{} -> {:?}", url, status);
                    }
                }
                Err(err) => println!("{}", err),
            }
        })
        .buffer_unordered(50)
        .collect::<Vec<_>>()
        .await;
}
