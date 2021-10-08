mod utils;
mod webgl;
use wasm_bindgen::prelude::*;
use web_sys;
// use js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(raw_module = "../js-deps/three.module.js")]
extern { }


#[wasm_bindgen(raw_module = "../js-deps/pixi.mjs")]
extern {
    pub type Application;
    pub type IApplicationOptions;
    #[wasm_bindgen(constructor)]
    fn new(options: Option<IApplicationOptions>) -> Application;


    //#[wasm_bindgen(js_namespace = PIXI)]
    //fn isWebGLSupported() -> bool;
}


// Called by our JS entry point to run the example
//#[wasm_bindgen(start)]
//pub fn run() -> Result<(), JsValue> {
//    // Use `web_sys`'s global `window` function to get a handle on the global
//    // window object.
//    let window = web_sys::window().expect("no global `window` exists");
//    let document = window.document().expect("should have a document on window");
//    let body = document.body().expect("document should have a body");
//
//    // Manufacture the element we're gonna append
//    let val = document.create_element("p")?;
//    val.set_text_content(Some("Hello from Rust!"));
//
//    body.append_child(&val)?;
//
//    Ok(())
//}




#[wasm_bindgen]
pub fn create_pixi_app() -> Result<Application, JsValue> {
    let app = Application::new(None);

    //let yes = if isWebGLSupported() {
    //    "yes"
    //}
    //else {
    //    "no"
    //};
    let yes = "";

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    let ss = format!("Hello from Rust!").to_string();
    val.set_text_content(Some("boi"));
    body.append_child(&val)?;
    Ok(app)
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-pack-lib!");
}
