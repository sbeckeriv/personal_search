#[cfg(feature = "static")]
use actix_web_static_files;
#[cfg(feature = "static")]
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::http::StatusCode;
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
    id: String,
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
fn doc_to_json(retrieved_doc: &tantivy::Document, schema: &tantivy::schema::Schema) -> SearchJson {
    let mut m = HashMap::new();
    for f in retrieved_doc.field_values().iter() {
        m.entry(schema.get_field_name(f.field()))
            .or_insert_with(Vec::new)
            .push(f.value())
    }

    let tags = retrieved_doc
        .get_all(schema.get_field("tags").expect("tags"))
        .iter()
        .map(|s| {
            if let tantivy::schema::Value::Facet(facet) = s {
                facet.to_path_string()
            } else {
                "".to_string()
            }
        })
        .collect::<Vec<_>>();

    SearchJson {
        id: m
            .get("id")
            .map(|t| t.get(0).map(|f| f.text().unwrap_or("")).unwrap())
            .unwrap_or("")
            .to_string(),
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
        //no longer real
        keywords: m
            .get("keywords")
            .map(|t| {
                t.iter()
                    .map(|ff| ff.text().unwrap_or("").to_string())
                    .collect()
            })
            .unwrap_or_default(),

        tags,
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
}

fn search(query: String, limit: usize) -> Vec<SearchJson> {
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
    let query = if query.contains("hidden:") {
        query
    } else {
        format!("(({}) AND {})", query, "hidden:0")
    };

    if let Ok(query) = query_parser.parse_query(&query) {
        let top_docs = searcher
            .search(&query, &TopDocs::with_limit(limit))
            .expect("serach");
        let schema = index.schema();

        top_docs
            .iter()
            .map(|doc| {
                let retrieved_doc = searcher.doc(doc.1).expect("doc");
                doc_to_json(&retrieved_doc, &schema)
            })
            .collect()
    } else {
        vec![]
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    q: String,
    limit: Option<usize>,
}

/// This handler uses json extractor
async fn search_request(
    web::Query(info): web::Query<SearchRequest>,
) -> web::Json<HashMap<String, Vec<SearchJson>>> {
    let mut m = HashMap::new();
    m.insert(
        "results".to_string(),
        search(info.q, info.limit.unwrap_or(20)),
    );
    web::Json(m)
}

#[derive(Debug, Deserialize)]
pub struct AttributeArrayRequest {
    url: String,
    field: String,
    value: String,
    action: String,
}
async fn attribute_array_request(
    web::Query(info): web::Query<AttributeArrayRequest>,
) -> web::Json<Option<SearchJson>> {
    let index = indexer::search_index().expect("could not open search index");
    let _searcher = indexer::searcher(&index);

    let mut meta = indexer::UrlMeta::default();
    match info.action.as_str() {
        "add" => {
            let tag = info.value.trim().to_string();
            let tag = if tag.starts_with('/') {
                tag
            } else {
                format!("/{}/{}", info.field, tag)
            };

            meta.tags_add = Some(vec![tag]);
        }
        "remove" => {
            let tag = info.value.trim().to_string();
            let tag = if tag.starts_with('/') {
                tag
            } else {
                format!("/{}/{}", info.field, tag)
            };
            meta.tags_remove = Some(vec![tag]);
        }
        _ => {}
    }

    if let Some(_doc_address) = indexer::find_url(&info.url, &index) {
        let id = indexer::md5_hash(&info.url);
        let old_doc = tantivy::Term::from_field_text(
            index.schema().get_field("id").expect("domain field"),
            &id,
        );

        let mut index_writer = index.writer(50_000_000).expect("writer");
        index_writer.delete_term(old_doc);

        let url_hash = indexer::md5_hash(&info.url);

        indexer::update_cached(&url_hash, &index, meta, &mut index_writer);
        index_writer.commit().expect("commit");
        index_writer.wait_merging_threads().expect("merge");
    } else {
        let url = info.url.clone();
        //tokio::spawn(lazy(move |_| {
        indexer::index_url(url, meta, Some(&index), indexer::NoAuthBlockingGetter {});
        //}));
    }

    let index = indexer::search_index().expect("could not open search index");
    if let Some(doc_address) = indexer::find_url(&info.url, &index) {
        let searcher = indexer::searcher(&index);
        let schema = index.schema();
        let retrieved_doc = searcher.doc(doc_address).expect("doc");
        web::Json(Some(doc_to_json(&retrieved_doc, &schema)))
    } else {
        web::Json(None)
    }
}

fn attribute_update(info: &AttributeRequest) -> web::Json<Option<SearchJson>> {
    let index = indexer::search_index().expect("could not open search index");
    let _searcher = indexer::searcher(&index);

    let mut meta = indexer::UrlMeta::default();
    match info.field.as_str() {
        "pinned" => {
            meta.pinned = Some(info.value.into());
        }
        "hide" => {
            meta.hidden = Some(info.value.into());
        }
        _ => {}
    }
    if let Some(_doc_address) = indexer::find_url(&info.url, &index) {
        let id = indexer::md5_hash(&info.url);
        let old_doc = tantivy::Term::from_field_text(
            index.schema().get_field("id").expect("domain field"),
            &id,
        );

        let mut index_writer = index.writer(50_000_000).expect("writer");
        index_writer.delete_term(old_doc);
        //    index_writer.commit().expect("commit");
        //     index_writer.wait_merging_threads().expect("merge");

        let url_hash = indexer::md5_hash(&info.url);

        indexer::update_cached(&url_hash, &index, meta, &mut index_writer);
        index_writer.commit().expect("commit");
        index_writer.wait_merging_threads().expect("merge");
    } else {
        let url = info.url.clone();
        //tokio::spawn(lazy(move |_| {
        println!("new");
        indexer::index_url(url, meta, Some(&index), indexer::NoAuthBlockingGetter {});
        //}));
    }

    let index = indexer::search_index().expect("could not open search index");
    if let Some(doc_address) = indexer::find_url(&info.url, &index) {
        let searcher = indexer::searcher(&index);
        let schema = index.schema();
        let retrieved_doc = searcher.doc(doc_address).expect("doc");
        web::Json(Some(doc_to_json(&retrieved_doc, &schema)))
    } else {
        web::Json(None)
    }
}

#[derive(Debug, Deserialize)]
pub struct AttributeRequest {
    url: String,
    field: String,
    value: i8,
}
async fn attribute_request(
    _request: HttpRequest,
    web::Query(info): web::Query<AttributeRequest>,
) -> web::Json<Option<SearchJson>> {
    if info.field.as_str() == "hide_domain" {
        let mut settings = indexer::read_settings();
        let parsed = url::Url::parse(&info.url).expect("url pase");
        if let Some(domain) = parsed.domain() {
            settings.ignore_domains.push(domain.to_string());
            indexer::write_settings(&settings);
        }
        web::Json(None)
    } else {
        attribute_update(&info)
    }
}

#[derive(Debug, Deserialize)]
pub struct FacetRequest {
    facet: String,
    facet_field: Option<String>,
}

async fn facet_request(web::Query(info): web::Query<FacetRequest>) -> web::Json<Vec<FacetCount>> {
    let field = info.facet_field.unwrap_or_else(|| "tags".to_string());
    web::Json(facets(info.facet, field))
}

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct UpdateSystemSettings {
    port: Option<String>,
    ignore_domains: Option<Vec<String>>,
    ignore_strings: Option<Vec<String>>,
    indexer_enabled: Option<bool>,
}
async fn update_settings(
    info: web::Json<UpdateSystemSettings>,
) -> web::Json<indexer::SystemSettings> {
    let mut settings = indexer::read_settings();
    if let Some(port) = &info.port {
        settings.port = port.clone();
    }

    if let Some(enabled) = &info.indexer_enabled {
        settings.indexer_enabled = *enabled;
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
    let path: PathBuf = name.parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[cfg(feature = "static")]
fn static_assets() -> actix_web_static_files::ResourceFiles {
    actix_web_static_files::ResourceFiles::new("/", generate())
}
#[cfg(not(feature = "static"))]
fn static_assets() -> actix_web::Resource {
    web::resource("/{filename:.*}").route(web::get().to(filesystem))
}

async fn view(web::Path(hash): web::Path<String>) -> Result<HttpResponse> {
    let hash = if hash.contains("://") {
        indexer::md5_hash(&hash)
    } else {
        hash
    };

    let mut body = format!("<div id='content'>url hash {} is not found</div>", hash);
    if let Some(json_string) = indexer::read_source(&hash) {
        let json: Result<serde_json::Value, _> = serde_json::from_str(&json_string);
        if let Ok(json) = json {
            if let Some(content) = json.get("content_raw") {
                if let serde_json::Value::Array(content) = content {
                    let content = indexer::view_body(content[0].as_str().unwrap_or(""));
                    body = format!("<div><a href='{}' target='_blank'>{}</a><br/><br/><div id='content'>{}</div></div>", json["url"][0].as_str().unwrap(), json["url"][0].as_str().unwrap(),content);
                }
            } else if let Some(content) = json.get("content") {
                if let serde_json::Value::Array(content) = content {
                    body = format!(
                        "<div><a href='{}' target='_blank'>{}</a><br/><div id='content'>{}</div>",
                        json["url"][0].as_str().unwrap(),
                        json["url"][0].as_str().unwrap(),
                        content[0]
                    );
                }
            } else {
            }
        }
    } else {
    }
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(body))
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
                    //.allowed_origin("http://localhost")
                    //.allowed_origin(&format!("http://localhost:{}", &port.clone()))
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
                web::resource("/view/{hash}")
                    .route(web::get().to(view))
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
                //yes i know it should be a post i dont care
                web::resource("/attributes_array").route(web::get().to(attribute_array_request)),
            )
            .service(
                web::resource("/facets")
                    .route(web::get().to(facet_request))
                    .route(web::head().to(HttpResponse::MethodNotAllowed)),
            )
            .service(static_assets())
    })
    .bind(&format!("127.0.0.1:{}", server_port))?
    .run()
    .await
}
