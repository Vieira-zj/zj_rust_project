// datetime

#[test]
fn it_time_expensive() {
    use std::thread;
    use std::time::{Duration, Instant};

    fn expensive() {
        thread::sleep(Duration::from_secs(1));
    }

    let start = Instant::now();
    expensive();
    let duration = start.elapsed();
    println!("time elapsed in expensive function is: {:?}", duration);
}

#[test]
fn it_time_now() {
    use chrono::prelude;

    let utc = prelude::Utc::now();
    let local = prelude::Local::now();
    println!("the utc is {}\nthe local is {}", utc, local);
}

#[test]
fn it_time_calculation() {
    use chrono::{Duration, Local};

    let now = Local::now();
    let after_one_week = now.checked_add_signed(Duration::weeks(1)).unwrap();
    let three_day_earlier = now.checked_sub_signed(Duration::days(3)).unwrap();
    println!(
        "now is {}\nnow after one week is {}\n and now before three days is {}",
        now, after_one_week, three_day_earlier
    );
}

#[test]
fn it_datetime_and_timestamp() {
    use chrono::{DateTime, TimeZone, Utc};

    // 时间戳转为日期
    let dt = Utc.timestamp_opt(1_500_000_000, 0).unwrap();
    println!("the date is {}", dt.to_rfc2822());

    // 日期转为时间戳
    let dt = DateTime::parse_from_rfc2822("Fri, 14 Jul 2017 02:40:00 +0000").unwrap();
    println!("the timestamp is {}", dt.timestamp());
}

#[test]
fn it_datetime_format() {
    use chrono::Local;

    let now = Local::now();
    println!("UTC now is: {}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3339 is: {}", now.to_rfc3339());
    println!(
        "UTC now in a custom format is: {}",
        now.format("%a %b %e %T %Y")
    );
    println!(
        "UTC now in a custom format is: {}",
        now.format("%Y-%m-%d %H:%M:%S")
    );
}

#[test]
fn it_parse_datetime() {
    use chrono::{DateTime, TimeZone, Utc};

    // 方式1
    let dt1 =
        DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z").unwrap();
    println!("date1 is {}", dt1);
    let dt2 = DateTime::parse_from_rfc2822("Fri, 28 Nov 2014 21:00:09 +0900").unwrap();
    println!("date2 is {}", dt2);

    // 方式2
    let dt1 = Utc
        .datetime_from_str("2014-11-28 12:00:09", "%Y-%m-%d %H:%M:%S")
        .unwrap();
    println!("date1 is {}", dt1);
    let dt2 = Utc
        .datetime_from_str("Fri Nov 28 12:00:09 2014", "%a %b %e %T %Y")
        .unwrap();
    println!("date2 is {}", dt2);
}

// file io

#[test]
fn it_read_write_file() {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};

    let path = "/tmp/test/test.txt";
    let mut out_f = File::create(path).unwrap();
    write!(out_f, "rust\nhello world").unwrap();

    let in_f = File::open(path).unwrap();
    let buf = BufReader::new(in_f);
    for line in buf.lines() {
        println!("{}", line.unwrap());
    }
}

#[test]
fn it_get_modified_files() {
    use std::{env, fs};

    let cur_dir = env::current_dir().unwrap();
    println!("entries modified in the last 24 hours in {:?}:", cur_dir);

    for entry in fs::read_dir(cur_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap();
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        if last_modified > (24 * 3600) && metadata.is_file() {
            println!(
                "last modified: {} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("null").unwrap()
            );
        }
    }
}

#[test]
fn it_get_modified_json_files() {
    use walkdir::WalkDir;

    let path = "/tmp/test";
    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata().unwrap().modified().unwrap();
        if f_name.ends_with(".json") && sec.elapsed().unwrap().as_secs() < 86400 {
            println!("modified json file in last 24 hours: {}", f_name);
        }
    }
}

#[test]
fn it_get_files_in_dir() {
    use walkdir::{DirEntry, WalkDir};

    fn is_not_hidden(entry: &DirEntry) -> bool {
        return entry
            .file_name()
            .to_str()
            .map(|s| entry.depth() == 0 || !s.starts_with("."))
            .unwrap_or(false);
    }

    let path = "/tmp/test";
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|e| e.ok())
        .for_each(|e| println!("{}", e.path().display()));
}

#[test]
fn it_get_dir_total_size() {
    use walkdir::WalkDir;

    let path = "/tmp/test";
    let total_size = WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| e.metadata().ok())
        .filter(|md| md.is_file())
        .fold(0, |total, md| total + md.len());
    println!("total size: {} bytes.", total_size);
}

#[test]
fn it_find_files_in_dir() {
    use glob::{glob_with, MatchOptions};

    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };
    for entry in glob_with("/tmp/test/*.json", options).expect("failed to read glob pattern") {
        println!("{}", entry.unwrap().display());
    }
}

// random

#[test]
fn it_generate_rand_values() {
    use rand::Rng;

    // 生成 u8 范围内的随机数
    let x = rand::random::<u8>();
    println!("{}", x);
    // 生成 u32 范围内的随机数
    let x = rand::random::<u32>();
    println!("{}", x);
    // 生成 0 和 1 之间的浮点随机数，不包含 1
    let y = rand::random::<f64>();
    println!("{}", y);

    // rand bool
    let result: bool = rand::random();
    println!("{}\n", result);

    let mut rng = rand::thread_rng();
    println!("{}", rng.gen_range(0..10));
    println!("{}", rng.gen_range(0.0..10.0));

    // rand tuple
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    println!("{:?}", rand_tuple);
}

#[test]
fn it_generate_rand_string() {
    use rand::distributions::{Alphanumeric, DistString};
    use rand::Rng;

    // rand string
    let s = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    println!("random string: {}", s);

    // rand password
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
        abcdefghijklmnopqrstuvwxyz\
        0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 30;

    let mut rng = rand::thread_rng();
    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("password: {}", password);
}

// regex

#[test]
fn it_regex_find_tags() {
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::collections::HashSet;

    fn extract_hashtags(text: &str) -> HashSet<&str> {
        lazy_static! {
            static ref HASHTAG_REGEX: Regex = Regex::new(r"\#[a-zA-Z][0-9a-zA-Z_]*").unwrap();
        }
        HASHTAG_REGEX.find_iter(text).map(|m| m.as_str()).collect()
    }

    let tweet = "Hey #world, I just got my new #dog, say hello to Till. #dog #forever #2 #_ ";
    let tags = extract_hashtags(tweet);
    assert_eq!(tags.len(), 3);
    assert!(tags.contains("#dog") && tags.contains("#forever") && tags.contains("#world"));
}

#[test]
fn it_regex_relace_all() {
    use lazy_static::lazy_static;
    use regex::Regex;
    use std::borrow::Cow;

    fn reformat_dates(text: &str) -> Cow<str> {
        lazy_static! {
            static ref ISO8601_DATE_REGEX: Regex =
                Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        }
        ISO8601_DATE_REGEX.replace_all(text, "$m/$d/$y")
    }

    let before = "2012-03-14, 2013-01-15 and 2014-07-05";
    let after = reformat_dates(before);
    assert_eq!(after, "03/14/2012, 01/15/2013 and 07/05/2014");
}
