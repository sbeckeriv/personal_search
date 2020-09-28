use crate::pages::{About, Home};
use serde::ser;
use serde_json;
use yew::prelude::*;
use yew::services::fetch::{Request, Response};
use yew_router::{prelude::*, route::Route, switch::Permissive, Switch};

pub struct App {
    navbar_items: Vec<bool>,
    link: ComponentLink<Self>,
    search_term: String,
    search: String,
    search_items: Vec<serde_json::Result<Request<Vec<u8>>>>,
    term_items: Vec<serde_json::Result<Request<Vec<u8>>>>,
}

impl App {
    fn init(&self) {
        //load_terms()
    }
    fn load_search(&self) {}

    fn load_terms(&self) {}
    fn content(&self) -> Html {}
    fn header(&self) -> Html {
        html! {
        <>
        <header>

        <nav class="top-nav">
            <div class="container">
                <div class="nav-wrapper">
                    <a href="#" data-target="slide-out" class="sidenav-trigger brand-logo"><i class="material-icons">{"menu"}</i></a>
                    <form>
                    <div class="input-field">
                        <input id="search" type="search" required=true value={self.search.clone()} oninput=self.link.callback(|e: InputData| Msg::Search(e.value))/>
                        <label class="label-icon" for="search"><i class="material-icons">{"search"}</i></label>
                        <i class="material-icons">{"close"}</i>
                    </div>
                </form>
                </div>
            </div>
        </nav>

        <div class="container"/>
            <ul id="slide-out" class="sidenav sidenav-fixed">
                <li>{self.search.clone()}</li>
                <li><div class="divider"></div></li>
                <li><a class="subheader">{"Subheader"}</a></li>
                <li><a class="waves-effect" href="#!">{"Third Link With Waves"}</a></li>
            </ul>
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
            search_items: empty,
            term_items: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Search(search_string) => {
                self.search = search_string;
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
        </>
        }
    }
}
