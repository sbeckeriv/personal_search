extern crate probabilistic_collections;
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use dirs;
use glob::glob;
use personal_search::indexer;
use probabilistic_collections::similarity::{ShingleIterator, SimHash};
use probabilistic_collections::SipHasherBuilder;
use reqwest;
use rusqlite::{params, Connection, Result};
use select::document;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::iter::FromIterator;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, ReloadPolicy};
use toml;

fn find_places_file() -> Option<PathBuf> {
    //~/.mozilla/firefox/xdfjt9cu.default/places.sqlite
    let home = dirs::home_dir().expect("no home dir");
    let mut entries = glob(&format!(
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
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let arg_path = args.get(1);
    let index = indexer::search_index().unwrap();
    let place = match arg_path {
        Some(arg_path) => Some(PathBuf::from(arg_path)),
        None => find_places_file(),
    };
    let path_name = format!(".firefox_sync_cache.toml");
    let mut s = String::new();
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path_name);
    match file {
        Err(why) => {
            //println!("couldn't open {}: {}", path_name, why.to_string());
        }
        Ok(mut file) => match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", path_name, why),
            Ok(_) => (),
        },
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

            let mut stmt = conn.prepare("SELECT id, url, title, description, visit_count, hidden, last_visit_date FROM moz_places").expect("place prep");
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
                    if let Some(id_check) = last_id {
                        if id_check > place.id {
                            continue;
                        }
                    }
                    if place.id % 10 == 0 {
                        let mut file = OpenOptions::new()
                            .truncate(true)
                            .write(true)
                            .create(true)
                            .open(&path_name)
                            .expect("cache file");

                        file.write_all(format!("last_id = {}", place.id).as_bytes());
                    }

                    let meta = indexer::UrlMeta {
                        title: place.title,
                        bookmarked: Some(bookmarks.contains(&place.id)),
                        last_visit: place
                            .last_visit_date
                            .map_or(None, |num| Some(Utc.timestamp(num / 1000000, 0))),
                        access_count: Some(place.visit_count),
                        pinned: None,
                        keywords: None,
                    };
                    indexer::index_url(place.url, meta, Some(&index))
                }
            }
        }
        _ => {
            println!("bad");
        }
    };

    Ok(())
}
