use crate::Resource;
use kd_tree::{KdPoint, KdTree, KdTreeN};
use serde::{Deserialize, Serialize};
use typenum::U2;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct EmbeddedResource {
    pub id: String,
    pub title: String,
    pub url: String,
    pub embeddings: Vec<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Query {
    Embeddings(Vec<f32>),
}

impl KdPoint for EmbeddedResource {
    type Scalar = f32;
    type Dim = U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.embeddings[k]
    }
}

pub type Index = KdTreeN<EmbeddedResource, U2>;

pub fn index(input: Resource) -> anyhow::Result<Index> {
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

    let index = KdTree::build_by(data, |a, b, k| {
        a.embeddings
            .clone()
            .into_iter()
            .nth(k)
            .partial_cmp(&b.embeddings.clone().into_iter().nth(k))
            .unwrap()
    });

    Ok(index)
}

pub fn search<'a>(
    index: &'a Index,
    query: &'a Query,
    k: usize,
) -> anyhow::Result<Vec<EmbeddedResource>> {
    let query: Vec<f32> = match query {
        Query::Embeddings(q) => q.to_owned(),
    };
    let query = EmbeddedResource {
        id: "".to_owned(),
        title: "".to_owned(),
        url: "".to_owned(),
        embeddings: query,
    };
    let nearests: Vec<EmbeddedResource> = index
        .nearests(&query, k)
        .into_iter()
        .map(|x| x.item.to_owned())
        .collect();

    Ok(nearests)
}

#[cfg(test)]
mod tests {
    use super::{index, search, Query};
    use crate::{EmbeddedResource, Resource};

    #[test]
    fn it_indexes_embeddings_and_returns_search_result() {
        let resource: Resource = Resource {
            embeddings: vec![
                EmbeddedResource {
                    id: "abd".to_owned(),
                    title: "That is a very happy Person".to_owned(),
                    url: "/path/to/one".to_owned(),
                    embeddings: vec![1.0, 2.0, 3.0],
                },
                EmbeddedResource {
                    id: "abd".to_owned(),
                    title: "That is a Happy Dog".to_owned(),
                    url: "/path/to/two".to_owned(),
                    embeddings: vec![3.0, 1.0, 2.0],
                },
                EmbeddedResource {
                    id: "abd".to_owned(),
                    title: "Today is a sunny day".to_owned(),
                    url: "/path/to/three".to_owned(),
                    embeddings: vec![2.0, 3.0, 1.0],
                },
            ],
        };

        let index = index(resource).unwrap();

        let query = Query::Embeddings(vec![3.1, 0.9, 2.1]);

        let result = search(&index, &query, 1).unwrap();

        assert_eq!(
            result.into_iter().nth(0).unwrap().title,
            "That is a Happy Dog",
        );
    }
}
