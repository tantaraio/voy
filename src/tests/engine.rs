use super::engine_fixtures::{CONTENT, CONTENT_RAW, QUESTION};
use crate::engine::{index, search, Query};
use crate::{EmbeddedResource, Resource};

fn get_resource() -> Resource {
    let embeddings = CONTENT
        .iter()
        .enumerate()
        .map(|(i, x)| EmbeddedResource {
            id: i.to_string(),
            title: CONTENT_RAW.get(i).unwrap().to_string(),
            url: "".to_owned(),
            embeddings: x.to_vec(),
        })
        .collect();
    Resource { embeddings }
}

#[test]
fn it_indexes_embeddings() {
    let resource: Resource = get_resource();
    let index = index(resource).unwrap();

    assert_eq!(index.size(), 6);
}

#[test]
fn it_returns_vector_search_result() {
    let resource: Resource = get_resource();
    let index = index(resource).unwrap();

    let query = Query::Embeddings(QUESTION.to_vec());
    let result = search(&index, &query, 1).unwrap();

    assert_eq!(result.get(0).unwrap().item, 0);
}
