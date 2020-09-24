use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use dirs;
use glob::glob;
use reqwest;
use rusqlite::{params, Connection, Result};
use select::document;
use std::collections::HashSet;
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

mod Indexer {
    use chrono::prelude::*;
    use lazy_static;
    use reqwest;
    use select::document;
    use std::collections::HashSet;
    use std::path::Path;
    use std::time::Duration;
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::*;
    use tantivy::{Index, ReloadPolicy};

    pub fn hash_index() -> std::result::Result<tantivy::Index, tantivy::TantivyError> {
        let system_path = ".private_search_hash";
        let index_path = Path::new(system_path);
        // create it..
        if !index_path.is_dir() {
            println!("not found");
        }

        let directory = tantivy::directory::MmapDirectory::open(index_path);

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("domain", TEXT | STORED);
        schema_builder.add_facet_field("hashes");

        let schema = schema_builder.build();
        match directory {
            Ok(dir) => Index::open_or_create(dir, schema.clone()),
            Err(_) => {
                println!("dir not found");
                Err(tantivy::TantivyError::SystemError(format!(
                    "could not open index directory {}",
                    system_path
                )))
            }
        }
    }
    pub fn search_index() -> std::result::Result<tantivy::Index, tantivy::TantivyError> {
        let system_path = ".private_search";
        let index_path = Path::new(system_path);
        // create it..
        if !index_path.is_dir() {
            println!("not found");
        }

        let directory = tantivy::directory::MmapDirectory::open(index_path);

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("url", TEXT | STORED);
        schema_builder.add_text_field("content", TEXT);
        schema_builder.add_text_field("domain", TEXT | STORED);
        schema_builder.add_text_field("context", TEXT);
        //schema_builder.add_text_field("preview_image", STORED);
        //schema_builder.add_text_field("preview_hash", STORED);
        //schema_builder.add_bytes_field("preview_image");
        schema_builder.add_i64_field("bookmarked", STORED | INDEXED);
        schema_builder.add_i64_field("pinned", STORED | INDEXED);
        schema_builder.add_i64_field("accessed_count", STORED);
        schema_builder.add_facet_field("outlinks");
        schema_builder.add_facet_field("tags");
        schema_builder.add_facet_field("keywords");
        schema_builder.add_date_field("added_at", STORED);
        schema_builder.add_date_field("last_accessed_at", STORED | INDEXED);

        let schema = schema_builder.build();
        match directory {
            Ok(dir) => Index::open_or_create(dir, schema.clone()),
            Err(_) => {
                println!("dir not found");
                Err(tantivy::TantivyError::SystemError(format!(
                    "could not open index directory {}",
                    system_path
                )))
            }
        }
    }

    pub fn get_url(url: &String) -> Result<String, reqwest::Error> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        let res = client.get(url).send()?;
        let body = res.text()?;

