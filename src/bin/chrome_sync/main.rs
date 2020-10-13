extern crate probabilistic_collections;
use chrono::{TimeZone, Utc};
use glob::glob;
use personal_search::indexer;
use rusqlite::{params, Connection, Result};
use std::collections::HashSet;

use std::io::prelude::*;
use std::io::Read;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
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
    //~/.config/google-chrome/Default/History
    //~/.config/google-chrome-beta/Default/History
    //~/.config/google-chrome-unstable/Default/History
    //~/.config/chromium/Default/History
    let home = dirs::home_dir().expect("no home dir");

    Some(
        Path::new(&format!(
            "{}/.config/google-chrome/Default/History",
            home.display()
        ))
        .into(),
    )
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

use std::fs::OpenOptions;
use toml::Value;
fn main() -> tantivy::Result<()> {
    let opt = Opt::from_args();
    let _index = indexer::search_index().unwrap();
    let place = match opt.db.clone() {
        Some(arg_path) => Some(arg_path),
        None => find_places_file(),
    };
    let path_name = ".private_search/chrome_sync_cache.toml".to_string();
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
    let last_id = if !s.is_empty() { Some(0) } else { None };

    match place {
        Some(place_file) => {
            let conn = Connection::open(place_file).expect("opening sqlite file");

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
            let places = places_iter
                .map(|record| {
                    let place = record.unwrap();

                    dbg!(&place);

                    if place.visit_count > 0 && place.hidden == 0 {
                        dbg!(&last_id);
                        //move off of index and on time last_visit_date for updates
                        if let Some(id_check) = last_id {
                            if let Some(last_visit) = place.last_visit_date {
                                if opt.backfill {
                                    // backfill we start with the newest so we want the oldest.
                                    if id_check < last_visit {
                                        return None;
                                    }
                                } else {
                                    // move forward in time.
                                    if id_check > last_visit {
                                        return None;
                                    }
                                }
                            }
                        }

                        let meta = indexer::UrlMeta {
                            url: Some(place.url.clone()),
                            title: place.title,
                            bookmarked: None,
                            last_visit: place
                                .last_visit_date
                                //https://gist.github.com/dropmeaword/9372cbeb29e8390521c2
                                .map(|num| Utc.timestamp(num / 1000000 - 11644473600, 0)),
                            access_count: Some(place.visit_count),
                            pinned: None,
                            tags_add: None,
                            tags_remove: None,
                            hidden: None,
                        };
                        Some((place.url, meta, place.id, place.last_visit_date))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // first run only the last 1000 urls
            let places = if last_id.is_none() {
                places.iter().take(1000)
            } else {
                places.iter().take(1000000)
            };

            for record in places.rev() {
                if let Some((url, meta, _id, raw_date)) = record {
                    if let Some(date) = raw_date {
                        dbg!(&raw_date);
                        let mut file = OpenOptions::new()
                            .truncate(true)
                            .write(true)
                            .create(true)
                            .open(&path_name)
                            .expect("cache file");

                        file.write_all(format!("last_id = {}", date).as_bytes())
                            .expect("ff cache");
                    }

                    dbg!(&url);
                    indexer::index_url(
                        url.to_string(),
                        meta.clone(),
                        None,
                        indexer::NoAuthBlockingGetter {},
                    );
                }
            }
        }
        _ => {
            println!("bad");
        }
    };

    Ok(())
}
