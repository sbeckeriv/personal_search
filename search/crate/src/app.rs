use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use url::form_urlencoded::byte_serialize;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{History, Location, PopStateEvent};
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response, Uri};
use yew::utils::document;
pub mod rawhtml;
pub mod search;
pub mod settings;
pub mod view;
use search::SearchResults;
use settings::Settings;
use view::ViewPage;

pub struct App {
    link: ComponentLink<Self>,
    search: String,
    show_hash: String,
    settings_click: i64,
    port: String,
    fetching: bool,
    network_task: Option<yew::services::fetch::FetchTask>,
}

impl App {
    fn setting_modal(&self) -> Html {
        html! {
            <Settings clicked_at=self.settings_click />
        }
    }
    fn content(&self) -> Html {
        html! {
        <div class="row results">
            <div class="col s11">
                <SearchResults search_input=self.search.clone()/>
            </div>
        </div>
        }
    }

    fn header(&self) -> Html {
        html! {
        <header>
            <nav class="top-nav grey darken-3">
                    <div class="nav-wrapper">
                        <a href="#" data-target="slide-out" class="sidenav-trigger brand-logo"><i class="material-icons">{"menu"}</i></a>
                        <div class="input-field">
                            <input id="search" type="search" autocomplete="off" required=true value={self.search.clone()} oninput=self.link.callback(|e: InputData| Msg::Search(e.value))/>
                            <label class="label-icon" for="search"><i class="material-icons">{"search"}</i></label>
                        </div>
                         <a class="btn-floating btn-large halfway-fab waves-effect waves-light grey modal-trigger" href="#setting_modal" onclick=self.link.callback(|_| Msg::ClickSettings)>
                            <i class="material-icons">{"settings"}</i>
                          </a>
                    </div>
            </nav>
        </header>
        }
    }
}

pub enum Msg {
    Search(String),
    Pin(String),
    Unpin(String),
    Hide(String),
    Tag((String, String)),
    Untag((String, String)),
    HideDomain(String),

    //settings
    RemoveIgnoreDomains(String),
    UpdateIgnoreDomains(String),
    IgnoreStrings(String),
    UpdatePort(String),
    ToggleIndexer,

    ClickSettings,
    FetchReady((String, Result<Value, Error>)),
    ViewString(String),
    Ignore,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _empty: Vec<serde_json::Result<Request<Vec<u8>>>> = vec![];
        let mut param_search = "".to_string();
        let mut show_hash = "".to_string();
        if let Some(location) = document().location() {
            if let Ok(params) = location.search() {
                if params.starts_with("?q=") {
                    ConsoleService::log(&format!("{:?}", params));
                    param_search = params.replace("?q=", "");
                } else if params.starts_with("?view=") {
                    show_hash = params.replace("?view=", "");
                }
            }
        }
        App {
            link,
            search: param_search,
            show_hash: show_hash,
            settings_click: 0,
            // write /read from local stoage
            // https://dev.to/davidedelpapa/yew-tutorial-04-and-services-for-all-1non
            port: "7172".to_string(),
            fetching: false,
            network_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickSettings => {
                self.settings_click += 1;
            }
            Msg::Search(search_string) => {
                self.search = search_string;
            }
            Msg::FetchReady(_response) => {
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
        if self.show_hash.is_empty() {
            html! {
            <>
                { self.header() }
                <main>
                    { self.content() }
                    { self.setting_modal() }
                </main>
            </>
            }
        } else {
            html! { <ViewPage hash=self.show_hash.clone() port=self.port.clone()/> }
        }
    }
}
