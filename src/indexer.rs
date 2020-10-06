use chrono::prelude::*;
use probabilistic_collections::similarity::{ShingleIterator, SimHash};
use probabilistic_collections::SipHasherBuilder;
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};
use select::document;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;
use std::fs::File;
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

fn create_directory(system_path: &str) {
    let index_path = Path::new(system_path);
    if !index_path.is_dir() {
        fs::create_dir(index_path).expect("could not make index dir");
        fs::create_dir(index_path.join("source")).expect("could not make index dir");
        fs::create_dir(index_path.join("index")).expect("could not make index dir");
        fs::create_dir(index_path.join("hashes")).expect("could not make index dir");
    }
}

fn index_directory(
    system_path: &str,
) -> Result<tantivy::directory::MmapDirectory, tantivy::directory::error::OpenDirectoryError> {
    create_directory(system_path);
    let index_path = Path::new(system_path);

    tantivy::directory::MmapDirectory::open(index_path.join("index"))
}

fn hash_directory(
    system_path: &str,
) -> Result<tantivy::directory::MmapDirectory, tantivy::directory::error::OpenDirectoryError> {
    create_directory(system_path);
    let index_path = Path::new(system_path);

    tantivy::directory::MmapDirectory::open(index_path.join("hashes"))
}

pub fn hash_index(system_path: &str) -> std::result::Result<tantivy::Index, tantivy::TantivyError> {
    // dont keep its own index? we are writing the domain and duplicate urls to prevent them
    // from reloading. just use that?
    let directory = hash_directory(&system_path);

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
    let system_path = ".private_search";
    let directory = index_directory(&system_path);

    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("id", TEXT | STORED);
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
                system_path
            )))
        }
    }
}

// doesnt work. updates need a full rewrite.
pub fn pin_url(url: &str, pinned: i8) {
    let index = search_index().expect("search index");
    let searcher = searcher(&index);
    let mut index_writer = index.writer(50_000_000).expect("writer");
    let query_parser = QueryParser::for_index(
        &index,
        vec![index.schema().get_field("url").expect("domain field")],
    );
    let query = query_parser
        .parse_query(&format!("\"{}\"", url))
        .expect("query parse for domain match");

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(1))
        .expect("search");

    let _doc = if let Some(result) = top_docs.first() {
        let mut doc = searcher.doc(result.1).expect("doc");
        let old_doc =
            Term::from_field_text(index.schema().get_field("url").expect("domain field"), &url);
        index_writer.delete_term(old_doc);

        dbg!(&doc);
        doc.add_i64(
            index.schema().get_field("pinned").expect("pinned"),
            pinned.into(),
        );
        dbg!(index.schema().get_field("pinned").expect("pinned"));
        dbg!(pinned);
        dbg!(&doc);
        index_writer.add_document(doc);
        index_writer.commit().expect("commit");
    };
}

