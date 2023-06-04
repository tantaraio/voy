use crate::Resource;
use kiddo::float::{distance::squared_euclidean, kdtree::KdTree};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

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

pub type Tree = KdTree<f32, usize, 768, 32, u16>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Index {
    // "IDX" is set to u16 to optimize CPU cache.
    // Read more: https://github.com/sdd/kiddo/blob/7a0bb6ecce39963b27ffdca913c6be7a265e3523/src/types.rs#L35
    pub tree: Tree,
    pub data: Vec<Document>,
}

pub fn index(resource: Resource) -> anyhow::Result<Index> {
    let data: Vec<Document> = resource
        .embeddings
        .iter()
        .map(|resource| Document {
            id: resource.id.to_owned(),
            title: resource.title.to_owned(),
            url: resource.url.to_owned(),
        })
        .collect();

    let mut tree: Tree = KdTree::new();

    resource
        .embeddings
        .iter()
        .enumerate()
        .for_each(|(index, resource)| {
            let query: &[f32; 768] = &resource.embeddings[..768].try_into().unwrap();
            // "item" holds the position of the document in "data"
            tree.add(query, index)
        });

    Ok(Index { tree, data })
}

pub fn search<'a>(index: &'a Index, query: &'a Query, k: usize) -> anyhow::Result<Vec<Document>> {
    let query: Vec<f32> = match query {
        Query::Embeddings(q) => q.to_owned(),
    };
    let query: &[f32; 768] = &query[..768].try_into().unwrap();
    let neighbors = index.tree.nearest_n(query, k, &squared_euclidean);

    let mut result: Vec<Document> = vec![];

    for neighbor in &neighbors {
        let doc = index.data[neighbor.item].to_owned();
        result.push(doc);
    }

    Ok(result)
}

pub fn add<'a>(index: &'a mut Index, resource: &'a Resource) {
    for item in &resource.embeddings {
        let query: &[f32; 768] = item.embeddings[..768].try_into().unwrap();
        let doc = Document {
            id: item.id.to_owned(),
            title: item.title.to_owned(),
            url: item.url.to_owned(),
        };
        index.data.push(doc);
        index.tree.add(query, index.data.len());
    }
}

pub fn remove<'a>(index: &'a mut Index, resource: &'a Resource) {
    for item in &resource.embeddings {
        let query: &[f32; 768] = item.embeddings[..768].try_into().unwrap();
        let doc_index = index.data.iter().position(|x| x.id == item.id);

        if let Some(i) = doc_index {
            index.tree.remove(query, i);
            index.data.remove(i);
        }
    }
}
