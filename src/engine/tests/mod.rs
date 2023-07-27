mod fixtures;

use crate::engine;
use crate::{EmbeddedResource, Resource};
use fixtures::*;
use rstest::*;

#[rstest]
fn it_indexes_embeddings(resource_fixture: Resource) {
    let index = engine::index(resource_fixture).unwrap();

    assert_eq!(index.tree.size(), 6);
}

#[rstest]
fn it_returns_vector_search_result(
    resource_fixture: Resource,
    question_fixture: [f32; 768],
    content_fixture: [&'static str; 6],
) {
    let index = engine::index(resource_fixture).unwrap();
    let query = engine::Query::Embeddings(question_fixture.to_vec());
    let result = engine::search(&index, &query, 6).unwrap();

    assert_eq!(result.get(0).unwrap().title, content_fixture[0]);
    assert_eq!(result.get(1).unwrap().title, content_fixture[1]);
    assert_eq!(result.get(2).unwrap().title, content_fixture[2]);
    assert_eq!(result.get(3).unwrap().title, content_fixture[4]);
    assert_eq!(result.get(4).unwrap().title, content_fixture[5]);
    assert_eq!(result.get(5).unwrap().title, content_fixture[3]);
}

#[rstest]
fn it_adds_embeddings_to_index(
    resource_fixture: Resource,
    content_fixture: [&'static str; 6],
    embedding_fixture: [[f32; 768]; 6],
) {
    let mut index = engine::index(resource_fixture).unwrap();
    let addition = Resource {
        embeddings: vec![EmbeddedResource {
            id: "5".to_owned(),
            title: content_fixture.get(5).unwrap().to_string(),
            url: "".to_owned(),
            embeddings: embedding_fixture[5].to_vec(),
        }],
    };

    engine::add(&mut index, &addition);
    assert_eq!(index.tree.size(), 7);
}

#[rstest]
fn it_removes_embeddings_from_index(
    resource_fixture: Resource,
    content_fixture: [&'static str; 6],
    embedding_fixture: [[f32; 768]; 6],
) {
    let mut index = engine::index(resource_fixture).unwrap();
    let target = Resource {
        embeddings: vec![EmbeddedResource {
            id: "1".to_owned(),
            title: content_fixture.get(1).unwrap().to_string(),
            url: "".to_owned(),
            embeddings: embedding_fixture[1].to_vec(),
        }],
    };

    engine::remove(&mut index, &target);
    assert_eq!(index.tree.size(), 5);
}

#[rstest]
fn it_clears_all_embeddings_from_index(resource_fixture: Resource) {
    let mut index = engine::index(resource_fixture).unwrap();
    assert_eq!(index.tree.size(), 6);
    assert_eq!(index.data.len(), 6);

    engine::clear(&mut index);
    assert_eq!(index.tree.size(), 0);
    assert_eq!(index.data.len(), 0);
}

#[rstest]
fn it_returns_the_size_of_index(resource_fixture: Resource) {
    let index = engine::index(resource_fixture).unwrap();
    assert_eq!(index.tree.size(), 6);
    assert_eq!(index.data.len(), 6);
    assert_eq!(engine::size(&index), 6);
}
