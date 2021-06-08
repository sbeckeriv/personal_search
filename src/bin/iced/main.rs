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
