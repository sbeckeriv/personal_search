use anyhow::Error;

use serde_derive::{Deserialize, Serialize};

use serde_json::Value;
use url::form_urlencoded::byte_serialize;

use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, Uri};
use yew_router::{switch::Permissive, Switch};

pub struct App {
    link: ComponentLink<Self>,
    search: String,
    port: String,
    fetching: bool,
    network_task: Option<yew::services::fetch::FetchTask>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SystemSettings {
    pub port: String,
    pub ignore_domains: Vec<String>,
    pub ignore_strings: Vec<String>,
}

pub struct Settings {
    link: ComponentLink<Self>,
    settings: Option<SystemSettings>,
    port: String,
    new_ignore_string: String,
    new_ignore_domains: String,
    fetching: bool,
    network_task: Option<yew::services::fetch::FetchTask>,
}
//impl FetchJson for Settings {} figure this out with link
impl Settings {
    fn post_json(
        &mut self,
        binary: bool,
        url: String,
        body: &SystemSettings,
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
        let request = Request::post(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .body(Json(body))
            .unwrap();
        if binary {
            FetchService::fetch_binary(request, callback).unwrap()
        } else {
            FetchService::fetch(request, callback).unwrap()
        }
    }

    fn update_settings(&mut self, port: Option<String>) {
        self.fetching = true;
        let settings = self.settings.clone();
        let settings = settings.unwrap();

        self.network_task = Some(self.post_json(
            false,
            format!(
                "http://localhost:{}/settings",
                port.unwrap_or_else(|| self.port.clone()),
            ),
            &settings,
            "settings".to_string(),
        ));
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
        let request = Request::get(url)
            .header("Accept", "application/json")
            .body(Nothing)
            .unwrap();
        if binary {
            FetchService::fetch_binary(request, callback).unwrap()
        } else {
            FetchService::fetch(request, callback).unwrap()
        }
    }
    fn fetch_settings(&mut self, port: Option<String>) {
        self.fetching = true;
        self.network_task = Some(self.fetch_json(
            false,
            format!(
                "http://localhost:{}/settings",
                port.unwrap_or_else(|| self.port.clone())
            ),
            "settings".to_string(),
        ));
    }
    fn chip_it(&self, chip: &str) -> Html {
        let domain = chip.clone();
        let domain = domain.to_string();
        let id = format!("chip-{}", domain);
        ConsoleService::log(&format!("{:?}", domain));
        html! {
          <div class="chip" id=id>
            { format!("{}", domain.clone()) }
            <i class="close material-icons" onclick=self.link.callback(move |e: MouseEvent| Msg::RemoveIgnoreDomains(domain.clone()))>{"close"}</i>
          </div>
        }
    }
    fn loaded(&self) -> Html {
        if let Some(settings) = self.settings.as_ref() {
            ConsoleService::log(&format!("{:?}", settings));
            html! {<>
              <div class="row">
                  <div class="cliplist">
                    { settings.ignore_domains.iter().map(|d| self.chip_it(d)).collect::<Html>() }
                  </div>
                <div class="input-field col s12">
                  <input id="ignore_domains" type="text" value=self.new_ignore_domains.clone() oninput=self.link.callback(|e: InputData| Msg::UpdateIgnoreDomains(e.value))/>
                  <label class="active" for="ignore_domains">{ "Ignore domains (space adds it to the list)" }</label>
                </div>
              </div>
              <div class="row">
                <div class="input-field col s12">
                  <input id="ignore_strings" type="text" value={settings.ignore_strings.join(", ")} oninput=self.link.callback(|e: InputData| Msg::IgnoreStrings(e.value))/>
                  <label class="active" for="ignore_strings">{ "Ignore strings (csv list)" }</label>
                </div>
              </div>
            </>}
        } else {
            html! {<>
              <div class="row">
              { "The settings from the server have not loaded yet. If you changed the default port please manually restart the server and reload the page" }
              </div>
            </>}
        }
    }
    fn settings_modal(&self) -> Html {
        html! {
            <>
        <div id="setting_modal" class="modal">
          <div class="modal-content">
              <div class="row">
                <form class="col s12">
                  { self.loaded()}
                  <div class="row">
                    <div class="input-field col s6">
                      <input id="port" type="text" value={self.settings.as_ref().and_then(|s| Some(s.port.clone())).unwrap_or_else(|| self.port.clone())} oninput=self.link.callback(|e: InputData| Msg::UpdatePort(e.value))/>
                      <label class="active" for="port">{ "Server Port (manual restarts required)" }</label>
                    </div>
                  </div>
                </form>
              </div>
          </div>
        </div>
        </>
              }
    }
}

impl Component for Settings {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut s = Settings {
            link,
            settings: None,
            new_ignore_domains: String::new(),
            new_ignore_string: String::new(),
            port: "7172".to_string(),
            fetching: false,
            network_task: None,
        };
        s.fetch_settings(Some(s.port.clone()));
        s
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateIgnoreDomains(string) => {
                if string.ends_with(" ") {
                    if let Some(mut settings) = self.settings.as_mut() {
                        settings.ignore_domains.push(string.trim().to_string());
                        self.update_settings(None);
                    }
                    self.new_ignore_domains = String::new();
                } else {
                    self.new_ignore_domains = string;
                }
            }

