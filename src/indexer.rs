use chrono::prelude::*;

use glob::glob;
use probabilistic_collections::similarity::{ShingleIterator, SimHash};
use probabilistic_collections::SipHasherBuilder;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use select::document;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::panic;
use std::path::Path;
use std::time::Duration;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, ReloadPolicy};
use triple_accel::hamming;



pub enum GetterResults {
    Html(String),
    Text(String),
    Nothing,
}
pub trait IndexGetter {
    fn get_url(&self, url: &str) -> GetterResults {
        dbg!(&url);
        let agent = ureq::Agent::default().build();
        let res = agent.get(url).timeout(Duration::new(10, 0)).call();

        if let Some(lower) = res.header("Content-Type") {
            dbg!(&lower);
            let lower = lower.to_lowercase();
            if lower == ""
                || lower.contains("html")
                || (lower.contains("text") && !lower.contains("javascript"))
            {
                GetterResults::Html(res.into_string().unwrap_or("".to_string()))
            } else {
                GetterResults::Nothing
            }
        } else {
            GetterResults::Nothing
        }
    }
}
pub struct NoAuthBlockingGetter {}
impl IndexGetter for NoAuthBlockingGetter {}

#[derive(Serialize, Debug, Deserialize)]
pub struct SystemSettings {
    pub port: String,
    pub ignore_domains: Vec<String>,
    pub indexer_enabled: bool,
    pub ignore_strings: Vec<String>,
}

impl Default for SystemSettings {
    fn default() -> Self {
        SystemSettings {
            port: "7172".to_string(),
            ignore_strings: vec![],
            indexer_enabled: false,
            ignore_domains: vec![
                "//127.0.0.1".to_string(),
                "//192.168.".to_string(),
                ".lvh.me".to_string(),
                "//0.0.0.0".to_string(),
                "//lvh.me".to_string(),
                "//localhost/".to_string(),
                "//localhost:".to_string(),
                "google.com/".to_string(),
                "youtube.com/".to_string(),
                "ebay.com/".to_string(),
                "aha.io/".to_string(),
                "newrelic.com/".to_string(),
                "datadoghq.com/".to_string(),
                "amazon.com/".to_string(),
                "woot.com/".to_string(),
                "imgur.com".to_string(),
                "gstatic.com/".to_string(),
            ],
        }
    }
}
use std::env;
lazy_static::lazy_static! {
    pub static ref CACHEDCONFIG: SystemSettings = read_settings();
    pub static ref BASE_INDEX_DIR: String = match env::var("PS_INDEX_DIRECTORY") {
        Ok(val) => {
            if val.ends_with('/'){
                val
            }else{
                format!("{}/",val)
            }
        },
        Err(_) => {
            let home = dirs::home_dir().unwrap();
            let home = home.join(".config");

            if !home.is_dir() {
               fs::create_dir(&home).expect("could not make index dir");
            }
            home.join("private_search").to_str().unwrap().to_string()
        }
    };
}

pub fn write_settings(config: &SystemSettings) {
    let path_name = format!("{}/server_settings.toml", BASE_INDEX_DIR.to_string());
    create_directory(&BASE_INDEX_DIR);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path_name);
    file.expect("setting file write")
        .write_all(toml::to_string(&config).unwrap().as_bytes())
        .expect("file");
}

pub fn read_settings() -> SystemSettings {
    let path_name = format!("{}/server_settings.toml", BASE_INDEX_DIR.to_string());
    dbg!(&path_name);
    create_directory(&BASE_INDEX_DIR);
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
    dbg!(&s);
    if !s.is_empty() {
        let config: SystemSettings = toml::from_str(&s).expect("bad config parse");
        config
    } else {
        SystemSettings::default()
    }
}

fn create_directory(system_path: &str) {
    let index_path = Path::new(system_path);
    let paths = vec![
        index_path.join(""),
        index_path.join("source"),
        index_path.join("index"),
        index_path.join("hashes"),
    ];

    for path in paths {
        if !path.is_dir() {
            fs::create_dir(path).expect("could not make index dir");
        }
    }
}

fn index_directory(
) -> Result<tantivy::directory::MmapDirectory, tantivy::directory::error::OpenDirectoryError> {
    create_directory(&BASE_INDEX_DIR);
    let index_path = Path::new(BASE_INDEX_DIR.as_str());

    tantivy::directory::MmapDirectory::open(index_path.join("index"))
}

