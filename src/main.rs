use chrono::prelude::*;
use reqwest;
use select::document;
use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{Index, ReloadPolicy};
fn search_index() -> std::result::Result<tantivy::Index, tantivy::TantivyError> {
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
    schema_builder.add_text_field("content", TEXT | STORED);
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

fn get_url(url: &String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    let res = client.get(url).send()?;
    let body = res.text()?;

    Ok(body)
}

fn index_url(url: String) {
    let index = search_index();
    match index {
        Ok(index) => {
            match get_url(&url) {
                Ok(body) => {
                    let document = document::Document::from(body.as_str());
                    let mut doc = tantivy::Document::default();
                    let title = match document.find(select::predicate::Name("title")).nth(0) {
                        Some(node) => node.text().to_string(),
                        _ => "".to_string(),
                    };

                    let body = match document.find(select::predicate::Name("body")).nth(0) {
                        Some(node) => node.text(),
                        _ => "".to_string(),
                    };

                    doc.add_text(index.schema().get_field("title").expect("title"), &title);
                    doc.add_text(index.schema().get_field("content").expect("content"), &body);
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

                    let local: DateTime<Utc> = Utc::now();
                    doc.add_date(
                        index.schema().get_field("added_at").expect("added_at"),
                        &local,
                    );

                    doc.add_date(
                        index.schema().get_field("added_at").expect("added_at"),
                        &local,
                    );
                    doc.add_i64(index.schema().get_field("pinned").expect("pinned"), 0);
                    doc.add_i64(
                        index
                            .schema()
                            .get_field("accessed_count")
                            .expect("accessed_count"),
                        1,
                    );
                    doc.add_i64(
                        index.schema().get_field("bookmarked").expect("bookmarked"),
                        0,
                    );

                    let mut index_writer = index.writer(50_000_000).expect("writer");
                    index_writer.add_document(doc);
                    index_writer.commit().expect("commit");
                }
                _ => {}
            };
        }
        _ => {}
    }
}

fn main() -> tantivy::Result<()> {
    index_url("https://docs.rs/chrono/0.4.15/chrono/".to_string());
    let index = search_index();
    match index {
        Ok(index) => {
            let reader = index
                .reader_builder()
                .reload_policy(ReloadPolicy::Manual)
                .try_into()?;

            let searcher = reader.searcher();

            let query_parser = QueryParser::for_index(
                &index,
                vec![index.schema().get_field("content").expect("content field")],
            );

            let query = query_parser.parse_query("chrono")?;
            let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
            for (_score, doc_address) in top_docs {
                let retrieved_doc = searcher.doc(doc_address)?;
                println!("{}", index.schema().to_json(&retrieved_doc));
            }
        }
        Err(_) => println!("count not access index"),
    }

    Ok(())
}
