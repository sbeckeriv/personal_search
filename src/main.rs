use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
//use termimad;

mod indexer;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(long = "query", name = "query")]
    query: Option<String>,
    #[structopt(long = "import_url")]
    import_url: Option<String>,
    #[structopt(long = "facet")]
    facet: Option<String>,
    #[structopt(long = "facet_field")]
    facet_field: Option<String>,
    #[structopt(long = "json_source")]
    json_source: Option<String>,
    #[structopt(long = "show")]
    show: Option<String>,
    #[structopt(long = "debug")]
    debug: bool,
    #[structopt(long = "backfillcached")]
    backfillcached: bool,
    #[structopt(long = "movecachefiles")]
    movecachefiles: bool,
    #[structopt(short = "s", long = "silent")]
    silent: bool,
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(long = "search_folder")]
    #[structopt(parse(from_os_str))]
    search_folder_path: Option<PathBuf>,
}
use tantivy::collector::FacetCollector;

use tantivy::query::AllQuery;
use tantivy::schema::Facet;
fn facets(index: tantivy::Index, field: &str, facet: &str) {
    let searcher = indexer::searcher(&index);
    let tags = index
        .schema()
        .get_field(field)
        .unwrap_or_else(|| panic!("{} not a field", field));
    let mut facet_collector = FacetCollector::for_field(tags);
    facet_collector.add_facet(facet);
    let facet_counts = searcher.search(&AllQuery, &facet_collector).expect("facet");

    // This lists all of the facet counts
    let facets: Vec<(&Facet, u64)> = facet_counts.get(facet).collect();
    dbg!(facets);
}
fn search(query: String, index: tantivy::Index, _debug: bool) {
    let searcher = indexer::searcher(&index);
    let default_fields: Vec<tantivy::schema::Field> = index
        .schema()
        .fields()
        .filter(|&(_, ref field_entry)| match *field_entry.field_type() {
            tantivy::schema::FieldType::Str(ref text_field_options) => {
                text_field_options.get_indexing_options().is_some()
            }
            _ => false,
        })
        .map(|(field, _)| field)
        .collect();

    let query_parser = QueryParser::new(index.schema(), default_fields, index.tokenizers().clone());

    let query = query_parser.parse_query(&query).expect("query parse");
    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(10))
        .expect("serach");
    let schema = index.schema();
    for (score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address).expect("doc");
        let mut m = HashMap::new();
        for f in retrieved_doc.field_values().iter() {
            m.entry(schema.get_field_name(f.field()))
                .or_insert_with(Vec::new)
                .push(f.value())
        }

        let m: HashMap<_, _> = retrieved_doc
            .get_sorted_field_values()
            .into_iter()
            .map(|a| (schema.get_field_name(a.0), a.1))
            .collect();

        let title = m
            .get("title")
            .and_then(|r| r.first().unwrap().value().text())
            .unwrap_or("");
        let url = m
            .get("url")
            .and_then(|r| r.first().unwrap().value().text())
            .unwrap_or("");
        let summary = m
            .get("summary")
            .and_then(|r| r.first().unwrap().value().text())
            .unwrap_or("");

        let pinned = m
            .get("pinned")
            .map(|r| r.first().unwrap().value().i64_value())
            .unwrap()
            .unwrap_or(0);
        println!(
            "{score}: {title} - {url}\n{summary}\n{pinned}",
            score = score,
            title = title,
            url = url,
            summary = summary,
            pinned = pinned
        );
        let json = index.schema().to_json(&retrieved_doc);
        println!("{}:\n{}", score, json);
    }
}

// should never need this again. move from flat folder to sub dir
fn movefiles() {
    use glob::glob;
    use std::path::Path;
    let path = Path::new(indexer::BASE_INDEX_DIR.as_str());
    let path_name = path.join("source");
    dbg!(&path_name);
    let entries = glob(&format!(
        "{}/*.jsonc",
        path_name.to_str().expect("source_dir")
    ))
    .expect("Failed to read glob pattern");

    for entry in entries {
        if let Ok(file) = entry {
            let base = file.parent().unwrap();
            let dir_name = file.to_str().unwrap().to_string();

            let filename = dir_name.split('/').last().unwrap();
            dbg!(filename);
            let mut new_dir_name = filename.to_string();
            new_dir_name.truncate(2);
            dbg!(&new_dir_name);
            dbg!(base.join(new_dir_name.clone()));
            if std::fs::create_dir(base.join(new_dir_name.clone())).is_ok() {}
            let dir_path = base.join(new_dir_name.clone());
            dbg!(dir_path.join(filename));
            if std::fs::rename(file, dir_path.join(filename)).is_ok() {}
        }
    }
}

fn main() -> tantivy::Result<()> {
    let index = indexer::search_index();

    let opt = Opt::from_args();

    match index {
        Ok(index) => {
            if opt.movecachefiles {
                movefiles();
            } else if opt.backfillcached {
                indexer::backfill_from_cached();
            } else if let Some(query) = opt.query {
                search(query, index, opt.debug);
            } else if let Some(url) = opt.show {
                let url = if url.contains('.') {
                    indexer::md5_hash(&url)
                } else {
                    url
                };

                if let Some(json_string) = indexer::read_source(&url) {
                    let json: Result<serde_json::Value, _> = serde_json::from_str(&json_string);
                    if let Ok(json) = json {
                        if let Some(content) = json.get("content_raw") {
                            if let serde_json::Value::Array(content) = content {
                                let content = indexer::view_body(content[0].as_str().unwrap_or(""));
                                dbg!(content);
                                //let markdown = html2md::parse_html(&content);
                                //termimad::print_inline(&markdown);
                            }
                        } else if let Some(content) = json.get("content") {
                            if let serde_json::Value::Array(content) = content {
                                let content = content[0].as_str().unwrap_or("");
                                if !content.contains("<html>") {
                                    println!("{}", content);
                                } else {
                                    dbg!(content);
                                    //let markdown = html2md::parse_html(&content);
                                    //termimad::print_inline(&markdown);
                                }
                            }
                        } else {
                        }
                    }
                }
            } else if let Some(url) = opt.json_source {
                println!(
                    "{}",
                    indexer::read_source(&url).unwrap_or_else(|| "not found".to_string())
                )
            } else if let Some(url) = opt.import_url {
                indexer::index_url(
                    url,
                    indexer::UrlMeta::default(),
                    Some(&index),
                    indexer::NoAuthBlockingGetter {},
                );
            } else if let Some(facet) = opt.facet {
                let field = opt.facet_field.unwrap_or_else(|| "tags".to_string());
                facets(index, &field, &facet);
            }
        }
        Err(_) => println!("count not access index"),
    }

    Ok(())
}
