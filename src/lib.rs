mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(raw_module = "./pixi.mjs")]
// #[wasm_bindgen(raw_module = "pixi.mjs")]
extern {
    pub type Application;
    pub type IApplicationOptions;
    #[wasm_bindgen(constructor)]
    fn new(options: Option<IApplicationOptions>) -> Application;
}
#[wasm_bindgen]
pub fn create_pixi_app() -> Application {
    Application::new(None)
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-pack-lib!");
}
