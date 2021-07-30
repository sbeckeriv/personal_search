extern crate probabilistic_collections;
use chrono::{TimeZone, Utc};
use glob::glob;
use personal_search::indexer;
use rand;
use rayon::prelude::*;
use rusqlite::{params, Connection, Result};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Read;
use std::iter::FromIterator;
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

fn find_places_file() -> Option<PathBuf> {
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
#[derive(Debug, Clone)]
struct MozPlaces {
    id: i64,
    url: String,
    title: Option<String>,
    description: Option<String>,
    visit_count: i64,
    hidden: u8,
    last_visit_date: Option<i64>,
    bookmarked: bool,
}

#[derive(Debug)]
struct MozBookmarks {
    id: i64,
    fk: Option<i64>,
    title: Option<String>,
}

fn records(place_file: &PathBuf, backfill: bool, last_id: Option<i64>) -> Vec<MozPlaces> {
    let tmp_dir = tempfile::TempDir::new().expect("tmp_dir");
    let tempfile = tmp_dir.path().join("tmpfile");
    let tempfile = tempfile.to_str().unwrap();
    fs::copy(place_file, tempfile).unwrap();
    let conn = Connection::open(tempfile).expect("opening sqlite file");
    let mut stmt = conn
        .prepare("SELECT id, fk, title FROM moz_bookmarks")
        .expect("book prep");

    let bookmark_iter = stmt
        .query_map(params![], |row| {
            Ok(MozBookmarks {
                id: row.get(0).unwrap(),
                fk: row.get(1).unwrap(),
                title: row.get(2).unwrap(),
            })
        })
        .expect("bookmark sql");
    let bookmarks: HashSet<i64> = HashSet::from_iter(
        bookmark_iter
            .filter(|b| b.as_ref().ok().unwrap().fk.is_some())
            .map(|b| b.as_ref().ok().unwrap().fk.unwrap()),
    );

    let mut stmt = conn.prepare("SELECT id, url, title, description, visit_count, hidden, last_visit_date FROM moz_places order by last_visit_date desc").expect("place prep");
    let places = stmt
        .query_map(params![], |row| {
            // dont use wrapper object. we could call it right here.
            let id = row.get(0).unwrap();
            Ok(MozPlaces {
                id,
                url: row.get(1).unwrap(),
                title: row.get(2).unwrap(),
                description: row.get(3).unwrap(),
                visit_count: row.get(4).unwrap(),
                hidden: row.get(5).unwrap(),
                last_visit_date: row.get(6).unwrap(),
                bookmarked: bookmarks.contains(&id),
            })
        })
        .expect("place sql");

    places
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
    let _index = indexer::search_index().unwrap();
    let place = match opt.db.clone() {
        Some(arg_path) => Some(arg_path),
        None => find_places_file(),
    };
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
            let index = indexer::search_index().unwrap();
            dbg!(record_list.len());
            let _results = &record_list
                .par_iter()
                //.iter()
                .map(|record| {
                    (
                        record.url.clone(),
                        indexer::get_url(&record.url, &index, indexer::NoAuthBlockingGetter {}),
                    )
                })
                .map(|data| {
                    let place = records_data.get(&data.0).unwrap().last().unwrap();
                    if let indexer::GetUrlStatus::New(web_data) = &data.1 {
                        let meta = indexer::UrlMeta {
                            url: Some(place.url.clone()),
                            title: place.title.clone(),
                            bookmarked: Some(place.bookmarked),
                            last_visit: place
                                .last_visit_date
                                .map(|num| Utc.timestamp(num / 1000000, 0)),
                            access_count: Some(place.visit_count),
                            pinned: None,
                            tags_add: None,
                            tags_remove: None,
                            hidden: Some(0),
                        };
                        indexer::url_doc(
                            &place.url.clone(),
                            &index,
                            meta,
                            indexer::NoAuthBlockingGetter {},
                            Some(web_data.clone()),
                        )
                    } else {
                        None
                    }
                })
                .chunks(20)
                .map(|chunks| {
                    let mut added = false;
                    for data in chunks.into_iter() {
                        if let Some(doc) = data {
                            added = true;
                            let index_writer_read = indexer::SEARCHINDEXWRITER.clone();
                            index_writer_read.write().unwrap().add_document(doc);
                        }
                    }
                    if added {
                        {
                            let mut index_writer_wlock =
                                indexer::SEARCHINDEXWRITER.write().unwrap();

                            index_writer_wlock.commit().expect("commit");
                        }
                        let hash_index_writer_read = indexer::HASHINDEXWRITER.clone();
                        hash_index_writer_read.write().unwrap().commit().unwrap();
                    } else {
                    }
                    //println!("done commit {}", time);
                })
                .collect::<Vec<_>>();

            let mut file = OpenOptions::new()
                .truncate(true)
                .write(true)
                .create(true)
                .open(&path_name)
                .expect("cache file");

            file.write_all(format!("last_id = {}", date).as_bytes())
                .expect("ff cache");
        }
        _ => {
            println!("bad firefox place sql file");
        }
    };

    Ok(())
}