fn hash_directory(
) -> Result<tantivy::directory::MmapDirectory, tantivy::directory::error::OpenDirectoryError> {
    create_directory(&BASE_INDEX_DIR);
    let index_path = Path::new(BASE_INDEX_DIR.as_str());

    tantivy::directory::MmapDirectory::open(index_path.join("hashes"))
}

pub fn hash_index(system_path: &str) -> std::result::Result<tantivy::Index, tantivy::TantivyError> {
    // dont keep its own index? we are writing the domain and duplicate urls to prevent them
    // from reloading. just use that?
    let directory = hash_directory();

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("domain", TEXT | STORED);
    schema_builder.add_facet_field("hashes");

    let schema = schema_builder.build();
    match directory {
        Ok(dir) => Index::open_or_create(dir, schema),
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
    let directory = index_directory();

    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("id", STRING | STORED);
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("url", TEXT | STORED);
    schema_builder.add_text_field("content", TEXT);
    schema_builder.add_text_field("domain", TEXT | STORED);
    schema_builder.add_text_field("context", TEXT);
    schema_builder.add_text_field("summary", STORED);
    schema_builder.add_text_field("description", STORED);
    schema_builder.add_i64_field("bookmarked", STORED | INDEXED);
    schema_builder.add_i64_field("duplicate", STORED | INDEXED);
    schema_builder.add_i64_field("content_hash", STORED | INDEXED);
    schema_builder.add_i64_field("pinned", STORED | INDEXED);
    schema_builder.add_i64_field("accessed_count", STORED);
    schema_builder.add_date_field("added_at", STORED);
    schema_builder.add_date_field("last_accessed_at", STORED | INDEXED);
    schema_builder.add_facet_field("tags");

    let schema = schema_builder.build();
    match directory {
        Ok(dir) => Index::open_or_create(dir, schema),
        Err(_) => {
            println!("dir not found");
            Err(tantivy::TantivyError::SystemError(format!(
                "could not open index directory {}",
                BASE_INDEX_DIR.as_str()
            )))
        }
    }
}

pub fn get_url(url: &str) -> Result<String, ureq::Error> {
    let agent = ureq::Agent::default().build();
    let res = agent.get(url).timeout(Duration::new(10, 0)).call();

    let body = if let Some(lower) = res.header("Content-Type") {
        dbg!(&lower);
        let lower = lower.to_lowercase();
        if lower == ""
            || lower.contains("html")
            || (lower.contains("text") && !lower.contains("javascript"))
        {
            res.into_string().unwrap_or("".to_string())
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

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

#[derive(Debug, Clone, Default)]
pub struct UrlMeta {
    pub url: Option<String>,
    pub title: Option<String>,
    pub bookmarked: Option<bool>,
    pub last_visit: Option<DateTime<Utc>>,
    pub tags_add: Option<Vec<String>>,
    pub tags_remove: Option<Vec<String>>,
    pub pinned: Option<i64>,
    pub access_count: Option<i64>,
    pub hidden: Option<i64>,
}

pub fn url_skip(url: &str) -> bool {
    // lazy static this
    let parsed = url::Url::parse(&url).expect("url pase");
    if !parsed.scheme().starts_with("http") {
        true
    } else {
        CACHEDCONFIG.ignore_domains.iter().any(|s| {
            if s.ends_with('$') {
                let mut x = s.clone();
                x.pop();
                url.ends_with(&x)
            } else {
                url.contains(s)
            }
        })
    }
}

pub fn md5_hash(domain: &str) -> String {
    let digest = md5::compute(domain.as_bytes());
    format!("{:x}", digest)
}

pub fn add_hash(domain: &str, hash: u64) {
    let index = hash_index(&BASE_INDEX_DIR).expect("hash index");
    let searcher = searcher(&index);
    let mut index_writer = index.writer(50_000_000).expect("writer");
    let query_parser = QueryParser::for_index(
        &index,
        vec![index.schema().get_field("domain").expect("domain field")],
    );
    let domain_hash = md5_hash(&domain);
    let query = query_parser
        .parse_query(&format!("\"{}\"", &domain_hash))
        .expect("query parse for domain match");

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(1))
        .expect("search");

    let new_hash = format!("/{}", hash);
    let mut doc = if let Some(result) = top_docs.first() {
        let doc = searcher.doc(result.1).expect("doc");
        // dont dup the facet
        for s in doc
            .get_all(index.schema().get_field("hashes").expect("f"))
            .iter()
        {
            if let tantivy::schema::Value::Facet(facet) = s {
                if facet.to_path_string() == new_hash {
                    return;
                }
            }
        }
        let frankenstein_isbn = Term::from_field_text(
            index.schema().get_field("domain").expect("domain field"),
            &domain_hash,
        );
        index_writer.delete_term(frankenstein_isbn);
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

    index_writer.add_document(doc);
    index_writer.commit().expect("commit");
    index_writer.wait_merging_threads().expect("merge");
}
pub fn update_document(url_hash: &str, index: &Index, meta: UrlMeta) -> Document {
    let json_string = read_source(url_hash);
    let mut json: Value = serde_json::from_str(&json_string).expect("cached json parse fail!");
    for keyword in meta.tags_add.unwrap_or_default() {
        let value = serde_json::Value::String(keyword.clone());
        if let Some(words) = json.get_mut("tags") {
            if let Some(array) = words.as_array_mut() {
                if !array.contains(&value) {
                    array.push(value);
                }
            }
        } else {
            json["tags"] = serde_json::Value::Array(vec![value]);
        }
    }

    for keyword in meta.tags_remove.unwrap_or_default() {
        if let Some(words) = json.get_mut("tags") {
            if let Some(array) = words.as_array_mut() {
                let value = serde_json::Value::String(keyword.clone());
                if let Some(index) = array.iter().position(|i| i == &value) {
                    array.remove(index); // remove the element at the position index (2)
                }
            }
        }
    }
    if let Some(last_visit) = meta.last_visit {
        *json.get_mut("last_accessed_at").unwrap() = json!(last_visit.to_rfc3339());
    }

    if let Some(pinned) = meta.pinned {
        *json.get_mut("pinned").unwrap() = json!(vec![pinned]);
    }

    if let Some(hidden) = meta.hidden {
        *json.get_mut("hidden").unwrap() = json!(vec![hidden]);
    }

    if let Some(accessed_count) = meta.access_count {
        *json.get_mut("accessed_count").unwrap() = json!(vec![accessed_count]);
    }

    if let Some(bookmarked) = meta.bookmarked {
        let bookmarked = if bookmarked { 1 } else { 0 };
        *json.get_mut("accessed_count").unwrap() = json!(vec![bookmarked]);
    }
    index
        .schema()
        .parse_document(&json.to_string())
        .expect("doc from json")
}

pub fn update_cached(
    url_hash: &str,
    index: &Index,
    meta: UrlMeta,
    index_writer: &mut tantivy::IndexWriter,
) {
    let doc = update_document(url_hash, index, meta);
    let json = index.schema().to_json(&doc);
    index_writer.add_document(doc);
    index_writer.commit().expect("commit");
    write_source(url_hash, json);
}
pub fn remote_index(url: &str, index: &Index, meta: UrlMeta, getter: impl IndexGetter) {
    let url_hash = md5_hash(&url);
    let parsed = url::Url::parse(&url).expect("url pase");

    let mut doc = tantivy::Document::default();
    match getter.get_url(&url) {
        GetterResults::Html(body) => {
            println!("processing {}", &url);
            let document = document::Document::from(body.as_str());
            let title = match document.find(select::predicate::Name("title")).next() {
                Some(node) => node.text(),
                _ => meta.title.unwrap_or_else(|| "".to_string()),
            };

            let meta_description = document
                .find(select::predicate::Name("meta"))
                .filter(|node| node.attr("name").unwrap_or("") == "description")
                .filter_map(|n| n.attr("content"))
                .map(str::to_string)
                .collect::<Vec<String>>();
            let empty = "".to_string();
            let description = match meta_description.first() {
                Some(node) => node,
                _ => &empty,
            };

            let body = match document.find(select::predicate::Name("body")).next() {
                Some(node) => node.text().split_whitespace().collect::<Vec<_>>().join(" "),
                _ => {
                    // nothing to index
                    return;
                }
            };
            if body.split_whitespace().nth(100).is_some() {
                let sim_hash = SimHash::with_hasher(SipHasherBuilder::from_seed(0, 0));
                let content_hash =
                    sim_hash.get_sim_hash(ShingleIterator::new(2, body.split(' ').collect()));
                let dup = duplicate(&parsed.domain().unwrap().to_string(), &content_hash);

                doc.add_i64(
                    index
                        .schema()
                        .get_field("content_hash")
                        .expect("content_hash"),
                    content_hash.try_into().unwrap_or(0),
                );
                add_hash(&parsed.domain().expect("domain"), content_hash);

                if !dup {
                    doc.add_text(index.schema().get_field("content").expect("content"), &body);
                    let config = SummarizationConfig::default();
                    if config.device.is_cuda() {
                        let result = panic::catch_unwind(|| {
                            let summarization_model =
                                SummarizationModel::new(config).expect("summarization_model fail");
                            let input = [body.as_str()];
                            summarization_model.summarize(&input).join(" ")
                        });

                        match result {
                            Ok(results) => {
                                doc.add_text(
                            index.schema().get_field("summary").expect("summary"),
                            &results.replace("Please email your photos to jennifer.smith@mailonline.co.uk. Send us photos of your family and pets. Visit CNN.com/sport for more photos and videos of family and friends in the U.S.", "").trim(),
                        );
                            }
                            _ => {
                                println!("sum error");
                            }
                        };
                    } else {
                        let mut short_body = body;
                        let mut new_len = 150;
                        // prevent panics by finding a safe spot to slice
                        while !short_body.is_char_boundary(new_len) {
                            new_len += 1;
                        }
                        short_body.truncate(new_len);
                        doc.add_text(
                            index.schema().get_field("summary").expect("summary"),
                            &short_body,
                        );
                    }
                } else {
                    doc.add_i64(index.schema().get_field("duplicate").expect("duplicate"), 1);
                }
            } else {
                // add the text anyway its small even if it is a dup
                doc.add_text(index.schema().get_field("content").expect("content"), &body);
            }

            doc.add_text(
                index
                    .schema()
                    .get_field("description")
                    .expect("description"),
                &description,
            );
            doc.add_text(index.schema().get_field("title").expect("title"), &title);
            doc.add_text(index.schema().get_field("url").expect("url"), &url);

            doc.add_text(
                index.schema().get_field("domain").expect("domain"),
                parsed.domain().unwrap_or(""),
            );
            let _found_urls = document
                .find(select::predicate::Name("a"))
                .filter_map(|n| n.attr("href"))
                .map(str::to_string)
                .collect::<HashSet<String>>();

            let keywords = document
                .find(select::predicate::Name("meta"))
                .filter(|node| node.attr("name").unwrap_or("") == "keywords")
                .filter_map(|n| n.attr("content"))
                .flat_map(|s| s.split(','))
                .map(str::to_string)
                .collect::<Vec<String>>();

            for keyword in keywords {
                doc.add_facet(
                    index.schema().get_field("tags").expect("tags"),
                    Facet::from(&format!("/keywords/{}", keyword.trim())),
                );
            }
            for keyword in meta.tags_add.unwrap_or_default() {
                doc.add_facet(
                    index.schema().get_field("tags").expect("tags"),
                    Facet::from(&keyword.to_string()),
                );
            }
        }
        _ => {}
    }
    doc.add_text(index.schema().get_field("url").expect("url"), &url);

    doc.add_text(
        index.schema().get_field("domain").expect("domain"),
        parsed.domain().unwrap_or(""),
    );
    doc.add_date(
        index.schema().get_field("added_at").expect("added_at"),
        &Utc::now(),
    );

    let last_visit: DateTime<Utc> = meta.last_visit.unwrap_or_else(Utc::now);
    doc.add_date(
        index
            .schema()
            .get_field("last_accessed_at")
            .expect("added_at"),
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
    doc.add_text(index.schema().get_field("id").expect("id"), &url_hash);
    let json = index.schema().to_json(&doc);

    let mut index_writer = index.writer(50_000_000).expect("writer");
    index_writer.add_document(doc);
    index_writer.commit().expect("commit");
    index_writer.wait_merging_threads().expect("merge");

    write_source(&url_hash, json);
}
pub fn index_url(url: String, meta: UrlMeta, index: Option<&Index>, getter: impl IndexGetter) {
    if CACHEDCONFIG.indexer_enabled {
        let i;
        let index = match index {
            Some(index) => index,
            None => {
                i = search_index().unwrap();
                &i
            }
        };

        let url_hash = md5_hash(&url);
        println!("indexing {} {}", &url_hash, &url);
        if url_skip(&url) {
            println!("skip {}", url);
        } else if let Some(_doc_address) = find_url(&url, &index) {
            println!("have {}", url);
        } else if source_exists(&url_hash) {
            println!("cached file {}", url);
            let mut index_writer = index.writer(50_000_000).expect("writer");
            update_cached(&url_hash, &index, meta, &mut index_writer);
            index_writer.wait_merging_threads().expect("merge");
        } else {
            let parsed = url::Url::parse(&url).expect("url pase");

            // covers ip only domains
            if parsed.domain().is_none() {
                return;
            }
            remote_index(&url, &index, meta, getter)
        };
    }
}
pub fn source_exists(filename: &str) -> bool {
    let index_path = Path::new(BASE_INDEX_DIR.as_str());
    let source_path = index_path.join("source");
    source_path.join(format!("{}.jsonc", filename)).exists()
}
pub fn write_source(url_hash: &str, json: String) {
    let index_path = Path::new(BASE_INDEX_DIR.as_str());
    let source_path = index_path.join("source");
    let output = File::create(source_path.join(format!("{}.jsonc", url_hash))).expect("write file");
    let mut writer = brotli::CompressorWriter::new(output, 4096, 11, 22);
    writer
        .write_all(json.as_bytes())
        .expect("write source file");
    //output.write_all(json.as_bytes()).expect("write");
}
pub fn read_source(url_hash: &str) -> String {
    let index_path = Path::new(BASE_INDEX_DIR.as_str());
    let source_path = index_path.join("source");
    let input = File::open(source_path.join(format!("{}.jsonc", url_hash)))
        .unwrap_or_else(|_| panic!("read source {}", url_hash));

    let mut reader = brotli::Decompressor::new(
        input, 4096, // buffer size
    );
    let mut json = String::new();
    reader.read_to_string(&mut json).expect("read source file");
    json
}

pub fn duplicate(domain: &str, content_hash: &u64) -> bool {
    let index = hash_index(BASE_INDEX_DIR.as_str()).expect("hash index");
    let searcher = searcher(&index);
    let query_parser = QueryParser::for_index(
        &index,
        vec![index.schema().get_field("domain").expect("domain field")],
    );

    let domain_hash = md5_hash(&domain);
    let query = query_parser
        .parse_query(&format!("\"!{}\"", domain_hash))
        .expect("query parse for domain match");

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(1))
        .expect("search");
    let content_hash_bytes = content_hash.to_le_bytes();
    for result in top_docs {
        for s in searcher
            .doc(result.1)
            .expect("doc")
            .get_all(index.schema().get_field("hashes").expect("f"))
            .iter()
        {
            if let tantivy::schema::Value::Facet(facet) = s {
                let hash_number = facet
                    .to_path()
                    .remove(0)
                    .parse::<i64>()
                    .unwrap_or(0)
                    .to_le_bytes();

                let ham = hamming(&hash_number, &content_hash_bytes);
                if ham < 4 {
                    return true;
                }
            };
        }
    }
    false
}
// move over to id hash
pub fn find_url(url: &str, index: &Index) -> std::option::Option<tantivy::DocAddress> {
    let searcher = searcher(&index);

    let url_hash = md5_hash(url);
    let query_parser = QueryParser::for_index(
        &index,
        vec![index.schema().get_field("id").expect("idfield")],
    );

    let query = query_parser
        .parse_query(&format!("\"{}\"", url_hash))
        .expect("query parse for url match");
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(1))
        .expect("search");
    // need to load the doc to get the real url to compare vs input.
    // roots like www.google.com/ will show up for
    // www.google.com/?q=some_search
    match top_docs.get(0) {
        Some((_, doc_address)) => Some(*doc_address),
        _ => None,
    }
}

pub fn backfill_from_cached() {
    let path_name = ".private_search/source".to_string();
    let entries = glob(&format!("{}/*.jsonc", path_name)).expect("Failed to read glob pattern");
    let mut counter = 0;

    let index = search_index().unwrap();
    let mut index_writer = index.writer(50_000_000).expect("writer");
    for entry in entries {
        if let Ok(file) = entry {
            {
                if counter % 10000 == 0 {
                    println!("commited {}", counter);
                    index_writer.commit().expect("commit");
                }
            }
            let file_string = file.to_str().expect("file_path");
            let url_hash = file_string.replace(".jsonc", "");
            let url_hash = url_hash.split('/').last().unwrap();
            let doc = update_document(&url_hash, &index, UrlMeta::default());
            index_writer.add_document(doc);
            counter += 1;
        }
    }
    index_writer.commit().expect("last commit");
    index_writer.wait_merging_threads().expect("merge");
}
