use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::StreamExt;
use json::JsonValue;
use personal_search::indexer;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use structopt::StructOpt;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(long = "port", name = "port")]
    port: Option<String>,
    #[structopt(short = "s", long = "silent")]
    silent: bool,
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(long = "search_folder")]
    #[structopt(parse(from_os_str))]
    search_folder_path: Option<PathBuf>,
}

fn search(query: String) -> Vec<String> {
    let index = indexer::search_index().expect("could not open search index");
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
    top_docs
        .iter()
        .map(|doc| {
            let retrieved_doc = searcher.doc(doc.1).expect("doc");
            let mut m = HashMap::new();
            for f in retrieved_doc.field_values().iter() {
                m.entry(schema.get_field_name(f.field()))
                    .or_insert_with(Vec::new)
                    .push(f.value())
            }
            index.schema().to_json(&retrieved_doc)
        })
        .collect()
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    q: String,
}
/// This handler uses json extractor
async fn index(web::Query(info): web::Query<SearchRequest>) -> String {
    format!("[{}]", search(info.q).join(","))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    if !opt.silent {
        std::env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();
    }
    let port = opt.port.unwrap_or("7273".to_string());
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/search").route(web::get().to(index)))
            .service(web::resource("/").route(web::get().to(index)))
    })
    .bind(&format!("127.0.0.1:{}", port))?
    .run()
    .await
}
