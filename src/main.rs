use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;
use tantivy::collector::TopDocs;
use tantivy::fastfield::FacetReader;
use tantivy::query::QueryParser;
mod indexer;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(long = "query", name = "query")]
    query: Option<String>,
    #[structopt(long = "import_url")]
    import_url: Option<String>,
    #[structopt(long = "facets")]
    facets: Option<String>,
    #[structopt(short = "s", long = "silent")]
    silent: bool,
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(long = "search_folder")]
    #[structopt(parse(from_os_str))]
    search_folder_path: Option<PathBuf>,
}
use tantivy::collector::FacetCollector;
use tantivy::doc;
use tantivy::query::AllQuery;
use tantivy::schema::{Facet, Schema, TEXT};
fn facets(index: tantivy::Index) {
    let index = indexer::search_index().expect("index");
    let reader = index.reader().expect("Reader");
    let searcher = reader.searcher();
    let tags = index.schema().get_field("tags").expect("tag");
    let mut facet_collector = FacetCollector::for_field(tags);
    facet_collector.add_facet("/Felidae");
    let facet_counts = searcher
        .search(&AllQuery, &facet_collector)
        .expect("search");
    // This lists all of the facet counts, right below "/Felidae".
    let facets: Vec<(&Facet, u64)> = facet_counts.get("/Felidae").collect();
    dbg!(&facets);
}
fn search(query: String, index: tantivy::Index) {
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
        println!(
            "{score}: {title} - {url}\n{summary}\n",
            score = score,
            title = title,
            url = url,
            summary = summary,
        );
        //let json = index.schema().to_json(&retrieved_doc);
        //println!("{}:\n{}", score, json);
    }
}

fn main() -> tantivy::Result<()> {
    let index = indexer::search_index();

    let opt = Opt::from_args();

    match index {
        Ok(index) => {
            if let Some(query) = opt.query {
                search(query, index);
            } else if let Some(url) = opt.import_url {
                indexer::index_url(url, indexer::UrlMeta::default(), Some(&index));
            } else if let Some(facet) = opt.facets {
                facets(index);
            }
        }
        Err(_) => println!("count not access index"),
    }

    Ok(())
}
