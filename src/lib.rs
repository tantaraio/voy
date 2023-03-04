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

#[derive(Serialize, Deserialize, Debug)]
pub struct Embedding {
    id: String,
    title: String,
    url: String,
    embeddings: Vec<f32>,
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
        _ => engine::Query::Embeddings(vec![]),
    };

    let result = engine::search(&index, &query, k).unwrap();

    Ok(to_value(&result)?)
}
