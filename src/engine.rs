use crate::document::Document;
use kd_tree::{ItemAndDistance, KdPoint, KdTree, KdTreeN};
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
use std::convert::TryInto;
use typenum::U2;

pub type Embeddings = [f32; 384];

pub type Embedded<'a> = (&'a Document, Embeddings);

pub struct EmbeddedDocument {
    id: String,
    body: String,
    embeddings: Embeddings,
}

impl KdPoint for EmbeddedDocument {
    type Scalar = f32;
    type Dim = U2; // 2 dimensional tree.
    fn at(&self, k: usize) -> f32 {
        self.embeddings[k]
    }
}

pub type Index = KdTreeN<EmbeddedDocument, U2>;

pub fn index(documents: &[Document]) -> anyhow::Result<Index> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()?;
    let inputs: Vec<String> = documents.iter().map(|doc| doc.to_owned().body).collect();
    let encoded = model.encode(&inputs)?;
    let embeddings: Vec<Embeddings> = encoded.iter().map(|x| to_sized(x)).collect();

    let data: Vec<EmbeddedDocument> = documents
        .into_iter()
        .zip(embeddings.into_iter())
        .map(|(doc, emb)| EmbeddedDocument {
            id: doc.to_owned().id,
            body: doc.to_owned().body,
            embeddings: emb,
        })
        .collect();

    let index = KdTree::build_by(data, |a, b, k| {
        a.embeddings[k].partial_cmp(&b.embeddings[k]).unwrap()
    });

    Ok(index)
}

pub fn search<'a>(
    index: &'a Index,
    query: &'a str,
    k: usize,
) -> anyhow::Result<Vec<ItemAndDistance<'a, EmbeddedDocument, f32>>> {
    let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
        .create_model()?;
    let query = model.encode(&[query])?;
    let query = query
        .into_iter()
        .map(|x| to_sized(&x))
        .map(|embeddings| EmbeddedDocument {
            id: "".to_owned(),
            body: "".to_owned(),
            embeddings,
        })
        .next()
        .unwrap();
    let nearests = index.nearests(&query, k);

    Ok(nearests)
}

fn to_sized(arr: &[f32]) -> Embeddings {
    arr.try_into()
        .expect("Unable to apply fixed size to slice.")
}

#[cfg(test)]
mod tests {
    use crate::document::Document;

    use super::{index, search};

    #[test]
    fn it_works() {
        let documents: Vec<Document> = vec![
            Document {id: "1".to_owned(), body: "Is Qwik Faster than React Server Component?".to_owned()},
            Document {id: "2".to_owned(), body: "Tired of Slow Code Reviews? Read this".to_owned()},
            Document {id: "3".to_owned(), body: "Deploying Like Vercel and Netlify with Cloud Run: Live, Preview, and Modern Workflow".to_owned()},
            Document {id: "4".to_owned(), body: "Easiest Way to Understand Rust Modules Across Multiple Files".to_owned()},
            Document {id: "5".to_owned(), body: "Ex-Principal Engineer's Guide to Design Thinking and Continuous Delivery".to_owned()},
            Document {id: "6".to_owned(), body: "Building A Custom Google Maps Marker React Component Like Airbnb in Next.js".to_owned()},
            Document {id: "7".to_owned(), body: "Event Bus for React".to_owned()},
            Document {id: "8".to_owned(), body: "React Explained for Product Managers".to_owned()},
            Document {id: "9".to_owned(), body: "Stress Testing Concurrent Features in React 18: A Case Study of startTransition & 3D Rendering".to_owned()},
            Document {id: "10".to_owned(), body: "I Built A Snappy Static Full-text Search with WebAssembly, Rust, Next.js, and Xor Filters".to_owned()},
        ];
        let query = "is react fast";
        let index = index(&documents).unwrap();
        let result = search(&index, query, 5).unwrap();

        let expected = vec![
            "1".to_owned(),
            "7".to_owned(),
            "8".to_owned(),
            "9".to_owned(),
            "10".to_owned(),
        ];

        result.iter().map(|x| &x.item.id).for_each(|x| {
            assert_eq!(expected.contains(&x), true);
        });
    }
}
