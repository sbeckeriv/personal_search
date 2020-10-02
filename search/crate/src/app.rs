use crate::pages::{About, Home};
use anyhow::Error;
use serde::ser;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use serde_json::Value;
use url::form_urlencoded::{byte_serialize, parse};
use yew::callback::Callback;
use yew::format::{Format, Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, Uri};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

pub struct App {
    search_results: SearchResults,
    facet_results: FacetResults,
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
    search_term: String,
    search: String,
    port: String,
    queued_search: Option<String>,
    fetching: bool,
    network_task: Option<yew::services::fetch::FetchTask>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SearchArray {
    results: Vec<SearchJson>,
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
#[derive(Default)]
pub struct SearchResults {
    search_json: Option<SearchArray>,
}

impl SearchResults {
    fn for_value(results: Option<Value>) -> Self {
        match results {
            Some(results) => SearchResults {
                search_json: serde_json::from_value(results).ok(),
            },
            None => SearchResults::default(),
        }
    }
    fn facet_item(&self, name: &str) -> Html {
        html! {
            <li class="collection-item hoverable"><div><a href="#!" class="secondary-content">{name}</a></div></li>
        }
    }
    fn search_item_html(&self, obj: &SearchJson) -> Html {
        let title = obj
            .title
            .get(0)
            .and_then(|s| Some(s.as_str()))
            .unwrap_or("");

        let url = obj.url.get(0).and_then(|s| Some(s.as_str())).unwrap_or("");

        let description = obj
            .description
            .get(0)
            .and_then(|s| Some(s.as_str()))
            .unwrap_or("");

        let summary = obj
            .summary
            .get(0)
            .and_then(|s| Some(s.as_str()))
            .unwrap_or("");

        let pinned = obj.pinned.get(0).unwrap_or(&0);
        let bookmarked = obj.bookmarked.get(0).unwrap_or(&0);

        html! {
          <li class="collection-item avatar">
            <span class="title"><a href=url target="_blank">{title}{" "}{url}</a></span>
            <p> {description} <br/>
            {summary}
            <br/>
            { obj.keywords.as_ref().unwrap_or(&vec![]).iter().map(|keyword| self.chip(&keyword)).collect::<Vec<Html>>()}
            </p>

            { self.pinned(pinned)}
            { self.bookmarked(bookmarked)}
          </li>
        }
    }

    fn pinned(&self, marked: &i8) -> Html {
        if marked == &1 {
            html! {
            <a href="#!" class="secondary-content tooltipped search-pinned" data-position="bottom" data-tooltip="Pinned">
                <i class="material-icons">{"star"}</i>
            </a>
            }
        } else {
            html! {
                <a href="#!" class="secondary-content tooltipped search-pinned"  data-position="bottom" data-tooltip="Pinned">
                    <i class="material-icons">{"star_border"}</i>
                </a>
            }
        }
    }
    fn bookmarked(&self, marked: &i8) -> Html {
        if marked == &1 {
            html! {
            <a href="#!" class="secondary-content tooltipped search-bookmarked" data-position="bottom" data-tooltip="Bookmark">
                <i class="material-icons">{"bookmark"}</i>
            </a>
            }
        } else {
            html! {
                <a href="#!" class="secondary-content tooltipped search-bookmarked"  data-position="bottom" data-tooltip="Bookmark">
                    <i class="material-icons">{"bookmark_border"}</i>
                </a>
            }
        }
    }

    fn chip(&self, string: &str) -> Html {
        let string = string.trim();
        let string = if string.starts_with("/") {
            let mut chars = string.chars();
            chars.next();
            chars.as_str()
        } else {
            string
        };
        if string.is_empty() {
            html! {<></>}
        } else {
            html! {
                <div class="chip">
                    {string}
                </div>
            }
        }
    }

    fn search_results(&self) -> Html {
        if let Some(json) = &self.search_json {
            html! {
            <ul class="collection">
                { json.results.iter().map(|i|{ self.search_item_html(&i) }).collect::<Html>() }
            </ul>
            }
        } else {
            html! {
                <></>
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FacetArray {
    results: Vec<FacetJson>,
}
#[derive(Serialize, Deserialize, Debug)]
struct FacetJson {
    name: Vec<String>,
    count: Option<Vec<isize>>,
}
#[derive(Default)]
pub struct FacetResults {
    facet_json: Option<FacetArray>,
    header: String,
}

impl FacetResults {
    fn for_value(results: Option<Value>, header: &str) -> Self {
        match results {
            Some(results) => FacetResults {
                facet_json: serde_json::from_value(results).ok(),
                header: header.to_string(),
            },
            None => FacetResults::default(),
        }
    }

    fn facet_item(&self, name: &str) -> Html {
        html! {
            <li class="collection-item hoverable"><div><a href="#!" class="secondary-content">{name}</a></div></li>
        }
    }

    fn facets(&self, header: &str) -> Html {
        if let Some(json) = &self.facet_json {
            html! {
            <div class="col s1">
              <ul class="collection blue-grey with-header">
                <li class="collection-header"><h6>{header}</h6></li>
                    //{{list.iter().map(|i| self.facet_item(&i)).collect::<Html>()}}
              </ul>
            </div>
              }
        } else {
            html! {
                <></>
            }
        }
    }
}

impl App {
    fn init(&self) {
        //load_terms()
    }

    fn fetch_json(
        &mut self,
        binary: bool,
        url: String,
        stored_data: String,
    ) -> yew::services::fetch::FetchTask {
        let callback = self
            .link
            .callback(move |response: Response<Json<Result<Value, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchReady((stored_data.clone(), data))
                } else {
                    Msg::Ignore // FIXME: Handle this error accordingly.
                }
            });
        let mut request = Request::get(url)
            .header("Accept", "application/json")
            .body(Nothing)
            .unwrap();
        if binary {
            FetchService::fetch_binary(request, callback).unwrap()
        } else {
            FetchService::fetch(request, callback).unwrap()
        }
    }

    fn loading_html(&self) -> Html {
        if self.fetching {
            html! {
              <div class="progress">
                  <div class="indeterminate"></div>
              </div>
            }
        } else {
            html! {<></>}
        }
    }
    fn setting_modal(&self) -> Html {
        html! {
            <>
        <div id="setting_modal" class="modal">
          <div class="modal-content">
              <div class="row">
                <form class="col s12">
                  <div class="row">
                    <div class="input-field col s6">
                      <input id="port" type="text" value={self.port.clone()} oninput=self.link.callback(|e: InputData| Msg::UpdatePort(e.value))/>
                      <label class="active" for="port">{ "Server Port" }</label>
                    </div>
                  </div>
                </form>
              </div>
          </div>
        </div>
        </>
              }
    }
    fn content(&self) -> Html {
        let mut tags: Vec<&str> = vec![];

        html! {
        <>
        <div class="row">
        {self.facet_results.facets("Keywords")}
        <div class="col s11">
        {self.loading_html()}
        {self.search_results.search_results()}

        </div>
        </div>
        </>
        }
    }
    fn header(&self) -> Html {
        html! {
        <>
        <header>
        <nav class="top-nav">
                <div class="nav-wrapper">
                    <a href="#" data-target="slide-out" class="sidenav-trigger brand-logo"><i class="material-icons">{"menu"}</i></a>
                    <form>
                    <div class="input-field">
                        <input id="search" type="search" autocomplete="off" required=true value={self.search.clone()} oninput=self.link.callback(|e: InputData| Msg::Search(e.value))/>
                        <label class="label-icon" for="search"><i class="material-icons">{"search"}</i></label>
                    </div>
                     <a class="btn-floating btn-large halfway-fab waves-effect waves-light teal modal-trigger" href="#setting_modal">
                        <i class="material-icons">{"settings"}</i>
                      </a>
                </form>
                </div>
        </nav>
        </header>

        </>
                    }
    }

    fn fetch_search(&mut self, string: &str) {
        self.fetching = true;
        let urlencoded: String = byte_serialize(string.as_bytes()).collect();
        // cause "debounce" the js kills the request the server still processes them
        self.network_task = Some(self.fetch_json(
            false,
            format!("http://localhost:{}/search?q={}", self.port, urlencoded),
            "search_items".to_string(),
        ));
    }
}

#[derive(Switch, Debug, Clone)]
pub enum AppRouter {
    #[to = "/!"]
    RootPath,
    #[to = "/about!"]
    AboutPath,
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
}

pub enum Msg {
    Search(String),
    UpdatePort(String),
    SearchTerms(String),
    Hide(String),
    HideDomain(String),
    FetchReady((String, Result<Value, Error>)),
    Ignore,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let empty: Vec<serde_json::Result<Request<Vec<u8>>>> = vec![];
        App {
            link,
            navbar_items: vec![true, false],
            search_term: "".to_string(),
            search: "".to_string(),
            // write /read from local stoage
            // https://dev.to/davidedelpapa/yew-tutorial-04-and-services-for-all-1non
            port: "7172".to_string(),
            search_results: SearchResults::default(),
            facet_results: FacetResults::default(),
            queued_search: None,
            fetching: false,
            network_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdatePort(string) => {
                self.port = string;
            }
            Msg::Search(search_string) => {
                self.search = search_string;
                // remove dup?
                self.search_results = SearchResults::default();
                if self.search.trim().len() > 0 {
                    if self.fetching {
                        //wonky debounce.
                        self.queued_search = Some(self.search.clone());
                    } else {
                        self.fetch_search(&self.search.clone())
                    }
                } else {
                    self.fetching = false;
                    self.queued_search = None;
                    self.network_task = None;
                }
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.network_task = None;
                if let Some(next) = &self.queued_search {
                    self.fetch_search(&next.clone())
                } else {
                    match response.0.as_str() {
                        "search_items" => {
                            let results = response.1.map(|data| data).ok();
                            // remove dup
                            self.search_results = SearchResults::for_value(results);
                        }
                        _ => {}
                    }
                }

                self.queued_search = None;
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <>
            { self.header() }
            { self.content() }
            { self.setting_modal() }
        </>
        }
    }
}
