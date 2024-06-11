// THIS IS MY OLD main.rs , Here you can see the progress of this reqwest prober from scratch , trying varioud crates to make the program fast , flexible and without making it panic! .


// use std::thread;
// use std::sync::mpsc;
//
//
// // USING THREADS USING MPCS TO SEND AND RECIEVE THE MESSAGES . THIS THREAD CREATED AND DESTROYED .
// #[warn(unused_variables)]
// fn status(urls : &[&str]){
//     let (send, recieve) = mpsc::channel();
//
//     for &url in urls{
//         let send = send.clone();
//         let url = url.to_string();
//
//         thread::spawn(move || {
//             match reqwest::blocking::get(&url){
//                 Ok(resp) => {
//                     match resp.status(){
//                         StatusCode::OK => send.send((url, StatusCode::OK)).unwrap(),
//                         status => send.send((url, status)).unwrap(),
//                     } 
//                 }
//                 Err(_err) => send.send((url, StatusCode::SERVICE_UNAVAILABLE)).unwrap(),
//             }
//         });
//     }
//
//     drop(send);
//
//     for  (url,status) in recieve{
//         match status{
//             StatusCode::OK => println!("{} -> {:?}", url , StatusCode::OK),
//             status => println!("{} -> {:?}", url , status),
//         }
//     }
// }

// USIG RYON !!
// use colored::Colorize;
// use reqwest::StatusCode;
// use rayon::prelude::*;
//
//
// fn status(urls: &[&str]){
//     urls.par_iter().for_each(|url| {
//         match reqwest::blocking::get(*url){
//             Ok(resp) => {
//                 match resp.status(){
//                     StatusCode::OK => println!("{} -> {:?}" ,url.green() , StatusCode::OK),
//                     status => println!("{} -> {:?}", url.red() , status),
//                 }
//             }
//             Err(_) => println!("Error {} -> No Such Domain ⚠️",url.yellow()),
//         }
//     });
// }

// USING TOKYO LIBRARY -- ooh soo fast as 

// Old Function 

// use reqwest::StatusCode;
// use tokio::time::{self, Duration};
// use futures::future::join_all;
//
// async fn status(urls: &[&str]){
//     let client = reqwest::Client::new();
//
//     let mut tasks = vec![];
//     for &url in urls {
//         let client = client.clone();
//         let task = async move{
//             match client.get(url).send().await{
//                 Ok(resp) => {
//                     match resp.status() {
//                         StatusCode::OK => println!("{} -> {:?}", url , StatusCode::OK),
//                         status => println!("{} -> {:?}", url ,status),
//                     }
//                 }
//                 Err(_) => println!("{} - !! There is no such domain ", url),
//             }
//         };
//         tasks.push(task);
//     }
//         
//     let results = join_all(tasks).await;
//     //let results = futures::future::join_all(tasks).await;
// }


// With function response Program !

// use reqwest::StatusCode;
// use tokio::time::{self, Duration};
// use futures::future::join_all;
//
// async fn status(urls: &[&str]) {
//     let client = reqwest::Client::new();
//
//     let mut tasks = vec![];
//     for &url in urls {
//         let cloned_client = client.clone();
//
//         let task = async move {
//             let response = tokio::time::timeout(Duration::from_secs(4), cloned_client.get(url).send()).await;
//
//             match response {
//                 Ok(result) => {
//                     match result {
//                         Ok(resp) => {
//                             match resp.status() {
//                                 StatusCode::OK => println!("{} -> {:?}", url, StatusCode::OK),
//                                 status => println!("{} -> {:?}", url, status),
//                             }
//                         }
//                         Err(_) => println!("{} -> No Such Website", url),
//                     }
//                 }
//                 Err(_) => println!("{} took too long to respond!", url),
//             }
//         };
//
//         tasks.push(task);
//     }
//
//     let results = join_all(tasks).await;
// }

// use reqwest::StatusCode;
// use std::io::{BufRead, BufReader};
// use tokio::time::{self, Duration};
// use futures::{future::join_all};
// use std::env;
// use std::fs::File;


// async fn status(urls: &[&str]) {
//     let client = reqwest::Client::new();

//     let mut tasks = vec![];
//     for &url in urls {
//         let cloned_client = client.clone();

//         let task = async move {
//             let response = tokio::time::timeout(Duration::from_secs(2), cloned_client.get(url).send()).await;

//             match response {
//                 Ok(result) => {
//                     match result {
//                         Ok(resp) => {
//                             match resp.status() {
//                                 StatusCode::OK => println!("{} is active. Status code: {:?}", url, StatusCode::OK),
//                                 status => println!("{} returned status code: {:?}", url, status),
//                             }
//                         }
//                         Err(err) => println!("Error connecting to {}: {:?}", url, err),
//                     }
//                 }
//                 Err(_) => println!("{} took too long to respond!", url),
//             }
//         };

