use super::rawhtml::RawHTML;
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
pub struct ViewPage {
    link: ComponentLink<Self>,
    pub hash: String,
    content: String,
    fetching: bool,
    pub port: String,
    network_task: Option<yew::services::fetch::FetchTask>,
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct ViewPageProps {
    pub hash: String,
    pub port: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct ViewJson {
    content: String,
}

impl Component for ViewPage {
    type Message = Msg;
    type Properties = ViewPageProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut view = ViewPage {
            hash: props.hash,
            content: "".to_string(),
            fetching: false,
            network_task: None,
            port: props.port,
            link,
        };

        view.fetch_settings(Some(view.port.clone()));
        view
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ViewString(response) => {
                self.fetching = false;
                self.network_task = None;
                ConsoleService::log(&format!("{:?}", response));
                self.content = response;
            }
            _ => {}
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        if self.fetching {
            html! {
            <div class="progress">
                <div class="indeterminate"></div>
            </div>
                }
        } else if !self.content.is_empty() {
            html! {
                <div class="container">
                    <RawHTML inner_html=self.content.clone()/>
                </div>
            }
        } else {
            html! {}
        }
    }
}

impl ViewPage {
    fn fetch_json(
        &mut self,
        binary: bool,
        url: String,
        stored_data: String,
    ) -> yew::services::fetch::FetchTask {
        let callback = self
            .link
            .callback(move |response: Response<Result<String, Error>>| {
                let (meta, data) = response.into_parts();
                if meta.status.is_success() {
                    Msg::ViewString(data.unwrap_or_else(|_| String::new()))
                } else {
                    Msg::ViewString(String::new())
                }
            });
        let request = Request::get(url)
            .header("Accept", "application/json")
            .body(Nothing)
            .unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
    fn fetch_settings(&mut self, port: Option<String>) {
        self.fetching = true;
        self.network_task = Some(self.fetch_json(
            false,
            format!(
                "http://localhost:{}/view/{}",
                port.unwrap_or_else(|| self.port.clone()),
                self.hash
            ),
            "view".to_string(),
        ));
    }
}
