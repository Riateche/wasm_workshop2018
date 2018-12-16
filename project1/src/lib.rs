mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

use web_sys::window;

use rhai::Engine;

#[wasm_bindgen]
pub fn greet(code: &str) -> String {
    let mut engine = Engine::new();

    let result =  engine.eval::<String>(code);
    format!("{:?}", result)
}