            Msg::RemoveIgnoreDomains(string) => {
                if let Some(mut settings) = self.settings.as_mut() {
                    settings.ignore_domains.retain(|x| x != &string);
                    self.update_settings(None);
                }
            }

            Msg::UpdatePort(string) => {
                self.port = string;
                self.fetch_settings(Some(self.port.clone()));
            }
            Msg::FetchReady(response) => match response.0.as_str() {
                "settings" => {
                    self.fetching = false;
                    self.network_task = None;
                    if let Ok(results) = response.1 {
                        let results: Option<SystemSettings> = serde_json::from_value(results).ok();
                        ConsoleService::log(&format!("{:?}", results));
                        self.settings = results;
                    }
                }
                _ => {}
            },
            _ => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
        <>
            { self.settings_modal() }
        </>
        }
    }
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
    added_at: String,
    last_accessed_at: String,
    keywords: Vec<String>,
    tags: Vec<String>,
    bookmarked: i64,
    pinned: i64,
    duplicate: i64,
    accessed_count: i64,
}

pub struct SearchResults {
    search_json: Option<SearchArray>,
    link: ComponentLink<Self>,
    search: String,
    port: String,
    queued_search: Option<String>,
    fetching: bool,
    props: SearchProps,
    network_task: Option<yew::services::fetch::FetchTask>,
    pin_task: Option<yew::services::fetch::FetchTask>,
}
#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SearchProps {
    search_input: String,
}
impl Component for SearchResults {
    type Message = Msg;
    type Properties = SearchProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _empty: Vec<serde_json::Result<Request<Vec<u8>>>> = vec![];

