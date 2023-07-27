use crate::utils::set_panic_hook;
use crate::{engine, Neighbor, NumberOfResult, Query, Resource, SearchResult, SerializedIndex};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Voy {
    index: engine::Index,
}

#[wasm_bindgen]
impl Voy {
    #[wasm_bindgen(constructor)]
    pub fn new(resource: Option<Resource>) -> Voy {
        set_panic_hook();

        let resource: Resource = match resource {
            Some(res) => res,
            _ => Resource { embeddings: vec![] },
        };
        let index = engine::index(resource).unwrap();
        Voy { index }
    }

    pub fn serialize(&self) -> SerializedIndex {
        serde_json::to_string(&self.index).unwrap()
    }

    pub fn deserialize(serialized_index: SerializedIndex) -> Voy {
        let index: engine::Index = serde_json::from_str(&serialized_index).unwrap();
        Voy { index }
    }

    pub fn index(&mut self, resource: Resource) {
        let index = engine::index(resource).unwrap();
        self.index = index
    }

    pub fn search(&self, query: Query, k: NumberOfResult) -> SearchResult {
        let query: engine::Query = engine::Query::Embeddings(query);
        let neighbors = engine::search(&self.index, &query, k).unwrap();
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

    pub fn add(&mut self, resource: Resource) {
        engine::add(&mut self.index, &resource);
    }

    pub fn remove(&mut self, resource: Resource) {
        engine::remove(&mut self.index, &resource);
    }

    pub fn clear(&mut self) {
        engine::clear(&mut self.index);
    }

    pub fn size(&self) -> usize {
        engine::size(&self.index)
    }
}