pub fn get_url(url: &str) -> Result<String, reqwest::Error> {
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

#[derive(Debug, Clone, Default)]
pub struct UrlMeta {
    pub title: Option<String>,
    pub bookmarked: Option<bool>,
    pub last_visit: Option<DateTime<Utc>>,
    pub keywords: Option<Vec<String>>,
    pub pinned: Option<i64>,
    pub access_count: Option<i64>,
}

pub fn url_skip(url: &str) -> bool {
    // lazy static this

    let parsed = reqwest::Url::parse(&url).expect("url pase");

    let ignore_includes = vec![
        "//127.0.0.1",
        "//192.168.",
        ".lvh.me",
        "//0.0.0.0",
        "//lvh.me",
        "google.com/",
        "ebay.com/",
        "aha.io/",
        "newrelic.com/",
        "datadoghq.com/",
        "amazon.com/",
        "woot.com/",
        "imgur.com",
        "gstatic.com/",
    ];
    let ignore_starts = vec!["moz-extension://"];
    if !parsed.scheme().starts_with("http") || ignore_starts.iter().any(|s| url.starts_with(s)) {
        true
    } else {
        ignore_includes.iter().any(|s| url.contains(s))
    }
}

fn md5_hash(domain: &str) -> String {
    let digest = md5::compute(domain.as_bytes());
    format!("{:x}", digest)
}

pub fn add_hash(domain: &str, hash: u64) {
    let system_path = ".private_search";
    let index = hash_index(system_path).expect("hash index");
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
}

pub fn update_cached(url_hash: &str, index: &Index, meta: UrlMeta) {
    let json_string = read_source(url_hash);
    let mut json: Value = serde_json::from_str(&json_string).expect("cached json parse fail!");
    for keyword in meta.keywords.unwrap_or_default() {
        let words = json.get_mut("keywords").expect("keywords");
        if let Some(array) = words.as_array_mut() {
            let value = serde_json::Value::String(keyword.clone());
            if !array.contains(&value) {
                array.push(value);
            }
        }
    }
    if let Some(last_visit) = meta.last_visit {
        *json.get_mut("last_accessed_at").unwrap() = json!(last_visit.to_rfc3339());
    }

    if let Some(pinned) = meta.pinned {
        *json.get_mut("pinned").unwrap() = json!(vec![pinned]);
    }

    if let Some(accessed_count) = meta.access_count {
        *json.get_mut("accessed_count").unwrap() = json!(vec![accessed_count]);
    }

    if let Some(bookmarked) = meta.bookmarked {
        let bookmarked = if bookmarked { 1 } else { 0 };
        *json.get_mut("accessed_count").unwrap() = json!(vec![bookmarked]);
    }
    let doc = index
        .schema()
        .parse_document(&json.to_string())
        .expect("doc from json");

    let json = index.schema().to_json(&doc);
    let mut index_writer = index.writer(50_000_000).expect("writer");
    index_writer.add_document(doc);
    index_writer.commit().expect("commit");
    write_source(url_hash, json);
}
pub fn remote_index(url: &str, index: &Index, meta: UrlMeta) {
    let url_hash = md5_hash(&url);
    let parsed = reqwest::Url::parse(&url).expect("url pase");
    match get_url(&url) {
        Ok(body) => {
            println!("processing {}", &url);
            let document = document::Document::from(body.as_str());
            let mut doc = tantivy::Document::default();
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
            for keyword in meta.keywords.unwrap_or_default() {
                doc.add_facet(
                    index.schema().get_field("tags").expect("tags"),
                    Facet::from(&format!("/keywords/{}", keyword)),
                );
            }

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

            write_source(&url_hash, json);
        }

        Err(e) => {
            dbg!(e);
        }
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

    let url_hash = md5_hash(&url);
    println!("indexing {} {}", &url_hash, &url);
    if url_skip(&url) {
        println!("skip {}", url);
    } else if let Some(_doc_address) = find_url(&url, &index) {
        println!("have {}", url);
    } else if source_exists(&url_hash) {
        println!("cached file {}", url);
        update_cached(&url_hash, &index, meta);
    } else {
        let parsed = reqwest::Url::parse(&url).expect("url pase");

        // covers ip only domains
        if parsed.domain().is_none() {
            return;
        }
        remote_index(&url, &index, meta)
    };
}
pub fn source_exists(filename: &str) -> bool {
    let system_path = ".private_search";
    let index_path = Path::new(system_path);
    let source_path = index_path.join("source");
    source_path.join(format!("{}.jsonc", filename)).exists()
}
pub fn write_source(url_hash: &str, json: String) {
    let system_path = ".private_search";
    let index_path = Path::new(system_path);
    let source_path = index_path.join("source");
    let output = File::create(source_path.join(format!("{}.jsonc", url_hash))).expect("write file");
    let mut writer = brotli::CompressorWriter::new(output, 4096, 11, 22);
    writer.write_all(json.as_bytes());
    //output.write_all(json.as_bytes()).expect("write");
}
pub fn read_source(url_hash: &str) -> String {
    let system_path = ".private_search";
    let index_path = Path::new(system_path);
    let source_path = index_path.join("source");
    let input = File::open(source_path.join(format!("{}.jsonc", url_hash))).expect("write file");

    let mut reader = brotli::Decompressor::new(
        input, 4096, // buffer size
    );
    let mut json = String::new();
    reader.read_to_string(&mut json);
    json
}

pub fn duplicate(domain: &str, content_hash: &u64) -> bool {
    let system_path = ".private_search";
    let index = hash_index(system_path).expect("hash index");
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

pub fn find_url(url: &str, index: &Index) -> std::option::Option<tantivy::DocAddress> {
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
    match top_docs.get(0) {
        Some((_, doc_address)) => Some(*doc_address),
        _ => None,
    }
}