        SearchResults {
            link,
            search_json: None,
            search: "".to_string(),
            // write /read from local stoage
            // https://dev.to/davidedelpapa/yew-tutorial-04-and-services-for-all-1non
            port: "7172".to_string(),
            queued_search: None,
            fetching: false,
            network_task: None,
            pin_task: None,
            props,
        }
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        ConsoleService::log(&format!("p{:?}", props));
        if self.props != props {
            self.update(Msg::Search(props.search_input.clone()));
            self.props = props;
            true
        } else {
            false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Pin(string) => self.remote_set_attribute(&string, &"pinned", 1),
            Msg::Unpin(string) => self.remote_set_attribute(&string, &"pinned", 0),
            Msg::Hide(string) => self.remote_set_attribute(&string, &"hide", 1),
            Msg::HideDomain(string) => self.remote_set_attribute(&string, &"hide_domain", 1),
            Msg::UpdatePort(string) => {
                self.port = string;
            }
            Msg::Search(search_string) => {
                self.search = search_string;
                // remove dup?
                if !self.search.trim().is_empty() {
                    if self.fetching {
                        //wonky debounce.
                        self.queued_search = Some(self.search.clone());
                    } else {
                        self.fetch_search(&self.search.clone())
                    }
                } else {
                    self.search_json = None;
                    self.fetching = false;
                    self.queued_search = None;
                    self.network_task = None;
                }
            }
            Msg::FetchReady(response) => {
                if let Some(next) = &self.queued_search {
                    self.fetching = false;
                    self.network_task = None;
                    self.fetch_search(&next.clone())
                } else {
                    match response.0.as_str() {
                        "search_items" => {
                            self.fetching = false;
                            self.network_task = None;
                            let results = response.1.ok();
                            ConsoleService::log(&format!("{:?}", results));
                            // remove dup
                            self.for_value(results);
                        }
                        "set_pin" => {
                            self.pin_task = None;
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

    fn view(&self) -> Html {
        html! {
        <>
            { self.search_results() }
        </>
        }
    }
}

impl SearchResults {
    fn for_value(&mut self, results: Option<Value>) {
        match results {
            Some(results) => {
                self.search_json = serde_json::from_value(results).ok();
            }
            None => {}
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

    fn remote_set_attribute(&mut self, url: &str, field: &str, value: i64) {
        let urlencoded: String = byte_serialize(url.as_bytes()).collect();
        // cause "debounce" the js kills the request the server still processes them
        self.pin_task = Some(self.fetch_json(
            false,
            format!(
                "http://localhost:{}/attrbiute?url={}&field={}&pinned={}",
                self.port, urlencoded, field, value
            ),
            format!("set_{}", field),
        ));
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
        let request = Request::get(url)
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
        if false {
            //self.fetching {
            html! {
              <div class="progress">
                  <div class="indeterminate"></div>
              </div>
            }
        } else {
            html! {<></>}
        }
    }

    fn search_item_html(&self, obj: &SearchJson) -> Html {
        html! {
          <li class="collection-item avatar">
            <span class="title"><a href=obj.url.clone() target="_blank">{&obj.title}{" "}{&obj.url}</a></span>
            <p> {&obj.description} <br/>
            {&obj.summary}
            <br/>
            { obj.keywords.iter().map(|keyword| self.chip(&keyword)).collect::<Vec<Html>>()}
            </p>

            { self.pinned(&obj.pinned, obj.url.clone()) }
            { self.bookmarked(&obj.bookmarked) }
          </li>
        }
    }

    fn pinned(&self, marked: &i64, url: String) -> Html {
        let url_pin = url.clone();
        if marked == &1 {
            html! {
            <a href="#!" class="secondary-content tooltipped search-pinned"
                data-position="bottom"
                data-url=url
                data-tooltip="Pinned"
                onclick=self.link.callback(move |e| Msg::Unpin(url_pin.clone()))
                >
                <i class="material-icons">{"star"}</i>
            </a>
            }
        } else {
            html! {
                <a href="#!" class="secondary-content tooltipped search-pinned"
                    data-position="bottom"
                    data-tooltip="Pinned"
                    data-url=url
                    onclick=self.link.callback(move |e| Msg::Pin(url_pin.clone()))
                   >
                    <i class="material-icons">{"star_border"}</i>
                </a>
            }
        }
    }

    fn bookmarked(&self, marked: &i64) -> Html {
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
        let string = if string.starts_with('/') {
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
        if self.fetching {
            self.loading_html()
        } else if let Some(json) = &self.search_json {
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

    fn set_pin(&mut self, _url: &str, _pinned: i64) {
        // find and set value.
        if let Some(_json) = &mut self.search_json {
            //for &mut result in json.results {
            //   if result.url == url {
            //      result.pinned = pinned;
            //   }
            // }
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
        if let Some(_json) = &self.facet_json {
            html! {
            <div class="col s1">
              <ul class="collection blue-grey with-header">
                <li class="collection-header"><h6>{header}</h6></li>
                    //{{json.iter().map(|i| self.facet_item(&i)).collect::<Html>()}}
              </ul>
            </div>
              }
        } else {
            html! {
            <div class="col s1">
              <ul class="collection blue-grey with-header">
                <li class="collection-header"><h6>{header}</h6></li>
              </ul>
            </div>
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
        let request = Request::get(url)
            .header("Accept", "application/json")
            .body(Nothing)
            .unwrap();
        if binary {
            FetchService::fetch_binary(request, callback).unwrap()
        } else {
            FetchService::fetch(request, callback).unwrap()
        }
    }

    fn setting_modal(&self) -> Html {
        html! {
        <Settings/>
          }
    }
    fn content(&self) -> Html {
        let _tags: Vec<&str> = vec![];

        html! {
        <>
        <div class="row results">
            <div class="col s11">
                <SearchResults search_input=self.search.clone()/>
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
                    <div class="input-field">
                        <input id="search" type="search" autocomplete="off" required=true value={self.search.clone()} oninput=self.link.callback(|e: InputData| Msg::Search(e.value))/>
                        <label class="label-icon" for="search"><i class="material-icons">{"search"}</i></label>
                    </div>
                     <a class="btn-floating btn-large halfway-fab waves-effect waves-light teal modal-trigger" href="#setting_modal">
                        <i class="material-icons">{"settings"}</i>
                      </a>
                </div>
        </nav>
        </header>

        </>
                    }
    }
}

pub enum Msg {
    Search(String),
    RemoveIgnoreDomains(String),
    UpdateIgnoreDomains(String),
    IgnoreStrings(String),
    Pin(String),
    Unpin(String),
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
        let _empty: Vec<serde_json::Result<Request<Vec<u8>>>> = vec![];
        App {
            link,
            search: "".to_string(),
            // write /read from local stoage
            // https://dev.to/davidedelpapa/yew-tutorial-04-and-services-for-all-1non
            port: "7172".to_string(),
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
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.network_task = None;
            }
            Msg::Ignore => {
                return false;
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
