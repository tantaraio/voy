mod document;
mod engine;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

type NumberOfResult = usize;
type Query = Vec<f32>;
type SerializedIndex = String;

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct EmbeddedResource {
    pub id: String,
    pub title: String,
    pub url: String,
    pub embeddings: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct Resource {
    // TODO: Support different type of resources
    // pub documents: Vec<Document>,
    // pub audio: Vec<Audio>,
    // pub video: Vec<Video>,
    pub embeddings: Vec<EmbeddedResource>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Neighbor {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SearchResult {
    neighbors: Vec<Neighbor>,
}

#[wasm_bindgen]
pub fn index(resource: Resource) -> SerializedIndex {
    console_error_panic_hook::set_once();

    let index = engine::index(resource);

    match index {
        Ok(tree) => serde_json::to_string(&tree).unwrap(),
        _ => "".to_owned(),
    }
}

#[wasm_bindgen]
pub fn search(index: SerializedIndex, query: Query, k: NumberOfResult) -> SearchResult {
    console_error_panic_hook::set_once();

    let index: engine::Index = serde_json::from_str(&index).unwrap();
    let query: engine::Query = engine::Query::Embeddings(query);
    let neighbors = engine::search(&index, &query, k).unwrap();

    let neighbors: Vec<Neighbor> = neighbors
        .into_iter()
        .map(|res| Neighbor {
            id: res.id,
            title: res.title,
            url: res.url,
        })
        .collect();

    SearchResult { neighbors }
}
