mod engine;
mod utils;

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use utils::set_panic_hook;
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
    pub neighbors: Vec<Neighbor>,
}

#[wasm_bindgen]
pub fn index(resource: Resource) -> SerializedIndex {
    set_panic_hook();

    let index = engine::index(resource);
    match index {
        Ok(tree) => serde_json::to_string(&tree).unwrap(),
        _ => "".to_owned(),
    }
}

#[wasm_bindgen]
pub fn search(index: SerializedIndex, query: Query, k: NumberOfResult) -> SearchResult {
    set_panic_hook();

    let index: engine::Index = serde_json::from_str(&index).unwrap();
    let query: engine::Query = engine::Query::Embeddings(query);

    let neighbors = engine::search(&index, &query, k).unwrap();
    let neighbors: Vec<Neighbor> = neighbors
        .into_iter()
        .map(|x| Neighbor {
            id: x.id,
            title: x.title,
            url: x.url,
        })
        .collect();

    SearchResult { neighbors }
}

#[wasm_bindgen]
pub fn add(index: SerializedIndex, resource: Resource) -> SerializedIndex {
    set_panic_hook();

    let mut index: engine::Index = serde_json::from_str(&index).unwrap();
    engine::add(&mut index, &resource);

    serde_json::to_string(&index).unwrap()
}

#[wasm_bindgen]
pub fn remove(index: SerializedIndex, resource: Resource) -> SerializedIndex {
    set_panic_hook();

    let mut index: engine::Index = serde_json::from_str(&index).unwrap();
    engine::remove(&mut index, &resource);

    serde_json::to_string(&index).unwrap()
}
