#[cfg(feature = "ml")]
use rust_bert::pipelines::summarization::{SummarizationConfig, SummarizationModel};

use std::io::Read;
use std::io::Write;

use std::path::Path;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;

fn hash_directory(
) -> Result<tantivy::directory::MmapDirectory, tantivy::directory::error::OpenDirectoryError> {
    super::create_directory(&super::BASE_INDEX_DIR);
    let index_path = Path::new(super::BASE_INDEX_DIR.as_str());

    tantivy::directory::MmapDirectory::open(index_path.join("hashes"))
}

pub fn hash_index() -> std::result::Result<tantivy::Index, tantivy::TantivyError> {
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
            Err(tantivy::TantivyError::SystemError(
                "could not open hash index directory".to_string(),
            ))
        }
    }
}

pub fn add_hash(domain: &str, hash: u64) {
    let index = hash_index().expect("hash index");
    let searcher = super::searcher(&index);
    let index_writer_read = super::HASHINDEXWRITER.clone();
    let query_parser = QueryParser::for_index(
        &index,
        vec![index.schema().get_field("domain").expect("domain field")],
    );
    let domain_hash = super::md5_hash(&domain);
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
        index_writer_read
            .read()
            .unwrap()
            .delete_term(frankenstein_isbn);
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

    index_writer_read.read().unwrap().add_document(doc);

    let mut index_writer_wlock = super::HASHINDEXWRITER.write().unwrap();
    index_writer_wlock.commit().unwrap();
}
