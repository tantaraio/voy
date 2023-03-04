mod document;
mod engine;

use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value, Error};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embedding {
    id: String,
    title: String,
    url: String,
    embdeddings: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    // pub documents: Vec<Document>,
    pub embeddings: Vec<Embedding>,
}

#[wasm_bindgen]
pub fn index(input: JsValue) -> String {
    console_error_panic_hook::set_once();

    let input: Input = from_value(input).unwrap();
    let index = engine::index(input);

    match index {
        Ok(tree) => serde_json::to_string(&tree).unwrap(),
        _ => "".to_owned(),
    }
}

#[wasm_bindgen]
pub fn search(index: &str, query: JsValue, k: usize) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let index: engine::Index = serde_json::from_str(index).unwrap();

    let query: Result<Vec<f32>, Error> = from_value(query);
    let query: engine::Query = match query {
        Ok(q) => engine::Query::Embeddings(q),
        _ => {
            console_log!("unable to cast query");
            engine::Query::Embeddings(vec![])
        }
    };

    let result = engine::search(&index, &query, k).unwrap();

    Ok(to_value(&result)?)
}
