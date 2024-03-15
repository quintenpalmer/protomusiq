use std::io;

use serde_json;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ArtistListResult {
    pub artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize)]
pub struct Artist {
    pub id: String,
    pub score: i32,
    pub name: String,
    #[serde(alias = "sort-name")]
    pub sort_name: String,
    pub isnis: Option<Vec<String>>,
    pub tags: Option<Vec<ArtistTag>>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistTag {
    pub count: i32,
    pub name: String,
}

impl ArtistListResult {
    pub fn from_json(json_str: String) -> ArtistListResult {
        let mut artist_result: ArtistListResult = serde_json::from_str(json_str.as_str()).unwrap();

        artist_result.artists.sort_by_key(|x| x.score);

        artist_result
    }
}

pub fn query_for_artist_raw(query: String) -> String {
    let mut request = ureq::get("https://musicbrainz.org/ws/2/artist");
    request = request.set("Accept", "application/json");
    request = request.set(
        "User-Agent",
        "musiqapp/0.9.0 ( quintenpalmer@protonmail.com )",
    );
    request = request.query_pairs([("query", query.as_str()), ("limit", "5")]);

    let resp = request.call().unwrap();
    //eprintln!("{}", resp.status());

    let resp_body = io::read_to_string(resp.into_reader()).unwrap();
    //println!("{}", resp_body);

    resp_body
}
