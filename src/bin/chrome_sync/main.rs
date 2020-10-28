extern crate probabilistic_collections;
use chrono::{TimeZone, Utc};
use personal_search::indexer;
use rayon::prelude::*;
use rusqlite::{params, Connection};
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use toml::Value;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(long = "backfill")]
    backfill: bool,
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(long = "db")]
    #[structopt(parse(from_os_str))]
    db: Option<PathBuf>,
}

#[cfg(not(feature = "brave"))]
fn find_places_file() -> Option<PathBuf> {
    //~/.config/google-chrome/Default/History
    //~/.config/google-chrome-beta/Default/History
    //~/.config/google-chrome-unstable/Default/History
    //~/.config/chromium/Default/History
    let home = dirs::home_dir().expect("no home dir");

    let path = if cfg!(target_os = "linux") {
        format!("{}/.config/google-chrome/Default/History", home.display())
    } else if cfg!(target_os = "macos") {
        format!(
            "{}/Library/Application Support/Google/Chrome/Default/History",
            home.display()
        )
    } else {
        format!(
            "{}\\AppData\\Local\\Google\\Chrome\\User Data\\Default\\History",
            home.display()
        )
    };
    Some(Path::new(&path).into())
}

#[cfg(feature = "brave")]
fn find_places_file() -> Option<PathBuf> {
    //~/.config/google-chrome/Default/History
    //~/.config/google-chrome-beta/Default/History
    //~/.config/google-chrome-unstable/Default/History
    //~/.config/chromium/Default/History
    let home = dirs::home_dir().expect("no home dir");

    let path = if cfg!(target_os = "linux") {
        format!(
            "{}/.config/BraveSoftware/Brave-Browser/Default/History",
            home.display()
        )
    } else if cfg!(target_os = "macos") {
        format!(
            "{}/Library/Application Support/BraveSoftware/Brave-Browser/Default/History",
            home.display()
        )
    } else {
        format!(
            "{}\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Default\\History",
            home.display()
        )
    };
    Some(Path::new(&path).into())
}

#[derive(Debug)]
struct Places {
    id: i64,
    url: String,
    title: Option<String>,
    description: Option<String>,
    visit_count: i64,
    hidden: u8,
    last_visit_date: Option<i64>,
}

fn records(place_file: &PathBuf, backfill: bool, last_id: Option<i64>) -> Vec<Places> {
    let tmp_dir = tempfile::TempDir::new().expect("tmp_dir");
    let tempfile = tmp_dir.path().join("tmpfile");
    let tempfile = tempfile.to_str().unwrap();
    fs::copy(place_file, tempfile).unwrap();
    let conn = Connection::open(tempfile).expect("opening sqlite file");

    let mut stmt = conn.prepare("select visits.id as id, urls.url as url, urls.title as title, urls.visit_count as visit_count, urls.hidden as hidden , visits.visit_time as last_visit_date from visits   join urls on visits.url = urls.id ORDER BY visits.visit_time DESC;").expect("place prep");
    let places_iter = stmt
        .query_map(params![], |row| {
            // dont use wrapper object. we could call it right here.
            Ok(Places {
                id: row.get(0).unwrap(),
                url: row.get(1).unwrap(),
                title: row.get(2).unwrap(),
                visit_count: row.get(3).unwrap(),
                hidden: row.get(4).unwrap(),
                last_visit_date: row.get(5).unwrap(),
                description: None,
            })
        })
        .expect("place sql");
    places_iter
        .filter(|record| {
            let place = record.as_ref().unwrap();
            if place.visit_count > 0 && place.hidden == 0 {
                //move off of index and on time last_visit_date for updates
                if let Some(id_check) = last_id {
                    if let Some(last_visit) = place.last_visit_date {
                        if backfill {
                            // backfill we start with the newest so we want the oldest.
                            id_check >= last_visit
                        } else {
                            // move forward in time.
                            id_check <= last_visit
                        }
                    } else {
                        true
                    }
                } else {
                    true
                }
            } else {
                false
            }
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
}

fn main() -> tantivy::Result<()> {
    let opt = Opt::from_args();
    let index = indexer::search_index().unwrap();
    let place = match opt.db.clone() {
        Some(arg_path) => Some(arg_path),
        None => find_places_file(),
    };
    let path = Path::new(indexer::BASE_INDEX_DIR.as_str());
    let path = path.join("chrome_sync_cache.toml");
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
    let last_id = if !s.is_empty() {
        let value = s.parse::<Value>().unwrap();
        value["last_id"].as_integer()
    } else {
        None
    };

    match place {
        Some(place_file) => {
            let records = records(&place_file, opt.backfill, last_id);
            let limit = if last_id.is_none() { 1000 } else { 100000000 };
            let record_list = records.iter().take(limit).collect::<Vec<_>>();
            let mut date = 0;
            let mut records_data = HashMap::new();
            for record in &record_list {
                if let Some(last_date) = record.last_visit_date {
                    if last_date > date {
                        date = last_date;
                    }
                }
                records_data
                    .entry(record.url.clone())
                    .or_insert_with(Vec::new)
                    .push(record.clone());
            }

            let _results = &record_list
                .par_iter()
                .map(|record| {
                    dbg!(&record.url);
                    (
                        record.url.clone(),
                        indexer::get_url(&record.url, &index, indexer::NoAuthBlockingGetter {}),
                    )
                })
                .chunks(20)
                .map(|chunks| {
                    let _time = Utc::now().timestamp();
                    for data in chunks.iter() {
                        let place = records_data.get(&data.0).unwrap().last().unwrap();
                        if let indexer::GetUrlStatus::New(web_data) = &data.1 {
                            //https://gist.github.com/dropmeaword/9372cbeb29e8390521c2
                            let date = place
                                .last_visit_date
                                .map(|num| Utc.timestamp(num / 1000000 - 11644473600, 0));
                            let meta = indexer::UrlMeta {
                                url: Some(place.url.clone()),
                                title: place.title.clone(),
                                bookmarked: None,
                                last_visit: date,
                                access_count: Some(place.visit_count),
                                pinned: None,
                                tags_add: None,
                                tags_remove: None,
                                hidden: None,
                            };
                            if let Some(doc) = indexer::url_doc(
                                &place.url.clone(),
                                &index,
                                meta,
                                indexer::NoAuthBlockingGetter {},
                                Some(web_data.clone()),
                            ) {
                                let index_writer_read = indexer::SEARCHINDEXWRITER.clone();
                                index_writer_read.read().unwrap().add_document(doc);
                            }
                        }
                        let mut file = OpenOptions::new()
                            .truncate(true)
                            .write(true)
                            .create(true)
                            .open(&path_name)
                            .expect("cache file");

                        file.write_all(format!("last_id = {}", date).as_bytes())
                            .expect("chrome cache");
                    }
                })
                .collect::<Vec<_>>();
        }

        _ => {
            println!("bad");
        }
    }
    Ok(())
}
