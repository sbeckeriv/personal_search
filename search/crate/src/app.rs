use crate::pages::{About, Home};
use anyhow::Error;
use serde::ser;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;
use yew::callback::Callback;
use yew::format::{Format, Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
    search_term: String,
    search: String,
    search_items: Option<Value>,
    term_items: Option<Value>,
    fetching: bool,
    network_task: Option<yew::services::fetch::FetchTask>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub country_name: String,
    pub country_code: String,
    pub city: String,
    pub ip: String,
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
    fn load_terms(&self) {}
    fn facets(&self) -> Html {
        html! {
        <div class="col s1">
          <ul class="collection with-header">
            <li class="collection-header"><h6>{"Facets"}</h6></li>
            <li class="collection-item hoverable"><div>{self.search.clone()}<a href="#!" class="secondary-content"><i class="material-icons">{"send"}</i></a></div></li>
            <li class="collection-item hoverable"><div>{"Alvin"}<a href="#!" class="secondary-content"><i class="material-icons">{"send"}</i></a></div></li>
            <li class="collection-item hoverable"><div>{"Alvin"}<a href="#!" class="secondary-content"><i class="material-icons">{"send"}</i></a></div></li>
            <li class="collection-item hoverable"><div>{"Alvin"}<a href="#!" class="secondary-content"><i class="material-icons">{"send"}</i></a></div></li>
            <li class="collection-item hoverable"><div>{"Alvin"}<a href="#!" class="secondary-content"><i class="material-icons">{"send"}</i></a></div></li>
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
        html! {
            <div class="chip">
                {string}
            </div>
        }
    }
    fn search_results(&self) -> Html {
        if let Some(items) = &self.search_items {
            html! {
            <ul class="collection">
                { items["results"].as_array().unwrap().iter().map(|i|{ self.search_item_html(&i) }).collect::<Html>() }
            </ul>
            }
        } else if self.fetching {
            html! {
            <ul class="collection">
                <li>
                    <div class="spinner-layer spinner-green">
                        <div class="circle-clipper left">
                          <div class="circle"></div>
                        </div><div class="gap-patch">
                          <div class="circle"></div>
                        </div><div class="circle-clipper right">
                          <div class="circle"></div>
                        </div>
                    </div>
                </li>
            </ul>
            }
        } else {
            html! { <></>}
        }
    }
    fn content(&self) -> Html {
        html! {
        <>
        <div class="row">
          {self.facets()}

        <div class="col s11">
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
            search_items: None,
            term_items: None,
            fetching: false,
            network_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(search_string) => {
                self.search = search_string;
                self.network_task = Some(self.fetch_json(
                    false,
                    format!("http://localhost:7273/search?q={}", self.search),
                    "search_items".to_string(),
                ));
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.network_task = None;
                match response.0.as_str() {
                    "search_items" => {
                        let results = response.1.map(|data| data).ok();
                        self.search_items = results;
                    }
                    _ => {}
                }
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
