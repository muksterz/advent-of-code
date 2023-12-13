use std::sync::Arc;

use keyring::Entry;
use reqwest::{cookie::Jar, Url};
use time::{UtcOffset, OffsetDateTime};

fn main() {
    let mut args = std::env::args().skip(1);
    let cmd = args.next().unwrap();


    match cmd.as_str() {
        "get" => println!("{}", get_token()),
        "set" => set_token(args.next().unwrap()),
        "download" => download(args),
        _ => panic!()
    }


}

fn get_token() -> String{
    let entry = keyring::Entry::new("aoc_runner", &whoami::username()).unwrap();
    let token = entry.get_password().expect("No token found");
    token
}

fn set_token(token: String) {
    let user = whoami::username();
    let entry = Entry::new("aoc_runner", &user).unwrap();
    entry.set_password(&token).unwrap();
}

fn download(mut args: impl Iterator<Item = String>) {
    let today = OffsetDateTime::now_utc();
    let today = today.to_offset(UtcOffset::from_hms(-5, 0, 0).unwrap());
    let year = args.next().map(|n| i32::from_str_radix(&n, 10).unwrap()).unwrap_or(today.year());
    let day = args.next().map(|n| u8::from_str_radix(&n, 10).unwrap()).unwrap_or(today.day());
    let file = format!("input/{year}/day{day}.txt");
    let session = get_token();


    if std::fs::metadata(&file).is_err() {
        let url = format!("https://adventofcode.com/{year}/day/{day}/input");
        let cookies = Jar::default();
        cookies.add_cookie_str(&format!("session={session}"), &url.parse::<Url>().unwrap());
        let client = reqwest::blocking::Client::builder().cookie_provider(Arc::new(cookies)).build().unwrap();
        let r = client.get(url).send().unwrap();
        std::fs::write(&file, r.text().unwrap()).unwrap();
        println!("Downloaded {year}/day/{day} to {file}");
    } else {
        println!("Input already downloaded at {file}");
    }
}

