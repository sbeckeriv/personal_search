use personal_search::indexer;

use std::collections::HashMap;
use tantivy::collector::FacetCollector;
use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::AllQuery;
use tantivy::query::QueryParser;

use iced::{
    button, scrollable, text_input, Align, Application, Button, Checkbox, Clipboard, Column,
    Command, Container, Element, Font, HorizontalAlignment, Length, Row, Scrollable, Settings,
    Text, TextInput,
};
use serde::{Deserialize, Serialize};

pub fn main() -> iced::Result {
    Search::run(Settings::default())
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
            .flatten()
            .unwrap_or(0),
        pinned: m
            .get("pinned")
            .map(|t| t.get(0).map(|f| f.i64_value()).unwrap())
            .flatten()
            .unwrap_or(0),
        duplicate: m
            .get("duplicate")
            .map(|t| t.get(0).map(|f| f.i64_value()).unwrap())
            .flatten()
            .unwrap_or(0),
        accessed_count: m
            .get("accessed_count")
            .map(|t| t.get(0).map(|f| f.i64_value()).unwrap())
            .flatten()
            .unwrap_or(0),
    }
}

#[derive(Debug)]
enum Search {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    scroll: scrollable::State,
    input: text_input::State,
    input_value: String,
    results: Vec<SearchJson>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    ClickResult,
    CreateSearchResult,
    FilterChanged(Filter),
    SearchResultMessage(usize, SearchResultMessage),
}

impl Application for Search {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Search, Command<Message>) {
        (
            Search::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        let dirty = match self {
            Search::Loading => false,
            Search::Loaded(state) => state.dirty,
        };

        format!("Search{} - nIced", if dirty { "*" } else { "" })
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match self {
            Search::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = Search::Loaded(State {
                            input_value: state.input_value,
                            results: state.results,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = Search::Loaded(State::default());
                    }
                    _ => {}
                }

                Command::none()
            }
            Search::Loaded(state) => {
                match message {
                    Message::InputChanged(value) => {
                        // add debounce..
                        state.results = search(value.clone(), 10);
                        state.input_value = value;
                    }
                    _ => {}
                }
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        match self {
            Search::Loading => loading_message(),
            Search::Loaded(State {
                scroll,
                input,
                input_value,
                results,
                ..
            }) => {
                let title = Text::new("Personal Search")
                    .width(Length::Fill)
                    .size(50)
                    .color([0.5, 0.5, 0.5])
                    .horizontal_alignment(HorizontalAlignment::Center);

                let input = TextInput::new(input, "Search", input_value, Message::InputChanged)
                    .padding(15)
                    .size(30);

                let results: Element<_> = results
                    .iter_mut()
                    .enumerate()
                    .fold(Column::new().spacing(20), |column, (i, task)| {
                        column.push(
                            task.view()
                                .map(move |message| Message::SearchResultMessage(i, message)),
                        )
                    })
                    .into();

                let content = Column::new()
                    //.spacing(20)
                    .push(title)
                    .push(input)
                    .push(results);

                Scrollable::new(scroll)
                    .padding(40)
                    .push(Container::new(content).width(Length::Fill).center_x())
                    .into()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SearchResultMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}

impl SearchJson {
    fn view(&mut self) -> Element<SearchResultMessage> {
        let title = Text::new(&self.title)
            .width(Length::Fill)
            .size(16)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Left);

        let url = Text::new(&self.url)
            .width(Length::Fill)
            .size(16)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Left);
        let title_row = Row::new()
            .push(title)
            .push(url)
            .spacing(20)
            .align_items(Align::Center);

        let summary = Text::new(&self.summary)
            .width(Length::Fill)
            .size(16)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Left);
        let content_row = Row::new()
            .push(summary)
            .spacing(20)
            .align_items(Align::Center);
        Column::new().push(title_row).push(content_row).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

impl Filter {
    fn matches(&self, task: &SearchJson) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => false,
            Filter::Completed => false,
        }
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    Container::new(
        Text::new("Loading...")
            .horizontal_alignment(HorizontalAlignment::Center)
            .size(50),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_y()
    .into()
}

fn empty_message<'a>(message: &str) -> Element<'a, Message> {
    Container::new(
        Text::new(message)
            .width(Length::Fill)
            .size(25)
            .horizontal_alignment(HorizontalAlignment::Center)
            .color([0.7, 0.7, 0.7]),
    )
    .width(Length::Fill)
    .height(Length::Units(200))
    .center_y()
    .into()
}

fn icon(unicode: char) -> Text {
    Text::new(&unicode.to_string())
        .width(Length::Units(20))
        .horizontal_alignment(HorizontalAlignment::Center)
        .size(20)
}

fn edit_icon() -> Text {
    icon('\u{F303}')
}

fn delete_icon() -> Text {
    icon('\u{F1F8}')
}

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    filter: Filter,
    results: Vec<SearchJson>,
}

#[derive(Debug, Clone)]
enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
enum SaveError {
    FileError,
    WriteError,
    FormatError,
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    async fn load() -> Result<SavedState, LoadError> {
        serde_json::from_str("{}").map_err(|_| LoadError::FormatError)
    }
}
