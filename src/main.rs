use std::{fs, io::{Read, Write}, path, process::exit, thread, time::Duration};

use r34_api::*;
use tokio;

#[tokio::main]
async fn main() {
    startup();

    let mut pid = String::new();
    let mut limit_buf = String::new();

    println!("Add Post Limit. Max 1000. (post limit = posts per request you want)");
    std::io::stdin().read_line(&mut limit_buf).unwrap();

    println!("Add Request Amount.");
    std::io::stdin().read_line(&mut pid).unwrap();

    let limit = match limit_buf.trim().parse::<usize>() {
        Ok(u) => u,
        Err(_) => {
            println!("Couldn't read input\nU probably didn't enter any number.\nOr it could be a to big input.\nPlease try again with another input");
            thread::sleep(Duration::from_secs(5));
            panic!();
        }
    };

    let pid = match pid.trim().parse::<usize>() {
        Ok(u) => {
            match u {
                0 => 1,
                _ => u,
            }
        },
        Err(_) => {
            println!("Couldn't read input\nU probably didn't enter any number.\nOr it could be a to big input.\nPlease try again with another input");
            thread::sleep(Duration::from_secs(5));
            panic!();
        }
    };

    let mut tasks = Vec::new();

    println!("Downloading...");

    for i in 0..pid {
        let req_url = ApiUrl::new().add_tags(read_config()).set_limit(limit).set_pid(i).to_api_url();
        tasks.push(dw(req_url, i));
    }

    for task in tasks {
        tokio::join!(task);
    }

    println!("Finished downloading!");
}

fn startup() {
    if !path::Path::new("./downloads").exists() {
        fs::create_dir("./downloads").unwrap();
    }
    if !path::Path::new("./source_list.txt").exists() {
        fs::File::create("./source_list.txt").unwrap();
    }
    if !path::Path::new("./config.txt").exists() {
        fs::File::create("./config.txt").unwrap()
        .write_all(String::from("#Add all your tags on the next lines. Seperate them with spaces.").as_bytes()).unwrap();
        println!("Go into the config.txt file and read the instructions or go on the Github page for more information.");
        thread::sleep(Duration::from_secs(5));
        exit(69);
    }

    fs::File::create("./log.txt").unwrap();
}

fn read_config() -> Vec<String>{
    let mut buf = String::new();
    fs::File::open("./config.txt").unwrap().read_to_string(&mut buf).unwrap();
    let mut tag_string = String::new();
    for line in buf.lines() {
        if let Some(c) = line.chars().next() {
            if c != '#' {
                tag_string = line.to_string();
            }
        }
    }
    let tags: Vec<String> = tag_string.split_whitespace().map(|s| s.to_string()).collect();
    tags
}

async fn dw(req_url: String, _req_id: usize) {
    let json_response = reqwest::get(req_url).await.unwrap().text().await.unwrap();
    let posts = match r34_api::R34JsonParser::new().from_api_response(&json_response) {
        Ok(p) => p,
        Err(e) => {
            wlog(&e.to_string());
            exit(0);
        }
    };

    let mut tasks = Vec::new();

    let mut dups = 0;

    for post in posts {
        let path = format!("./downloads/{}", &post.image);
        if !path::Path::new(&path).exists() {
            tasks.push(download_posts(post.clone()));
        } else {
            dups += 1;
        }
    }
    let duplicates = format!("Duplicates(Not Downloaded): {}", dups.to_string());
    wlog(&duplicates);

    for task in tasks {
        tokio::join!(task);
    }
}

async fn download_posts(post: Post) {
    let image_path = format!("./downloads/{}", post.image);
    let file_url = &post.file_url.clone();
    let image = reqwest::get(file_url).await.unwrap().bytes().await.unwrap();
    let list_entry = format!("Image: {}\nSource: {}\n", post.image, post.source);
    fs::File::options().write(true).append(true).open("./source_list.txt").unwrap().write_all(list_entry.as_bytes()).unwrap();
    fs::File::create(image_path).unwrap().write_all(&image).unwrap();
}

fn wlog(message: &str) {
    fs::File::options().write(true).append(true).open("./log.txt").unwrap().write_all(message.as_bytes()).unwrap();
}