//         tasks.push(task);
//     }

//     let results = join_all(tasks).await;
// }




// #[tokio::main]
// async fn main(){

//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         panic!("Usage: {} <file_path>", args[0]);
//     }

//     let file_path = &args[1];
    
//     let mut url_string = String::new();
//     for url in urls {
//         url_string.push_str(url);
//         url_string.push(','); 
//     }

//     let file = File::open(file_path).unwrap();
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         let line = line.unwrap();
//         if line.starts_with("http"){
//             urls.push(line.as_str());
//         }
//     }

//     status(urls.to_str()).await;
// }

// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use std::process;
//

// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use reqwest::StatusCode;
// use rayon::prelude::*;
// use tokio::time::{self, Duration};
//
// async fn status(urls: Vec<String>){
//     let client = reqwest::Client::new();
//
//     let task: Vec<_> = urls
//         .par_iter()
//         .map(|url| {
//             let clone_client = client.clone();
//             tokio::spawn(async move {
//                 match clone_client.get(url).send().await{
//                     Ok(resp) => {
//                         match resp.status(){
//                             StatusCode::OK => println!("{} -> {:?}", url, StatusCode::OK),
//                             status => println!("{} -> {:?}",url, status),
//                         }
//                     }
//                     Err(_) => println!("Unable to request the Site : {}", url),
//                 }
//             })
//         })
//         .collect();
//
//     for proc in task {
//         proc.await.expect("Failed");
//     }
// }
//
//
//
// #[tokio::main]
//
// async fn main(){
//     let args: Vec<String> = env::args().collect();
//
//     if args.len() != 2{
//         println!("USAGE 2 ");
//         std::process::exit(1);
//     }
//
//     let file_path = &args[1];
//     let mut urls = Vec::new();
//     
//     let file = File::open(file_path).unwrap();
//     let content = BufReader::new(file);
//
//     for line in content.lines() {
//         let line = line.unwrap();
//         if line.starts_with("http"){
//             urls.push(line);
//         }
//     }
//
//     status(urls).await;
//     
//
// }

// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use reqwest::StatusCode;
// use rayon::prelude::*;
// use tokio::time::{self, Duration};

// async fn status<'a>(urls: Vec<String>, client: reqwest::Client) {
//     let task: Vec<_> = urls
//         .par_iter()
//         .map(|url| {
//             tokio::spawn(async move {
//                 match client.get(url.into()).send().await {
//                     Ok(resp) => {
//                         match resp.status() {
//                             StatusCode::OK => println!("{} -> {:?}", url, StatusCode::OK),
//                             status => println!("{} -> {:?}", url, status),
//                         }
//                     }
//                     Err(_) => println!("Unable to request the Site : {}", url),
//                 }
//             })
//         })
//         .collect();

//     for proc in task {
//         proc.await.expect("Failed");
//     }
// }

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         println!("USAGE 2 ");
//         std::process::exit(1);
//     }

//     let file_path = &args[1];
//     let mut urls = Vec::new();

//     let file = File::open(file_path).unwrap();
//     let content = BufReader::new(file);

//     for line in content.lines() {
//         let line = line.unwrap();
//         if line.starts_with("http") {
//             urls.push(line);
//         }
//     }

//     let client = reqwest::Client::new();
//     status(urls, client).await;
// }

// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use reqwest::StatusCode;
// use rayon::prelude::*;
// use tokio::time::{self, Duration};

// async fn status<'a>(urls: Vec<String>, client: reqwest::Client) {
//     let tasks: Vec<_> = urls
//         .par_iter()
//         .map(|url| {
//             let client = client.clone(); // Clone the client for each task
//             async move {
//                 match client.get(url).send().await {
//                     Ok(resp) => {
//                         match resp.status() {
//                             StatusCode::OK => println!("{} -> {:?}", url, StatusCode::OK),
//                             status => println!("{} -> {:?}", url, status),
//                         }
//                     }
//                     Err(_) => println!("Unable to request the Site : {}", url),
//                 }
//             }
//         })
//         .collect();

