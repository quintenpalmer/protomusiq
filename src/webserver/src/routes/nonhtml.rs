use std::convert::Infallible;
use std::fs;
use std::path;

use hyper::{header, Body, Response};

use crate::contenttype;
use crate::ds::Datastore;
use crate::model;

pub fn static_assets(asset_name: &str) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(match asset_name {
        "stylesheet.css" => fs::read("static/stylesheet.css").unwrap(),
        "main.js" => fs::read("static/main.js").unwrap(),
        _ => fs::read("images/not_found.png").unwrap(),
    })))
    .map(|mut resp| {
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(
                match path::Path::new(&asset_name)
                    .extension()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .as_str()
                {
                    "css" => "text/css",
                    "js" => "text/javascript",
                    ext => panic!("unknown media file requested: {}", ext),
                },
            ),
        );
        resp
    })
}

pub fn image(image_name: &str) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(match image_name {
        "favicon.ico" => fs::read("images/favicon.ico").unwrap(),
        "play_single.png" => fs::read("images/play_single.png").unwrap(),
        "play_single_next.png" => fs::read("images/play_single_next.png").unwrap(),
        "play_single_append.png" => fs::read("images/play_single_append.png").unwrap(),
        _ => panic!("could not find image: {}", image_name),
    })))
    .map(|mut resp| {
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/png"),
        );
        resp
    })
}

pub fn album_art(
    artist_id: model::ID,
    album_id: model::ID,
    ds: Datastore,
) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(
        ds.get_artist_album_cover(artist_id, album_id),
    )))
    .map(|mut resp| {
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("image/jpg"),
        );
        resp
    })
}

pub fn audio(
    artist_id: model::ID,
    album_id: model::ID,
    disc_no: u64,
    track_no: u64,
    filename_with_ext: String,
    ds: Datastore,
) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from(
        ds.get_artist_album_track_audio_from_id(artist_id, album_id, disc_no, track_no),
    )))
    .map(|mut resp| {
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static(contenttype::audio_content_type_from_ext(
                path::Path::new(&filename_with_ext)
                    .extension()
                    .unwrap()
                    .to_string_lossy()
                    .to_string()
                    .as_str(),
            )),
        );
        resp
    })
}
