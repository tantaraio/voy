use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Document {
    pub id: String,
    pub body: String,
}
