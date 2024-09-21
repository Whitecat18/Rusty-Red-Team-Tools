
use reqwest::{blocking::Client, Url};
use regex::Regex;
use std::path::Path;
use clap::{App, Arg};

struct SvnHack {
    root_dir: Option<String>,
    url: String,
    client: Client,
}

impl SvnHack {
    fn new(url: String) -> Self {
        SvnHack {
            root_dir: None,
            url,
            client: Client::new(),
        }
    }

    fn list_dir(&self) {
        let res = self.client.get(&self.url).send().unwrap().text().unwrap();
        let dir_re = Regex::new(r"\n(.*?)\ndir").unwrap();
        let file_re = Regex::new(r"\n(.*?)\nfile").unwrap();

        for cap in dir_re.captures_iter(&res) {
            if !cap[1].is_empty() {
                println!("\x1b[1;34;40m{}\x1b[0m", &cap[1]);
            }
        }
        println!("--------");

        for cap in file_re.captures_iter(&res) {
            println!("{}", &cap[1]);
        }
    }


    fn read_file(&self) {
        let res = self.client.get(&self.url).send().unwrap().text().unwrap();
        println!("{}", res);
    }

    fn is_exists<P: AsRef<Path>>(&self, dir: P) -> bool {
        !dir.as_ref().exists()
    }

    fn fetch_dir(&self, entries_url: &str) -> Vec<String> {
        let res = self.client.get(entries_url).send().unwrap().text().unwrap();
        let dir_re = Regex::new(r"\n(.*?)\ndir").unwrap();
        let dic: Vec<String> = dir_re
            .captures_iter(&res)
            .filter_map(|cap| if !cap[1].is_empty() { Some(cap[1].to_string()) } else { None })
            .collect();

        let mut next_url_list = Vec::new();
        if !dic.is_empty() {
            for i in &dic {
                let url = format!("{}{}/.svn/entries", entries_url.split(".svn").next().unwrap(), i);
                let path = format!("./{}{}", self.root_dir.as_ref().unwrap(), url.split("?").next().unwrap());
                if self.is_exists(&path) {
                    std::fs::create_dir_all(&path).unwrap();
                }
                next_url_list.push(url);
            }
        }
        next_url_list
    }
    
    fn download_file(&self, entries_url: &str) {
        let res = self.client.get(entries_url).send().unwrap().text().unwrap();
        let file_re = Regex::new(r"\n(.*?)\nfile").unwrap();
        for cap in file_re.captures_iter(&res) {
            let url = format!("{}{}", entries_url.split(".svn").next().unwrap(), &cap[1]);
            let path = format!("./{}{}", self.root_dir.as_ref().unwrap(), url.split("?").next().unwrap());
            let content = self.client.get(&url).send().unwrap().text().unwrap();
            println!("[Fetch] {}", url);
            std::fs::write(&path, content).unwrap();
        }
    }

    fn download_site(&mut self) {
        let res = self.client.get(&self.url).send().unwrap().text().unwrap();
        self.root_dir = Some(Url::parse(&self.url).unwrap().host_str().unwrap().to_string());

        if self.is_exists(self.root_dir.as_ref().unwrap()) {
            std::fs::create_dir(self.root_dir.as_ref().unwrap()).unwrap();
        }

        let dir_re = Regex::new(r"\n(.*?)\ndir").unwrap();
        let mut dir_list = Vec::new();
        for cap in dir_re.captures_iter(&res) {
            if !cap[1].is_empty() {
                if self.is_exists(&format!("{}/{}", self.root_dir.as_ref().unwrap(), &cap[1])) {
                    std::fs::create_dir(&format!("{}/{}", self.root_dir.as_ref().unwrap(), &cap[1])).unwrap();
                    let entries_url = format!("{}{}/.svn/entries", self.url.split(".svn").next().unwrap(), &cap[1]);
                    dir_list.push(entries_url);
                }
            }
        }

        while !dir_list.is_empty() {
            let next_dir = self.fetch_dir(dir_list.pop().unwrap().as_str());
            dir_list.extend(next_dir);
            for url in &dir_list {
                self.download_file(url);
            }
        }

        self.download_file(&self.url);
    }

    fn audit(&mut self, matches: &clap::ArgMatches) {
        if let Some(url) = matches.value_of("url") {
            self.url = url.to_string();

            if let Some(dirname) = matches.value_of("dirname") {
                self.url = format!("{}{}/.svn/entries", self.url.split(".svn").next().unwrap(), dirname);
            }

            if matches.is_present("download") {
                self.download_site();
                std::process::exit(0);
            }

            if let Some(readfile) = matches.value_of("readfile") {
                self.url = format!("{}{}/.svn/text-base/{}.svn-base", self.url.split(".svn").next().unwrap(), "", readfile);
                self.read_file();
            } else {
                self.list_dir();
            }
        }
    }
}


fn main() {
    let matches = App::new("SvnHack")
        .version("1.0")
        .author("Your Name")
        .about("A tool for interacting with SVN repositories")
        .arg(Arg::with_name("url")
             .short('u')
             .long("url")
             .value_name("URL")
             .help("Add an SVN URL")
             .takes_value(true))
        .arg(Arg::with_name("dirname")
             .short('d')
             .long("dic")
             .value_name("DIRECTORY")
             .help("List a directory")
             .takes_value(true))
        .arg(Arg::with_name("readfile")
             .short('r')
             .long("read")
             .value_name("FILE")
             .help("Read a file")
             .takes_value(true))
        .arg(Arg::with_name("download")
             .long("download")
             .help("Download the entire site"))
        .get_matches();

    let mut svn = SvnHack::new(matches.value_of("url").unwrap_or("").to_string());
    svn.audit(&matches);
}
