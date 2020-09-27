extern crate probabilistic_collections;
use chrono::prelude::*;
use chrono::{DateTime, TimeZone, Utc};
use dirs;
use glob::glob;
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

mod Indexer {
    use chrono::prelude::*;
    use lazy_static;
    use probabilistic_collections::similarity::{ShingleIterator, SimHash};
    use probabilistic_collections::SipHasherBuilder;
    use reqwest;
    use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
    use select::document;
    use std::collections::HashSet;
    use std::fs;
    use std::panic;
    use std::path::Path;
    use std::time::Duration;
    use tantivy::collector::TopDocs;
    use tantivy::query::QueryParser;
    use tantivy::schema::*;
    use tantivy::{doc, Index, ReloadPolicy};
    use triple_accel::hamming;
    fn directory(
        system_path: &str,
    ) -> Result<tantivy::directory::MmapDirectory, tantivy::directory::error::OpenDirectoryError>
    {
        let index_path = Path::new(system_path);
        if !index_path.is_dir() {
            fs::create_dir(index_path).expect("could not make index dir");
        }

        tantivy::directory::MmapDirectory::open(index_path)
    }

    pub fn hash_index() -> std::result::Result<tantivy::Index, tantivy::TantivyError> {
        // dont keep its own index? we are writing the domain and duplicate urls to prevent them
        // from reloading. just use that?
        let system_path = ".private_search_hashes";
        let directory = directory(&system_path);

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
        let directory = directory(&system_path);

        let mut schema_builder = Schema::builder();
        schema_builder.add_text_field("title", TEXT | STORED);
        schema_builder.add_text_field("url", TEXT | STORED);
        schema_builder.add_text_field("content", TEXT);
        schema_builder.add_text_field("domain", TEXT | STORED);
        schema_builder.add_text_field("context", TEXT);
        schema_builder.add_text_field("summary", TEXT | STORED);
        schema_builder.add_text_field("description", TEXT | STORED);
        schema_builder.add_i64_field("bookmarked", STORED | INDEXED);
        schema_builder.add_i64_field("duplicate", STORED | INDEXED);
        schema_builder.add_u64_field("content_hash", STORED | INDEXED);
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
    fn domain_hash(domain: &str) -> String {
        let digest = md5::compute(domain.as_bytes());
        format!("{:x}", digest)
    }
    pub fn add_hash(domain: &str, hash: u64) {
        let index = hash_index().expect("hash index");
        let searcher = searcher(&index);
        let mut index_writer = index.writer(50_000_000).expect("writer");
        let query_parser = QueryParser::for_index(
            &index,
            vec![index.schema().get_field("domain").expect("domain field")],
        );
        let domain_hash = domain_hash(&domain);
        let query = query_parser
            .parse_query(&format!("\"{}\"", &domain_hash))
            .expect("query parse for domain match");

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(1))
            .expect("search");
        dbg!(domain);

        let new_hash = format!("/{}", hash);
        let mut doc = if let Some(result) = top_docs.first() {
            let doc = searcher.doc(result.1).expect("doc");
            // dont dup the facet
            for s in doc
                .get_all(index.schema().get_field("hashes").expect("f"))
                .iter()
            {
                match s {
                    tantivy::schema::Value::Facet(facet) => {
                        if facet.to_path_string() == new_hash {
                            dbg!("already have the hash");
                            return;
                        }
                    }
                    _ => {}
                }
            }
            let frankenstein_isbn = Term::from_field_text(
                index.schema().get_field("domain").expect("domain field"),
                &domain_hash,
            );
            index_writer.delete_term(frankenstein_isbn.clone());
            doc
        } else {
            let mut doc = tantivy::Document::default();
            doc.add_text(
                index.schema().get_field("domain").expect("domain"),
                &domain_hash,
            );

            doc
        };

        doc.add_facet(
            index.schema().get_field("hashes").expect("hash"),
            Facet::from(&new_hash),
        );
        dbg!(&doc);

        index_writer.add_document(doc);
        index_writer.commit().expect("commit");
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

                    let description = match document
                        .find(select::predicate::Name("meta"))
                        .into_selection()
                        .filter(select::predicate::Attr("name", "description"))
                        .iter()
                        .nth(0)
                    {
                        Some(node) => node.text().to_string(),
                        _ => "".to_string(),
                    };

                    let body = match document.find(select::predicate::Name("body")).nth(0) {
                        Some(node) => node.text().split_whitespace().collect::<Vec<_>>().join(" "),
                        _ => {
                            // nothing to index
                            return;
                        }
                    };

                    let parsed = reqwest::Url::parse(&url).expect("url pase");

                    let sim_hash = SimHash::with_hasher(SipHasherBuilder::from_seed(0, 0));
                    //dbg!(&body);
                    let content_hash =
                        sim_hash.get_sim_hash(ShingleIterator::new(2, body.split(' ').collect()));
                    let dup = duplicate(&parsed.domain().unwrap().to_string(), &content_hash);
                    dbg!(&dup);

                    doc.add_u64(
                        index
                            .schema()
                            .get_field("content_hash")
                            .expect("content_hash"),
                        content_hash,
                    );
                    doc.add_text(index.schema().get_field("title").expect("title"), &title);

                    if !dup {
                        doc.add_text(index.schema().get_field("content").expect("content"), &body);
                        doc.add_text(
                            index
                                .schema()
                                .get_field("description")
                                .expect("description"),
                            &description,
                        );

                        let config = SummarizationConfig::default();

                        let result = panic::catch_unwind(|| {
                            let summarization_model =
                                SummarizationModel::new(config).expect("summarization_model fail");
                            let input = [body.as_str()];
                            summarization_model.summarize(&input).join(" ")
                        });

                        if result.is_ok() {
                            doc.add_text(
                                index.schema().get_field("summary").expect("summary"),
                                &result.unwrap(),
                            );
                        } else {
                            println!("sum error");
                        }
                    } else {
                        doc.add_i64(index.schema().get_field("duplicate").expect("duplicate"), 1);
                    }
                    doc.add_text(index.schema().get_field("url").expect("url"), &url);

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

                    add_hash(&parsed.domain().expect(("domian")), content_hash);
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

    pub fn duplicate(domain: &String, content_hash: &u64) -> bool {
        let index = hash_index().expect("hash index");
        let searcher = searcher(&index);
        let query_parser = QueryParser::for_index(
            &index,
            vec![index.schema().get_field("domain").expect("domain field")],
        );

        let domain_hash = domain_hash(&domain);
        let query = query_parser
            .parse_query(&format!("\"!{}\"", domain_hash))
            .expect("query parse for domain match");

        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(1))
            .expect("search");
        dbg!(domain);
        dbg!(domain_hash);
        let content_hash_bytes = content_hash.to_le_bytes();
        dbg!(&content_hash);
        for result in top_docs {
            for s in searcher
                .doc(result.1)
                .expect("doc")
                .get_all(index.schema().get_field("hashes").expect("f"))
                .iter()
            {
                match s {
                    tantivy::schema::Value::Facet(facet) => {
                        dbg!(&facet);
                        let hash_number = facet
                            .to_path()
                            .remove(0)
                            .parse::<i64>()
                            .unwrap_or(0)
                            .to_le_bytes();

                        //dbg!(&hash_number);
                        //dbg!(&content_hash_bytes);
                        let ham = hamming(&hash_number, &content_hash_bytes);
                        dbg!(&ham);
                        if ham < 4 {
                            return true;
                        }
                    }
                    _ => {}
                };
            }
        }
        false
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
        // need to load the doc to get the real url to compare vs input.
        // roots like www.google.com/ will show up for
        // www.google.com/?q=some_search
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
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let arg_path = args.get(1);
    let index = Indexer::search_index().unwrap();
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
