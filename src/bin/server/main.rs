use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use personal_search::indexer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;
use tantivy::collector::FacetCollector;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::AllQuery;
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

fn set_attribute(_url: String, _field: String, _value: i8) {
    //indexer::pin_url(&url, pin);
}

#[derive(Serialize)]
struct FacetCount {
    name: String,
    count: u64,
}

fn facets(query: String, field: String) -> Vec<FacetCount> {
    let query = if query.starts_with('/') {
        query
    } else {
        format!("/{}", query)
    };

    let index = indexer::search_index().expect("could not open search index");
    let searcher = indexer::searcher(&index);
    let tags = index
        .schema()
        .get_field(&field)
        .unwrap_or_else(|| panic!("{} not a field", field));
    let mut facet_collector = FacetCollector::for_field(tags);
    facet_collector.add_facet(&query);

    let facet_counts = searcher.search(&AllQuery, &facet_collector).expect("facet");

    facet_counts
        .get(&query)
        .map(|f| FacetCount {
            name: format!("{}", f.0),
            count: f.1,
        })
        .collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchJson {
    title: String,
    url: String,
    summary: String,
    description: String,
    keywords: Vec<String>,
    tags: Vec<String>,
    bookmarked: i64,
    pinned: i64,
    duplicate: i64,
    accessed_count: i64,
    added_at: String,
    last_accessed_at: String,
}
fn search(query: String) -> Vec<SearchJson> {
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

    if let Ok(query) = query_parser.parse_query(&query) {
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

                SearchJson {
                    title: m
                        .get("title")
                        .map(|t| t.get(0).map(|f| f.text().unwrap_or("")).unwrap())
                        .unwrap_or("")
                        .to_string(),

                    url: m
                        .get("url")
                        .map(|t| t.get(0).map(|f| f.text().unwrap_or("")).unwrap())
                        .unwrap_or("")
                        .to_string(),
                    summary: m
                        .get("summary")
                        .map(|t| t.get(0).map(|f| f.text().unwrap_or("")).unwrap())
                        .unwrap_or("")
                        .to_string(),
                    description: m
                        .get("description")
                        .map(|t| t.get(0).map(|f| f.text().unwrap_or("")).unwrap())
                        .unwrap_or("")
                        .to_string(),
                    added_at: m
                        .get("added_at")
                        .map(|t| t.get(0).map(|f| f.text().unwrap_or("")).unwrap())
                        .unwrap_or("")
                        .to_string(),
                    last_accessed_at: m
                        .get("last_accessed_at")
                        .map(|t| t.get(0).map(|f| f.text().unwrap_or("")).unwrap())
                        .unwrap_or("")
                        .to_string(),

                    keywords: m
                        .get("keywords")
                        .map(|t| {
                            t.iter()
                                .map(|ff| ff.text().unwrap_or("").to_string())
                                .collect()
                        })
                        .unwrap_or_default(),

                    tags: m
                        .get("keywords")
                        .map(|t| {
                            t.iter()
                                .map(|ff| ff.text().unwrap_or("").to_string())
                                .collect()
                        })
                        .unwrap_or_default(),
                    bookmarked: m
                        .get("bookmarked")
                        .map(|t| t.get(0).map(|f| f.i64_value()).unwrap())
                        .unwrap_or(0),
                    pinned: m
                        .get("pinned")
                        .map(|t| t.get(0).map(|f| f.i64_value()).unwrap())
                        .unwrap_or(0),
                    duplicate: m
                        .get("duplicate")
                        .map(|t| t.get(0).map(|f| f.i64_value()).unwrap())
                        .unwrap_or(0),
                    accessed_count: m
                        .get("accessed_count")
                        .map(|t| t.get(0).map(|f| f.i64_value()).unwrap())
                        .unwrap_or(0),
                }
            })
            .collect()
    } else {
        vec![]
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    q: String,
}

/// This handler uses json extractor
async fn search_request(
    web::Query(info): web::Query<SearchRequest>,
) -> web::Json<HashMap<String, Vec<SearchJson>>> {
    let mut m = HashMap::new();
    m.insert("results".to_string(), search(info.q));
    web::Json(m)
}

#[derive(Debug, Deserialize)]
pub struct AttributeRequest {
    url: String,
    field: String,
    value: i8,
}
async fn attribute_request(
    web::Query(info): web::Query<AttributeRequest>,
) -> web::Json<Vec<FacetCount>> {
    set_attribute(info.url, info.field, info.value);
    web::Json(vec![])
}

#[derive(Debug, Deserialize)]
pub struct FacetRequest {
    facet: String,
    facet_field: Option<String>,
}

async fn facet_request(web::Query(info): web::Query<FacetRequest>) -> web::Json<Vec<FacetCount>> {
    let field = info.facet_field.unwrap_or_else(|| "keywords".to_string());
    web::Json(facets(info.facet, field))
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct UpdateSystemSettings {
    port: Option<String>,
    ignore_domains: Option<Vec<String>>,
    ignore_strings: Option<Vec<String>>,
}
async fn update_settings(
    info: web::Json<UpdateSystemSettings>,
) -> web::Json<indexer::SystemSettings> {
    let mut settings = indexer::read_settings();
    if let Some(port) = &info.port {
        settings.port = port.clone();
    }
    if let Some(ignore_domains) = &info.ignore_domains {
        settings.ignore_domains = ignore_domains.clone();
    }

    if let Some(ignore_strings) = &info.ignore_strings {
        settings.ignore_strings = ignore_strings.clone();
    }
    indexer::write_settings(&settings);
    web::Json(settings)
}

async fn get_settings() -> web::Json<indexer::SystemSettings> {
    web::Json(indexer::read_settings())
}

async fn filesystem(req: HttpRequest) -> Result<NamedFile> {
    let name = req.match_info().query("filename");
    let name = name.replace("/", ""); // try not to leave the dir
    let name = format!("{}/{}", "search/dist", name);
    dbg!(&name);
    let path: PathBuf = name.parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    if !opt.silent {
        std::env::set_var("RUST_LOG", "actix_web=debug");
        env_logger::init();
    }
    let port = opt.port.unwrap_or_else(|| indexer::read_settings().port);
    let server_port = port.clone();
    HttpServer::new(move || {
        App::new()
            .wrap(
                // not sure i need this if severing from here
                Cors::new()
                    .allowed_origin("http://localhost")
                    .allowed_origin(&format!("http://localhost:{}", &port.clone()))
                    .max_age(3600)
                    .allowed_methods(vec!["GET", "POST", "PUT"])
                    .finish(),
            )
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(
                web::resource("/search")
                    .route(web::get().to(search_request))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
            .service(
                web::resource("/settings")
                    .route(web::get().to(get_settings))
                    .route(web::post().to(update_settings))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
            .service(
                //yes i know it should be a post i dont care
                web::resource("/attributes")
                    .route(web::get().to(attribute_request))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
            .service(
                web::resource("/facets")
                    .route(web::get().to(facet_request))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
            .service(web::resource("/{filename:.*}").route(web::get().to(filesystem)))
    })
    .bind(&format!("127.0.0.1:{}", server_port))?
    .run()
    .await
}
