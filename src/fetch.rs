use std::path::Path;

use rookie::enums::Cookie;
use time::{Date, OffsetDateTime};

use crate::date_utils::{self, DateIter};

pub async fn execute(
    start: Option<Date>,
    end: Option<Date>,
    token: Option<String>,
    skip_sunday: bool,
    dest: impl AsRef<Path>,
) {
    let today = time::OffsetDateTime::now_utc().date();
    let start = start.unwrap_or(today);
    let end = end.unwrap_or(today).next_day().expect("the future");
    if !dest.as_ref().exists() {
        std::fs::create_dir_all(&dest).expect("create dest dir");
    }
    if end < start {
        eprintln!("Invalid start/end date `end` should be >= `start`");
        std::process::exit(1);
    }
    let token = token
        .or_else(try_find_cookie)
        .expect("could not find token");
    let mut date_iter = DateIter::new(start, end);
    if skip_sunday {
        date_iter.skip_sunday();
    }
    for date in date_iter {
        if date > today {
            eprintln!("cannot download future puzzles");
            std::process::exit(1);
        }
        eprintln!(
            "requesting puzzle for {}",
            date.format(time::macros::format_description!("[year]-[month]-[day]"))
                .unwrap()
        );
        fetch_puzzle_for(date, dest.as_ref(), &token).await;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

fn try_find_cookie() -> Option<String> {
    if let Ok(cookies) = rookie::firefox(None) {
        if let Some(ret) = scan_cookies(cookies.into_iter()) {
            return Some(ret);
        }
    }
    if let Ok(cookies) = rookie::chrome(None) {
        if let Some(ret) = scan_cookies(cookies.into_iter()) {
            return Some(ret);
        }
    }
    if let Ok(cookies) = rookie::brave(None) {
        if let Some(ret) = scan_cookies(cookies.into_iter()) {
            return Some(ret);
        }
    }
    if let Ok(cookies) = rookie::safari(None) {
        if let Some(ret) = scan_cookies(cookies.into_iter()) {
            return Some(ret);
        }
    }
    eprintln!("failed to find NYT-S cookie in firefox, chrome, brave, or safari cookie stores");
    None
}

fn scan_cookies(mut cookies: impl Iterator<Item = Cookie>) -> Option<String> {
    cookies.find_map(|co| {
        if co.name.contains("NYT-S")
            && co
                .expires
                .map(|ts| {
                    let exp = OffsetDateTime::from_unix_timestamp(ts as _).unwrap();
                    if exp > OffsetDateTime::now_utc() {
                        println!("token expires in {:?}", exp - OffsetDateTime::now_utc());
                        true
                    } else {
                        false
                    }
                })
                .unwrap_or(true)
        {
            return Some(co.value);
        }
        None
    })
}

async fn fetch_puzzle_for(date: Date, dest: &Path, token: &str) {
    let month = date_utils::month_str(date.month());
    let year = date.year() % 2000;
    let day = date.day();
    let year_dir = dest.join(date.year().to_string());
    let month_dir = year_dir.join(format!("{:02}", u8::from(date.month())));
    if !month_dir.exists() {
        std::fs::create_dir_all(&month_dir).unwrap();
    }
    let client = reqwest::Client::default();
    let bytes = client
        .get(format!(
            "https://www.nytimes.com/svc/crosswords/v2/puzzle/print/{month}{day:02}{year:02}.pdf"
        ))
        .header("cookie", format!("NYT-S={token}"))
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    if bytes.len() < 5 || bytes.slice(0..5) != b"%PDF-".as_slice() {
        eprint!("Unexpected payload from request for");
        eprintln!(
            "{}",
            date.format(time::macros::format_description!("[year]-[month]-[day]"))
                .unwrap()
        );
        let end_idx = bytes.len().min(255);
        let chunk = bytes.slice(0..end_idx);
        let as_str = String::from_utf8_lossy(&chunk);
        eprintln!("{as_str}");
        return;
    }
    let file_path = month_dir.join(format!("{:02}.pdf", date.day()));
    eprintln!("saving puzzle to {}", file_path.display());
    std::fs::write(&file_path, bytes.as_ref()).unwrap();
}
