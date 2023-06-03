use crate::Resource;
use kiddo::float::{distance::squared_euclidean, kdtree::KdTree, neighbour::Neighbour};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use typenum::U2;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

pub struct EmbeddedResource {
    pub id: String,
    pub title: String,
    pub url: String,
    pub embeddings: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Query {
    // TODO: support query in string
    // Phrase(String)
    Embeddings(Vec<f32>),
}

impl kd_tree::KdPoint for EmbeddedResource {
    type Scalar = f32;
    type Dim = U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.embeddings[k]
    }
}

// "IDX" is set to u16 to optimize CPU cache.
// Read more: https://github.com/sdd/kiddo/blob/7a0bb6ecce39963b27ffdca913c6be7a265e3523/src/types.rs#L35
pub type Index = KdTree<f32, usize, 768, 32, u16>;

pub fn index(input: Resource) -> anyhow::Result<Index> {
    // feed the embeddings and index from the hashmap to kiddo
    let mut tree: Index = KdTree::new();

    let data: Vec<EmbeddedResource> = input
        .embeddings
        .into_iter()
        .map(|emb| EmbeddedResource {
            id: emb.id,
            title: emb.title,
            url: emb.url,
            embeddings: emb.embeddings,
        })
        .collect();

    data.iter().enumerate().for_each(|(index, x)| {
        let query: &[f32; 768] = &x.embeddings[..768].try_into().unwrap();
        tree.add(query, index)
    });

    Ok(tree)
}

pub fn search<'a>(
    index: &'a Index,
    query: &'a Query,
    k: usize,
) -> anyhow::Result<Vec<Neighbour<f32, usize>>> {
    let query: Vec<f32> = match query {
        Query::Embeddings(q) => q.to_owned(),
    };
    let query: &[f32; 768] = &query[..768].try_into().unwrap();
    let neighbors = index.nearest_n(query, k, &squared_euclidean);

    Ok(neighbors)
}
