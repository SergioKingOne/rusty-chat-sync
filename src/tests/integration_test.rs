use rusty_chat_sync::components::chat::Chat;
use wasm_bindgen_test::*;
use web_sys::window;
use yew::prelude::*;
use yew::Renderer;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_chat_component_renders() {
    Renderer::<Chat>::with_root(window().unwrap().document().unwrap().body().unwrap()).render();
    // TODO
}
