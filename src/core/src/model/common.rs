use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub enum LoadMode {
    NoCache,
    Latest,
    Json,
    Sqlite,
}
