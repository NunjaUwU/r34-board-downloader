use std::{fmt::format, fs, io::Write, path, time::Duration};

use r34_api::*;
use tokio;

#[tokio::main]
async fn main() {
    if !path::Path::new("./files").exists() {
        fs::create_dir("./files").unwrap();
    }
    if !path::Path::new("./files/source_list.txt").exists() {
        fs::File::create("./files/source_list.txt").unwrap();
    }
    if !path::Path::new("./files/log.txt").exists() {
        fs::File::create("./files/log.txt").unwrap();
    }
    let mut tags_buf = String::new();
    let mut limit_buf = String::new();

    println!("Add Tags. Seperate with spaces.");
    std::io::stdin().read_line(&mut tags_buf).unwrap();

    if tags_buf.trim() == "".to_string() {
        println!("Please add some Tags. Seperate with spaces e.g. 'big_ass cock' or '-yaoi -tits' for blacklisting tags6");
        std::io::stdin().read_line(&mut tags_buf).unwrap();
    };

    println!("Add Response Limit. Max 1000.");
    std::io::stdin().read_line(&mut limit_buf).unwrap();


    let tags: Vec<String> = tags_buf.split_whitespace().map(|s| s.to_string()).collect();

    let limit = match limit_buf.trim().parse::<usize>() {
        Ok(u) => u,
        Err(_) => {
            println!("U probably forgot to set a limit\nSetting it to 10");
            10
        }
    };

    let req_url = Request::new().add_tags(tags).set_limit(limit).to_req_url();

    let json_response = reqwest::get(req_url).await.unwrap().text().await.unwrap();
    let posts = R34JsonParser::default().parse_json(&json_response);

    let mut tasks = Vec::new();

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));

    let mut dups = 0;

    for post in posts {
        let path = format!("./files/{}", &post.image);
        if !path::Path::new(&path).exists() {
            tasks.push(download(post.clone()));
        } else {
            dups += 1;
        }
    }

    for task in tasks {
        tokio::join!(task);
    }

    wlog(dups.to_string().as_str());

    spinner.finish();
    println!("Finished downloading!");
}

async fn download(post: Post) {
    let image_path = format!("./files/{}", post.image);
    let image = reqwest::get(post.file_url).await.unwrap().bytes().await.unwrap();
    let list_entry = format!("Image: {}\nSource: {}\n", post.image, post.source);
    fs::File::options().write(true).append(true).open("./files/source_list.txt").unwrap().write_all(list_entry.as_bytes()).unwrap();
    fs::File::create(image_path).unwrap().write_all(&image).unwrap();
}

fn wlog(message: &str) {
    fs::File::options().write(true).append(true).open("./files/log.txt").unwrap().write_all(message.as_bytes()).unwrap();
}