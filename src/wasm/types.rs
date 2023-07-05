use serde::{Deserialize, Serialize};
use tsify::Tsify;

pub type NumberOfResult = usize;
pub type Query = Vec<f32>;
pub type SerializedIndex = String;

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