        Ok(body)
    }

    pub fn searcher(index: &Index) -> tantivy::LeasedItem<tantivy::Searcher> {
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()
            .expect("reader");

        reader.searcher()
    }

    #[derive(Debug, Clone)]
    pub struct UrlMeta {
        pub title: Option<String>,
        pub bookmarked: Option<bool>,
        pub last_visit: Option<DateTime<Utc>>,
        pub keywords: Option<Vec<String>>,
        pub pinned: Option<i64>,
        pub access_count: Option<i64>,
    }

    pub fn url_skip(url: &String) -> bool {
        // lazy static this

        let parsed = reqwest::Url::parse(&url).expect("url pase");

        let ignore_includes = vec![
            "//127.0.0.1",
            "//192.168.",
            ".lvh.me",
            "//0.0.0.0",
            "//lvh.me",
            "google.com/",
        ];
        let ignore_starts = vec!["moz-extension://"];
        if !parsed.scheme().starts_with("http") {
            true
        } else if ignore_starts.iter().any(|s| url.starts_with(s)) {
            true
        } else if ignore_includes.iter().any(|s| url.contains(s)) {
            true
        } else {
            false
        }
    }

    pub fn index_url(url: String, meta: UrlMeta, index: Option<&Index>) {
        let i;
        let index = match index {
            Some(index) => index,
            None => {
                i = search_index().unwrap();
                &i
            }
        };
        if url_skip(&url) {
            println!("skip {}", url);
        } else if let Some(doc_address) = find_url(&url, &index) {
            println!("have {}", url);
        // update?

        //let searcher = searcher(&index);
        // let retrieved_doc = searcher.doc(doc_address).expect("doc");
        //    println!("{}", index.schema().to_json(&retrieved_doc));
        } else {
            dbg!(&url);
            match get_url(&url) {
                Ok(body) => {
                    let document = document::Document::from(body.as_str());
                    let mut doc = tantivy::Document::default();
                    let title = match document.find(select::predicate::Name("title")).nth(0) {
                        Some(node) => node.text().to_string(),
                        _ => meta.title.unwrap_or("".to_string()),
                    };

                    let body = match document.find(select::predicate::Name("body")).nth(0) {
                        Some(node) => node.text(),
                        _ => "".to_string(),
                    };

                    doc.add_text(index.schema().get_field("title").expect("title"), &title);
                    doc.add_text(
                        index.schema().get_field("content").expect("content"),
                        &body.split_whitespace().collect::<Vec<_>>().join(" "),
                    );
                    doc.add_text(index.schema().get_field("url").expect("url"), &url);
                    let parsed = reqwest::Url::parse(&url).expect("url pase");

                    doc.add_text(
                        index.schema().get_field("domain").expect("domain"),
                        parsed.domain().unwrap_or(""),
                    );
                    let found_urls = document
                        .find(select::predicate::Name("a"))
                        .filter_map(|n| n.attr("href"))
                        .map(str::to_string)
                        .collect::<HashSet<String>>();
                    for url in found_urls {
                        doc.add_facet(
                            index.schema().get_field("outlinks").expect("outlinks"),
                            Facet::from(&format!("/#{}", url.replacen("/", "?", 10000))),
                        );
                    }

                    let keywords = document
                        .find(select::predicate::Name("meta"))
                        .filter(|node| node.attr("name").unwrap_or("") == "keywords")
                        .filter_map(|n| n.attr("content"))
                        .flat_map(|s| s.split(","))
                        .map(str::to_string)
                        .collect::<Vec<String>>();

                    for keyword in keywords {
                        doc.add_facet(
                            index.schema().get_field("keywords").expect("keywords"),
                            Facet::from(&format!("/{}", keyword)),
                        );
                    }
                    for keyword in meta.keywords.unwrap_or(vec![]) {
                        doc.add_facet(
                            index.schema().get_field("keywords").expect("keywords"),
                            Facet::from(&format!("/{}", keyword)),
                        );
                    }

                    doc.add_date(
                        index.schema().get_field("added_at").expect("added_at"),
                        &Utc::now(),
                    );

                    let last_visit: DateTime<Utc> = meta.last_visit.unwrap_or(Utc::now());
                    doc.add_date(
                        index.schema().get_field("added_at").expect("added_at"),
                        &last_visit,
                    );
                    doc.add_i64(
                        index.schema().get_field("pinned").expect("pinned"),
                        meta.pinned.unwrap_or(0),
                    );
                    doc.add_i64(
                        index
                            .schema()
                            .get_field("accessed_count")
                            .expect("accessed_count"),
                        meta.access_count.unwrap_or(1),
                    );
                    doc.add_i64(
                        index.schema().get_field("bookmarked").expect("bookmarked"),
                        0,
                    );

                    let mut index_writer = index.writer(50_000_000).expect("writer");
                    index_writer.add_document(doc);
                    index_writer.commit().expect("commit");
                }

                Err(e) => {
                    dbg!(e);
                    // add a down domain list to skip
                    // error logging as well.
                }
            }
        };
    }

    pub fn find_url(url: &String, index: &Index) -> std::option::Option<tantivy::DocAddress> {
        let searcher = searcher(&index);
        let query_parser = QueryParser::for_index(
            &index,
            vec![index.schema().get_field("url").expect("url field")],
        );

        let query = query_parser
            .parse_query(&format!("\"{}\"", url))
            .expect("query parse for url match");
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(1))
            .expect("search");
        match top_docs.iter().nth(0) {
            Some((_, doc_address)) => Some(doc_address.clone()),
            _ => None,
        }
    }
}
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
    let index = Indexer::search_index().unwrap();
    let place = find_places_file();
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

                    let meta = Indexer::UrlMeta {
                        title: place.title,
                        bookmarked: Some(bookmarks.contains(&place.id)),
                        last_visit: place
                            .last_visit_date
                            .map_or(None, |num| Some(Utc.timestamp(num / 1000000, 0))),
                        access_count: Some(place.visit_count),
                        pinned: None,
                        keywords: None,
                    };

                    Indexer::index_url(place.url, meta, Some(&index))
                }
            }
        }
        _ => {
            println!("bad");
        }
    };

    Ok(())
}
