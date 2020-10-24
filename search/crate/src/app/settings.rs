use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub enum Msg {
    FetchReady((String, Result<Value, Error>)),
    Ignore,
    RemoveIgnoreDomains(String),
    ToggleIndexer,
    UpdateIgnoreDomains(String),
    UpdatePort(String),
}
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct SystemSettings {
    pub port: String,
    pub ignore_domains: Vec<String>,
    pub ignore_strings: Vec<String>,
    pub indexer_enabled: bool,
}

pub struct Settings {
    link: ComponentLink<Self>,
    settings: Option<SystemSettings>,
    port: String,
    new_ignore_domains: String,
    fetching: bool,
    network_task: Option<FetchTask>,
}

impl Settings {
    fn post_json(
        &mut self,
        binary: bool,
        url: String,
        body: &SystemSettings,
        stored_data: String,
    ) -> FetchTask {
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
    fn fetch_json(&mut self, binary: bool, url: String, stored_data: String) -> FetchTask {
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
        let domain = chip.to_string();
        let domain = domain;
        let id = format!("chip-{}", domain);
        ConsoleService::log(&format!("{:?}", domain));
        html! {
          <div class="chip" key=id.clone() id=id>
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
                  { "The url will not be indexed if it matches any of this list. Only html content types are indexed. Space adds the value to the list." }
                  <input id="ignore_domains" type="text" value=self.new_ignore_domains.clone() oninput=self.link.callback(|e: InputData| Msg::UpdateIgnoreDomains(e.value))/>
                </div>
                <div class="switch">
                    { "Indexer: " }
                    <label>
                      { "Off" }
                      <input type="checkbox" checked=settings.indexer_enabled, onclick=self.link.callback(|_| Msg::ToggleIndexer ) />
                      <span class="lever"></span>
                      { "On" }
                    </label>
                </div>
              </div>
            </>}
        } else {
            html! {
              <div class="row">
              { "The settings from the server have not loaded yet. If you changed the default port please manually restart the server and reload the page" }
              </div>
            }
        }
    }
    fn settings_modal(&self) -> Html {
        html! {
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
        }
    }
}

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct SettingsProp {
    pub clicked_at: i64,
}
impl Component for Settings {
    type Message = Msg;
    type Properties = SettingsProp;

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut s = Settings {
            link,
            settings: None,
            new_ignore_domains: String::new(),
            port: "7172".to_string(),
            fetching: false,
            network_task: None,
        };
        s.fetch_settings(Some(s.port.clone()));
        s
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.fetch_settings(Some(self.port.clone()));
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateIgnoreDomains(string) => {
                if string.ends_with(' ') {
                    if let Some(settings) = self.settings.as_mut() {
                        settings.ignore_domains.push(string.trim().to_string());
                        self.update_settings(None);
                    }
                    self.new_ignore_domains = String::new();
                } else {
                    self.new_ignore_domains = string;
                }
            }

            Msg::RemoveIgnoreDomains(string) => {
                if let Some(settings) = self.settings.as_mut() {
                    settings.ignore_domains.retain(|x| x != &string);
                    self.update_settings(None);
                }
            }

            Msg::ToggleIndexer => {
                if let Some(settings) = self.settings.as_mut() {
                    settings.indexer_enabled = !settings.indexer_enabled;
                    self.update_settings(None);
                }
            }

            Msg::UpdatePort(string) => {
                // server needs to be pre configured
                self.port = string;
                self.fetch_settings(Some(self.port.clone()));
            }
            Msg::FetchReady(response) => {
                if let "settings" = response.0.as_str() {
                    self.fetching = false;
                    self.network_task = None;
                    if let Ok(results) = response.1 {
                        let results: Option<SystemSettings> = serde_json::from_value(results).ok();
                        ConsoleService::log(&format!("{:?}", results));
                        self.settings = results;
                    }
                }
            }
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
