mod html;
mod nonhtml;

use std::convert::Infallible;

use hyper::{header, Body, Method, Request, Response, StatusCode};

use crate::ds::Datastore;
use crate::model;

use html::main_html_wrapper;

pub async fn service_handler(
    req: Request<Body>,
    ds: Datastore,
) -> Result<Response<Body>, Infallible> {
    let path_frags = req
        .uri()
        .path()
        .split('/')
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();

    println!(
        "responding to: {} ({}) ({:?})",
        req.uri().path(),
        req.method(),
        path_frags
    );
    match (req.method(), path_frags.as_slice()) {
        // Serve hard-coded images
        (&Method::GET, ["favicon.ico"]) => nonhtml::image("favicon.ico"),
        (&Method::GET, ["static", asset_name]) => nonhtml::static_assets(asset_name),
        (&Method::GET, ["images", image_name]) => nonhtml::image(image_name),
        (&Method::GET, ["artists", artist_id, "albums", album_id, "cover.jpg"]) => {
            nonhtml::album_art(
                model::ID::from_u64(artist_id.parse::<u64>().unwrap()),
                model::ID::from_u64(album_id.parse::<u64>().unwrap()),
                ds,
            )
        }
        (
            &Method::GET,
            ["artists", artist_id, "albums", album_id, "disc_tracks", disc_no, "tracks", track_no, "audio", filename_with_ext],
        ) => nonhtml::audio(
            model::ID::from_u64(artist_id.parse::<u64>().unwrap()),
            model::ID::from_u64(album_id.parse::<u64>().unwrap()),
            disc_no.parse::<u64>().unwrap(),
            track_no.parse::<u64>().unwrap(),
            filename_with_ext.to_string(),
            ds,
        ),
        (method, frags) => handle_pages(method, frags, ds),
    }
}

fn handle_pages(
    method: &Method,
    frags: &[&str],
    ds: Datastore,
) -> Result<Response<Body>, Infallible> {
    let found = match (method, frags) {
        // Serve some instructions at /
        (&Method::GET, []) => Ok(html::index()),
        (&Method::GET, ["tree"]) => Ok(html::tree(ds)),
        (&Method::GET, ["oldtree"]) => Ok(html::old_tree(ds)),
        (&Method::GET, ["artists"]) => Ok(html::artists(ds)),
        (&Method::GET, ["artists", artist_id, "albums"]) => Ok(html::artist_albums(
            model::ID::from_u64(artist_id.parse::<u64>().unwrap()),
            ds,
        )),
        (&Method::GET, ["artists", artist_id, "albums", album_id, "disc_tracks"]) => {
            Ok(html::artist_album_tracks(
                model::ID::from_u64(artist_id.parse::<u64>().unwrap()),
                model::ID::from_u64(album_id.parse::<u64>().unwrap()),
                ds,
            ))
        }
        (
            &Method::GET,
            ["artists", artist_id, "albums", album_id, "disc_tracks", disc_no, "tracks", track_no, "info"],
        ) => Ok(html::artist_album_track_info(
            model::ID::from_u64(artist_id.parse::<u64>().unwrap()),
            model::ID::from_u64(album_id.parse::<u64>().unwrap()),
            disc_no.parse::<u64>().unwrap(),
            track_no.parse::<u64>().unwrap(),
            ds,
        )),
        (&Method::GET, ["albums"]) => Ok(html::albums(ds)),
        (&Method::GET, ["tracks"]) => Ok(html::tracks(ds)),
        _ => Err(html::not_found()),
    };
    match found {
        Ok((breadcrumbs, content)) => Ok(Response::new(Body::from(main_html_wrapper(
            breadcrumbs,
            content,
        ))))
        .map(|mut resp| {
            resp.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/html"),
            );
            resp
        }),
        Err(not_found_resp) => {
            let mut resp = Response::new(Body::from(not_found_resp));
            resp.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("text/html"),
            );
            *resp.status_mut() = StatusCode::NOT_FOUND;
            Ok(resp)
        }
    }
}
