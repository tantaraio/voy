use crate::Input;
use kd_tree::{ItemAndDistance, KdPoint, KdTree, KdTreeN};
use serde::{Deserialize, Serialize};
use typenum::U2;

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbeddedDocument {
    id: String,
    title: String,
    url: String,
    body: String,
    embeddings: Vec<f32>,
}

pub enum Query {
    Embeddings(Vec<f32>),
}

impl KdPoint for EmbeddedDocument {
    type Scalar = f32;
    type Dim = U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.embeddings[k]
    }
}

pub type Index = KdTreeN<EmbeddedDocument, U2>;

pub fn index(input: Input) -> anyhow::Result<Index> {
    let data: Vec<EmbeddedDocument> = input
        .embeddings
        .into_iter()
        .map(|emb| EmbeddedDocument {
            id: emb.id,
            title: emb.title,
            body: "".to_owned(),
            url: emb.url,
            embeddings: emb.embdeddings,
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
    query: Query,
    k: usize,
) -> anyhow::Result<Vec<ItemAndDistance<'a, EmbeddedDocument, f32>>> {
    let query: Vec<f32> = match query {
        Query::Embeddings(q) => q,
        _ => vec![],
    };
    let query = EmbeddedDocument {
        id: "".to_owned(),
        title: "".to_owned(),
        url: "".to_owned(),
        body: "".to_owned(),
        embeddings: query,
    };
    let nearests = index.nearests(&query, k);

    Ok(nearests)
}

#[cfg(test)]
mod tests {
    use super::{index, search, Query};
    use crate::{Embedding, Input};

    #[test]
    fn it_indexes_embeddings_and_returns_search_result() {
        let input: Input = Input {
            embeddings: vec![
                Embedding {
                    id: "abd".to_owned(),
                    title: "That is a very happy Person".to_owned(),
                    url: "/path/to/one".to_owned(),
                    embdeddings: vec![1.0, 2.0, 3.0],
                },
                Embedding {
                    id: "abd".to_owned(),
                    title: "That is a Happy Dog".to_owned(),
                    url: "/path/to/two".to_owned(),
                    embdeddings: vec![3.0, 1.0, 2.0],
                },
                Embedding {
                    id: "abd".to_owned(),
                    title: "Today is a sunny day".to_owned(),
                    url: "/path/to/three".to_owned(),
                    embdeddings: vec![2.0, 3.0, 1.0],
                },
            ],
        };

        let index = index(input).unwrap();

        let query = Query::Embeddings(vec![3.1, 0.9, 2.1]);

        let result = search(&index, query, 1).unwrap();

        assert_eq!(
            result.into_iter().nth(0).unwrap().item.title,
            "That is a Happy Dog",
        );
    }
}
