// Fastest Prober ever seen but this will cause panic(ie) the ip TCP MTU packet will be blocked by your Router due to more number of requests . But this occures for large number of scan files only. 

use reqwest::StatusCode;
use std::io::{BufRead, BufReader};
use tokio::time::Duration;
use futures::future::join_all;
use std::fs::File;
use std::env;
use ansi_term::Color;

async fn status(urls: &[&str]){
    let client = reqwest::client::new();

    let mut tasks = vec![];
    for &url in urls {
        let cloned_client = client.clone();
        let task = async move{
            let full_url = String::from(url);
   
            if !full_url.starts_with("http://") && !full_url.starts_with("https://") {
                let https_url = format!("https://{}", url);
                if let Ok(resp) = cloned_client.get(&https_url).timeout(Duration::from_secs(5)).send().await {
                    match resp.status() {
                        StatusCode::OK => {
                            let status_code : String= StatusCode::OK.to_string();
                            println!("{} -> {:?}", https_url, Color::Green.paint(&status_code));
                        }
                        status => println!("{} -> {:?}", https_url, status),
                    }
                } else {
                    let http_url = format!("http://{}", url);
                    if let Ok(resp) = cloned_client.get(&http_url).timeout(Duration::from_secs(5)).send().await {
                        match resp.status() {
                            StatusCode::OK => println!("{} -> {:?}", http_url, StatusCode::OK),
                            status => println!("{} -> {:?}", http_url, status),
                        }
                    } else {
                        println!("{} -> No such site", full_url);
                    }
                }
            } else {
                let response = tokio::time::timeout(Duration::from_secs(4), cloned_client.get(&full_url).send()).await;
                match response {
                    Ok(result) => {
                        match result {
                            Ok(resp) => {
                                match resp.status() {
                                    StatusCode::OK => println!("{} -> {:?}", full_url, StatusCode::OK),
                                    status => println!("{} -> {:?}", full_url, status),
                                }
                            }
                            Err(_) => println!("{} -> No such site", full_url),
                        }
                    }
                    Err(_) => println!("{} Too Long to respond", full_url),
                }
            }
        };
        tasks.push(task);
    }
    let _results = join_all(tasks).await;
}

#[tokio::main]
async fn main(){
    let args: Vec<_> = env::args().collect();
    
    if args.len() == 1{
        println!("Usage: {} -- urls.txt ",args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];
    let mut urls = Vec::new();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = line.unwrap().trim().to_string();
        if !line.is_empty(){
            urls.push(line);
        }
    }
    // Here im converting the String(Heap) to &str stack to perform well . This is a foolish idea , but im just trying :)
    let url_str = urls.iter().map(|s| s.as_str().collect());

    status(&url_str);
}
