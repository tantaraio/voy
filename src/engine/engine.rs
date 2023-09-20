use crate::Resource;
use kiddo::float::{distance::squared_euclidean, kdtree::KdTree};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryInto};

use super::hash;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]

pub struct Document {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Query {
    // TODO: support query in string
    // Phrase(String)
    Embeddings(Vec<f32>),
}

// Wasm has a 4GB memory limit. Should make sure the bucket size and capacity
// doesn't exceed it and cause stack overflow.
// More detail: https://v8.dev/blog/4gb-wasm-memory
const BUCKET_SIZE: usize = 32;

pub type Tree = KdTree<f32, u64, 768, BUCKET_SIZE, u16>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    // "IDX" is set to u16 to optimize CPU cache.
    // Read more: https://github.com/sdd/kiddo/blob/7a0bb6ecce39963b27ffdca913c6be7a265e3523/src/types.rs#L35
    pub tree: Tree,
    pub data: HashMap<u64, Document>,
}

pub fn index(resource: Resource) -> anyhow::Result<Index> {
    let data_vec: Vec<(u64, Document)> = resource
        .embeddings
        .iter()
        .map(|resource| Document {
            id: resource.id.to_owned(),
            title: resource.title.to_owned(),
            url: resource.url.to_owned(),
        })
        .map(|document| (hash(&document), document))
        .collect();

    let data: HashMap<u64, Document> = data_vec.clone().into_iter().collect();

    let mut tree: Tree = KdTree::new();

    resource
        .embeddings
        .iter()
        .zip(data_vec.iter())
        .for_each(|(resource, data)| {
            let mut embeddings = resource.embeddings.clone();
            embeddings.resize(768, 0.0);

            let query: &[f32; 768] = &embeddings.try_into().unwrap();
            // "item" holds the position of the document in "data"
            tree.add(query, data.0);
        });

    Ok(Index { tree, data })
}

pub fn search<'a>(index: &'a Index, query: &'a Query, k: usize) -> anyhow::Result<Vec<Document>> {
    let mut query: Vec<f32> = match query {
        Query::Embeddings(q) => q.to_owned(),
    };
    query.resize(768, 0.0);

    let query: &[f32; 768] = &query.try_into().unwrap();
    let neighbors = index.tree.nearest_n(query, k, &squared_euclidean);

    let mut result: Vec<Document> = vec![];

    for neighbor in &neighbors {
        let doc = index.data.get(&neighbor.item);
        if let Some(document) = doc {
            result.push(document.to_owned());
        }
    }

    Ok(result)
}

pub fn add<'a>(index: &'a mut Index, resource: &'a Resource) {
    for item in &resource.embeddings {
        let mut embeddings = item.embeddings.clone();
        embeddings.resize(768, 0.0);

        let query: &[f32; 768] = &embeddings.try_into().unwrap();
        let doc = Document {
            id: item.id.to_owned(),
            title: item.title.to_owned(),
            url: item.url.to_owned(),
        };
        let id = hash(&doc);
        index.data.insert(id, doc);
        index.tree.add(query, id);
    }
}

pub fn remove<'a>(index: &'a mut Index, resource: &'a Resource) {
    for item in &resource.embeddings {
        let mut embeddings = item.embeddings.clone();
        embeddings.resize(768, 0.0);

        let query: &[f32; 768] = &embeddings.try_into().unwrap();
        let id = hash(&Document {
            id: item.id.to_owned(),
            title: item.title.to_owned(),
            url: item.url.to_owned(),
        });

        index.tree.remove(query, id);
        index.data.remove(&id);
    }
}

pub fn clear<'a>(index: &'a mut Index) {
    // simply assign a new tree and data because traversing the nodes to perform removal is the only alternative.
    // Kiddo provides only basic removal. See more: https://github.com/sdd/kiddo/issues/76
    index.tree = KdTree::new();
    index.data = HashMap::new();
}

pub fn size<'a>(index: &'a Index) -> usize {
    index.data.len()
}
