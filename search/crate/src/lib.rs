#![recursion_limit = "1024"]
extern crate wasm_bindgen;
extern crate web_sys;
extern crate yew;
extern crate yew_router;
use wasm_bindgen::prelude::*;


// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function to get better error messages if we ever panic.
extern crate console_error_panic_hook;
use console_error_panic_hook::set_once as set_panic_hook;

#[macro_use]
mod util;
mod app;

// Called by our JS entry point to run the example
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    // If the `console_error_panic_hook` feature is enabled this will set a panic hook, otherwise
    // it will do nothing.
    set_panic_hook();
    yew::start_app::<app::App>();

    Ok(())
}
