use crate::{
    engine, utils::set_panic_hook, Neighbor, NumberOfResult, Query, Resource, SearchResult,
    SerializedIndex,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn index(resource: Resource) -> SerializedIndex {
    set_panic_hook();

    let index = engine::index(resource);
    match index {
        Ok(idx) => serde_json::to_string(&idx).unwrap(),
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

#[wasm_bindgen]
pub fn clear(index: SerializedIndex) -> SerializedIndex {
    set_panic_hook();

    let mut index: engine::Index = serde_json::from_str(&index).unwrap();
    engine::clear(&mut index);

    serde_json::to_string(&index).unwrap()
}

#[wasm_bindgen]
pub fn size(index: SerializedIndex) -> usize {
    set_panic_hook();

    let index: engine::Index = serde_json::from_str(&index).unwrap();

    engine::size(&index)
}