//     // Await all tasks
//     for task in tasks {
//         task.await;
//     }
// }

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() != 2 {
//         println!("USAGE: <program> <file_path>");
//         std::process::exit(1);
//     }

//     let file_path = &args[1];
//     let mut urls = Vec::new();

//     let file = File::open(file_path).unwrap();
//     let content = BufReader::new(file);

//     for line in content.lines() {
//         let line = line.unwrap();
//         if line.starts_with("http") {
//             urls.push(line);
//         }
//     }

//     let client = reqwest::Client::new();
//     status(urls, client).await;
// }

// #[warn(unused_imports)]
// use reqwest::StatusCode;
// use std::io::{BufRead, BufReader};
// use tokio::time::{Duration};
// use futures::{future::join_all};
// use std::env;
// use std::fs::File;
//

// use ping::{Ping, PingResult};
// use std::io::{BufRead, BufReader};
// use tokio::time::{self, Duration};
// use futures::{future::join_all};
// use std::env;
// use std::fs::File;

// use core::slice::SlicePattern;

// async fn ping_urls(urls: &[&str]){
//     let mut tasks = vec![];

//     for &url in urls {
//         let ping = Ping::new();

//         let task = async move{
//             match ping.ping.ping(url , 4){
//                 Ok(PingResult::Reply(_)) => println!("{} -> Alive", url),
//                 Ok(PingResult::TimeOut) => println!("{} -> Timeout", url),
//                 Err(e) => println!("{} -> Error occures {}", url , e),
//             }
//         };
//         tasks.push(task);
//     }
//     let results = join_all(tasks).await;
// }


// async fn status(urls: &[&str]) {
//     let client = reqwest::Client::new();

//     let mut tasks = vec![];
//     for &url in urls {
//         let mut full_url = String::new();
//         full_url.push_str("https://");
//         full_url.push_str(url);

//         let cloned_client = client.clone();

//         let task = async move {
//             let response = tokio::time::timeout(Duration::from_secs(4), cloned_client.get(url).send()).await;

//             match response {
//                 Ok(result) => {
//                     match result {
//                         Ok(resp) => {
//                             match resp.status() {
//                                 StatusCode::OK => println!("{} -> {:?}", url, StatusCode::OK),
//                                 status => println!("{} -> {:?}", url, status),
//                             }
//                         }
//                         Err(_) => println!("{} -> No such site", url),
//                     }
//                 }
//                 Err(_) => println!("{} Too Long to respond", url),
//             }
//         };

//         tasks.push(task);
//     }

//     let _results = join_all(tasks).await;
// }

#[warn(unused_imports)]
use reqwest::StatusCode;
use std::io::{BufRead, BufReader};
use tokio::time::Duration;
use futures::future::join_all;
use std::fs::File;
use std::env;
use ansi_term::Color;
// async fn status(urls: &[&str]) {
//     let client = reqwest::Client::new();
//
//     let mut tasks = vec![];
//     for &url in urls {
//         let cloned_client = client.clone();
//         let task = async move {
//             let mut full_url = String::from(url);
//             if !full_url.starts_with("http://") && !full_url.starts_with("https://") {
//                 full_url = format!("https://{}", url);
//             }
//
//             let response = tokio::time::timeout(Duration::from_secs(4), cloned_client.get(&full_url).send()).await;
//
//             match response {
//                 Ok(result) => {
//                     match result {
//                         Ok(resp) => {
//                             match resp.status() {
//                                 StatusCode::OK => println!("{} -> {:?}", full_url, StatusCode::OK),
//                                 status => println!("{} -> {:?}", full_url, status),
//                             }
//                         }
//                         Err(_) => println!("{} -> No such site", full_url),
//                     }
//                 }
//                 Err(_) => println!("{} Too Long to respond", full_url),
//             }
//         };
//
//         tasks.push(task);
//     }
//
//     let _results = join_all(tasks).await;
// }

// New format to Fix the ping error 
//
// use reqwest::StatusCode;
// use tokio::time::Duration;
// use futures::future::join_all;

async fn status(urls: &[&str]) {
    let client = reqwest::Client::new();

    let mut tasks = vec![];
    for &url in urls {
        let cloned_client = client.clone();
        let task = async move {
            let full_url = String::from(url);
            
            if !full_url.starts_with("http://") && !full_url.starts_with("https://") {
                // HTTPS first
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
                    // If HTTPS fails, try HTTP
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
async fn main() {
    let args: Vec<_> = env::args().collect();
    
    // Lets Add Command Line arguments !
    if args.len() ==  1 {
        println!("Usage: {} <file_path> -- arguments ", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];

    let mut urls = Vec::new();
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap().trim().to_string();
        if !line.is_empty(){
            urls.push(line);
        }
    }
    let mut proper_url = Vec::new();
    for url in urls{
        if !url.starts_with("http://") && !url.starts_with("https://"){
            proper_url.push(format!("http://{}", &url));
            proper_url.push(format!("https://{}",&url));
        } else {
            proper_url.push(url);
        }
    }

    // let url_str : Vec<&str> = urls.iter().map(|s| s.as_str()).collect();
    let url_str : Vec<&str> = proper_url.iter().map(|s| s.as_str()).collect();

    status(&url_str).await;
    // ping_urls(&url_str).await;
}
