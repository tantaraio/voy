use super::engine_fixtures::{CONTENT, CONTENT_RAW, QUESTION};
use crate::engine::{add, index, remove, search, Query};
use crate::{EmbeddedResource, Resource};

fn get_resource(k: usize) -> Resource {
    let embeddings = CONTENT
        .iter()
        .take(k)
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
    let resource: Resource = get_resource(6);
    let index = index(resource).unwrap();

    assert_eq!(index.tree.size(), 6);
}

#[test]
fn it_returns_vector_search_result() {
    let resource: Resource = get_resource(6);
    let index = index(resource).unwrap();

    let query = Query::Embeddings(QUESTION.to_vec());
    let result = search(&index, &query, 1).unwrap();

    assert_eq!(result.get(0).unwrap().title, CONTENT_RAW[0]);
}

#[test]
fn it_adds_embeddings_to_index() {
    let resource: Resource = get_resource(2);
    let mut index = index(resource).unwrap();
    let addition = Resource {
        embeddings: vec![EmbeddedResource {
            id: "5".to_owned(),
            title: CONTENT_RAW.get(5).unwrap().to_string(),
            url: "".to_owned(),
            embeddings: CONTENT[5].to_vec(),
        }],
    };
    add(&mut index, &addition);

    assert_eq!(index.tree.size(), 3);
}

#[test]
fn it_removes_embeddings_from_index() {
    let resource: Resource = get_resource(2);
    let mut index = index(resource).unwrap();
    let target = Resource {
        embeddings: vec![EmbeddedResource {
            id: "1".to_owned(),
            title: CONTENT_RAW.get(1).unwrap().to_string(),
            url: "".to_owned(),
            embeddings: CONTENT[1].to_vec(),
        }],
    };
    remove(&mut index, &target);

    assert_eq!(index.tree.size(), 1);
}
