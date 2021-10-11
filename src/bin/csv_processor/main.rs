use csv;
use glob::glob;
use personal_search::indexer;
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(long = "input_directory")]
    #[structopt(parse(from_os_str))]
    input_directory: Option<PathBuf>,
}

fn find_files(base_path: &PathBuf) -> Option<PathBuf> {
    //~/.mozilla/firefox/xdfjt9cu.default/places.sqlite
    let home = dirs::home_dir().expect("no home dir");
    let path = if cfg!(target_os = "linux") {
        format!("{}/.mozilla/firefox/*/places.sqlite", home.display())
    } else if cfg!(target_os = "macos") {
        format!(
            "{}/Library/Application Support/Firefox/Profiles/*/places.sqlite",
            home.display()
        )
    } else {
        format!(
            "{}\\AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\*\\places.sqlite",
            home.display()
        )
    };
    let entries = glob(&path).expect("Failed to read glob pattern");
    let mut entries: Vec<PathBuf> = entries.filter_map(Result::ok).collect();
    entries.sort_by(|a, b| {
        b.metadata()
            .unwrap()
            .accessed()
            .unwrap()
            .partial_cmp(&a.metadata().unwrap().accessed().unwrap())
            .unwrap()
    });
    entries.pop()
}
#[derive(Debug, Clone, Deserialize)]
struct Urls {
    id: i64,
    url: String,
    title: Option<String>,
    description: Option<String>,
    visit_count: i64,
    hidden: u8,
    last_visit_date: Option<i64>,
    bookmarked: bool,
}

fn main() {
    let opt = Opt::from_args();

    let in_files_directory = if let Some(input_directory) = opt.input_directory.clone() {
        input_directory
    } else {
        let path = Path::new(indexer::BASE_INDEX_DIR.as_str());
        let path = path.join("csv_imports");
        path.to_str().expect("cache").into()
    };

    /*let out_file = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(&out_path)
    .expect("out file location");
    */
    let path = Path::new(indexer::BASE_INDEX_DIR.as_str());
    let path = path.join("firefox_sync_cache.toml");
    let path_name = path.to_str().expect("cache");
    let mut s = String::new();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path_name);

    match file {
        Err(_why) => {}
        Ok(mut file) => {
            if let Err(why) = file.read_to_string(&mut s) {
                panic!("couldn't read {}: {}", path_name, why)
            };
        }
    };
    let value = s.parse::<Value>().unwrap();

}
