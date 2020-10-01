use crate::pages::{About, Home};
use anyhow::Error;
use serde::ser;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use url::form_urlencoded::{byte_serialize, parse};
use yew::callback::Callback;
use yew::format::{Format, Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, Uri};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
    search_term: String,
    search: String,
    port: String,
    search_items: Option<Value>,
    facet_items: Option<Value>,
    queued_search: Option<String>,
    fetching: bool,
    network_task: Option<yew::services::fetch::FetchTask>,
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
    fn facet_item(&self, name: &str) -> Html {
        html! {
            <li class="collection-item hoverable"><div><a href="#!" class="secondary-content">{name}</a></div></li>
        }
    }

    fn facets(&self, header: &str, list: &Vec<&str>) -> Html {
        html! {
        <div class="col s1">
          <ul class="collection blue-grey with-header">
            <li class="collection-header"><h6>{header}</h6></li>
                {{list.iter().map(|i| self.facet_item(&i)).collect::<Html>()}}
          </ul>
        </div>
          }
    }
    fn search_item_html(&self, item: &Value) -> Html {
        if let Some(obj) = item.as_object() {
            let title = obj
                .get("title")
                .and_then(|s| s.as_array())
                .and_then(|a| a.get(0))
                .and_then(|s| s.as_str())
                .unwrap_or("");

            let url = obj
                .get("url")
                .and_then(|s| s.as_array())
                .and_then(|a| a.get(0))
                .and_then(|s| s.as_str())
                .unwrap_or("");

            let description = obj
                .get("description")
                .and_then(|s| s.as_array())
                .and_then(|a| a.get(0))
                .and_then(|s| s.as_str())
                .unwrap_or("");
            let summary = obj
                .get("summary")
                .and_then(|s| s.as_array())
                .and_then(|a| a.get(0))
                .and_then(|s| s.as_str())
                .unwrap_or("");

            let keywords: Vec<&str> = obj
                .get("keywords")
                .and_then(|s| s.as_array())
                .and_then(|a| Some(a.iter().map(|x| x.as_str().unwrap_or("")).collect()))
                .unwrap_or(vec![]);

            html! {
              <li class="collection-item avatar">
                <span class="title"><a href=url target="_blank">{title}{" "}{url}</a></span>
                <p> {description} <br/>
                {summary}
                <br/>
                { keywords.iter().map(|keyword| self.chip(&keyword)).collect::<Vec<Html>>()}
                </p>
                <a href="#!" class="secondary-content"><i class="material-icons">{"grade"}</i></a>
              </li>
            }
        } else {
            html! {<></>}
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
    fn search_results(&self) -> Html {
        if let Some(items) = &self.search_items {
            html! {
            <ul class="collection">
                { items["results"].as_array().unwrap().iter().map(|i|{ self.search_item_html(&i) }).collect::<Html>() }
            </ul>
            }
        } else {
            html! { <></>}
        }
    }
    fn setting_modal(&self) -> Html {
        html! {
            <>
        <a class="waves-effect waves-light btn modal-trigger" href="#modal1">{"Modal"}</a>

        <div id="modal1" class="modal">
          <div class="modal-content">
          </div>
        </div>
        </>
              }
    }
    fn content(&self) -> Html {
        let mut tags: Vec<&str> = vec![];

        if let Some(item_list) = &self.facet_items {
            for i in item_list
                .get("tags")
                .and_then(|s| s.as_array())
                .and_then(|a| Some(a.iter().map(|x| x.as_str().unwrap_or("").clone()).collect()))
                .unwrap_or(vec![])
                .iter()
            {
                tags.push(i.clone());
            }
        }
        html! {
        <>
        <div class="row">
        {self.facets("Facets", &tags )}
        <div class="col s11">
        {self.loading_html()}
        {self.search_results()}
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
                        <i class="material-icons">{"close"}</i>
                    </div>
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
            port: "7172".to_string(),
            search_items: None,
            facet_items: None,
            queued_search: None,
            fetching: false,
            network_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(search_string) => {
                self.search = search_string;
                self.search_items = None;
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
                            self.search_items = results;
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
            {self.header()}
            {self.content()}
        </>
        }
    }
}
