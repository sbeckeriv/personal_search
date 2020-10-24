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

// https://github.com/yewstack/yew/issues/1281
#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHTMLProps {
    pub inner_html: String,
}

pub struct RawHTML {
    pub props: RawHTMLProps,
}

impl Component for RawHTML {
    type Message = ();
    type Properties = RawHTMLProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&self.props.inner_html[..]);

        let node = web_sys::Node::from(div);
        let vnode = yew::virtual_dom::VNode::VRef(node);
        vnode
    }
}
