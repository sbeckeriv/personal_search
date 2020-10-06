extern crate probabilistic_collections;
use chrono::{TimeZone, Utc};
use glob::glob;
use personal_search::indexer;
use rusqlite::{params, Connection, Result};
use std::collections::HashSet;

use std::io::prelude::*;
use std::io::Read;
use std::iter::FromIterator;
use std::path::PathBuf;
use structopt::StructOpt;

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
    let entries = glob(&format!(
        "{}/.mozilla/firefox/*/places.sqlite",
        home.display()
    ))
    .expect("Failed to read glob pattern");
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
#[derive(Debug)]
struct MozPlaces {
    id: i64,
    url: String,
    title: Option<String>,
    description: Option<String>,
    visit_count: i64,
    hidden: u8,
    last_visit_date: Option<i64>,
}

#[derive(Debug)]
struct MozBookmarks {
    id: i64,
    fk: Option<i64>,
    title: Option<String>,
}

use std::fs::OpenOptions;
use toml::Value;
fn main() -> tantivy::Result<()> {
    let opt = Opt::from_args();
    let _index = indexer::search_index().unwrap();
    let place = match opt.db {
        Some(arg_path) => Some(arg_path),
        None => find_places_file(),
    };
    let path_name = ".private_search/firefox_sync_cache.toml".to_string();
    let mut s = String::new();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path_name);

    match file {
        Err(_why) => {
            //println!("couldn't open {}: {}", path_name, why.to_string());
        }
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
            dbg!(&place_file);
            let conn = Connection::open(place_file).expect("opening sqlite file");
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
            let places_iter = stmt
                .query_map(params![], |row| {
                    // dont use wrapper object. we could call it right here.
                    Ok(MozPlaces {
                        id: row.get(0).unwrap(),
                        url: row.get(1).unwrap(),
                        title: row.get(2).unwrap(),
                        description: row.get(3).unwrap(),
                        visit_count: row.get(4).unwrap(),
                        hidden: row.get(5).unwrap(),
                        last_visit_date: row.get(6).unwrap(),
                    })
                })
                .expect("place sql");
            for places in places_iter {
                let place = places.unwrap();
                if place.visit_count > 0 && place.hidden == 0 {
                    //move off of index and on time last_visit_date for updates
                    if let Some(id_check) = last_id {
                        if let Some(last) = place.last_visit_date {
                            if opt.backfill {
                                // backfill we start with the newest so we want the oldest.
                                if id_check < last {
                                    continue;
                                }
                            } else {
                                // move forward in time.
                                if id_check > last {
                                    continue;
                                }
                            }
                        }
                    }
                    if place.id % 10 == 0 {
                        if let Some(last) = place.last_visit_date {
                            let mut file = OpenOptions::new()
                                .truncate(true)
                                .write(true)
                                .create(true)
                                .open(&path_name)
                                .expect("cache file");

                            file.write_all(format!("last_id = {}", last).as_bytes());
                        }
                    }

                    let meta = indexer::UrlMeta {
                        title: place.title,
                        bookmarked: Some(bookmarks.contains(&place.id)),
                        last_visit: place
                            .last_visit_date
                            .map(|num| Utc.timestamp(num / 1000000, 0)),
                        access_count: Some(place.visit_count),
                        pinned: None,
                        keywords: None,
                    };
                    indexer::index_url(place.url, meta, None)
                }
            }
        }
        _ => {
            println!("bad");
        }
    };

    Ok(())
}
