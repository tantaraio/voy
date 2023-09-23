use crate::utils::set_panic_hook;
use crate::{engine, Neighbor, NumberOfResult, Query, Resource, SearchResult, SerializedIndex};

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::prelude::*;

pub struct Options {
    on_init: Option<Function>,
    on_index: Option<Function>,
    on_add: Option<Function>,
    on_remove: Option<Function>,
    on_search: Option<Function>,
    on_clear: Option<Function>,
    on_serialize: Option<Function>,
    on_deserialize: Option<Function>,
}

#[wasm_bindgen]
pub struct Voy {
    index: engine::Index,
    options: Option<Options>
}

fn reify(js_options: Option<Object>) -> Option<Options> {
    match js_options {
        None => None,
        Some(obj) => {
            let on_init  = match Reflect::get(&obj, &"onInit".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            let on_index  = match Reflect::get(&obj, &"onIndex".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            let on_add  = match Reflect::get(&obj, &"onAdd".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            let on_remove  = match Reflect::get(&obj, &"onRemove".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            let on_search  = match Reflect::get(&obj, &"onSearch".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            let on_clear  = match Reflect::get(&obj, &"onClear".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            let on_serialize  = match Reflect::get(&obj, &"onSerialize".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            let on_deserialize  = match Reflect::get(&obj, &"onDeserialize".into()) {
                Ok(callback) => Some(callback.dyn_into::<Function>().unwrap()),
                _ => None,
            };
            Some(Options {
                on_init,
                on_index,
                on_add,
                on_remove,
                on_search,
                on_clear,
                on_serialize,
                on_deserialize
            })
        }
    }
}

#[wasm_bindgen]
impl Voy {
    #[wasm_bindgen(constructor)]
    pub fn new(resource: Option<Resource>, options: Option<Object>) -> Voy {
        set_panic_hook();

        let resource: Resource = match resource {
            Some(res) => res,
            _ => Resource { embeddings: vec![] },
        };
        let index = engine::index(resource).unwrap();
        let opts = reify(options);

        if let Some(callbacks) = &opts {
            if let Some(callback) = &callbacks.on_init{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };

        Voy { index, options: opts }
    }

    pub fn serialize(&self) -> SerializedIndex {
        if let Some(callbacks) = &self.options {
            if let Some(callback) = &callbacks.on_serialize{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };
        serde_json::to_string(&self.index).unwrap()
    }

    pub fn deserialize(serialized_index: SerializedIndex, options: Option<Object>) -> Voy {
        let index: engine::Index = serde_json::from_str(&serialized_index).unwrap();
        let opts = reify(options);

        if let Some(callbacks) = &opts {
            if let Some(callback) = &callbacks.on_deserialize{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };
        Voy { index, options: opts }
    }

    pub fn index(&mut self, resource: Resource) {
        let index = engine::index(resource).unwrap();

        if let Some(callbacks) = &self.options {
            if let Some(callback) = &callbacks.on_index{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };
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
        
        if let Some(callbacks) = &self.options {
            if let Some(callback) = &callbacks.on_search{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };
        SearchResult { neighbors }
    }

    pub fn add(&mut self, resource: Resource) {
        engine::add(&mut self.index, &resource);

        if let Some(callbacks) = &self.options {
            if let Some(callback) = &callbacks.on_add{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };
    }

    pub fn remove(&mut self, resource: Resource) {
        engine::remove(&mut self.index, &resource);

        if let Some(callbacks) = &self.options {
            if let Some(callback) = &callbacks.on_remove{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };
    }

    pub fn clear(&mut self) {
        engine::clear(&mut self.index);
        
        if let Some(callbacks) = &self.options {
            if let Some(callback) = &callbacks.on_clear{
                callback.call0(&JsValue::undefined()).unwrap();
            }
        };
    }

    pub fn size(&self) -> usize {
        engine::size(&self.index)
    }
}
