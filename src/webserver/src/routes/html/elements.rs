use musiqlibrary::model::ID;

use crate::contenttype;
use crate::model::{FullTrackMetadata, PlayPriority};

pub const NOATTRS: Vec<(String, String)> = Vec::new();
const NOKWS: Vec<String> = Vec::new();

fn render_element<W: ToString, K: ToString, V: ToString, S: ToString>(
    tag_name: &'static str,
    keywords: Vec<W>,
    attributes: Vec<(K, V)>,
    contents: Option<Vec<S>>,
) -> String {
    let rendered_attrs = attributes
        .into_iter()
        .fold("".to_string(), |total, (key, value)| {
            format!("{} {}=\"{}\"", total, key.to_string(), value.to_string())
        });
    let rendered_keywords = keywords.into_iter().fold("".to_string(), |total, keyword| {
        format!("{} {}", total, keyword.to_string())
    });
    match contents {
        Some(c) => format!(
            "<{} {} {}>{}</{}>",
            tag_name,
            rendered_keywords,
            rendered_attrs,
            c.into_iter().fold("".to_string(), |total, current| format!(
                "{} {}",
                total,
                current.to_string()
            )),
            tag_name,
        ),
        None => format!("<{} {}/>", tag_name, rendered_attrs),
    }
}

#[allow(dead_code)]
pub fn html<L: ToString, S: ToString>(lang: L, contents: Vec<S>) -> String {
    format!(
        "{}{}",
        "<!DOCTYPE html>",
        render_element("html", NOKWS, vec![("lang", lang)], Some(contents))
    )
}

#[allow(dead_code)]
pub fn head<S: ToString>(contents: Vec<S>) -> String {
    render_element("head", NOKWS, NOATTRS, Some(contents))
}

#[allow(dead_code)]
pub fn meta<C: ToString>(charset: C) -> String {
    render_element(
        "meta",
        NOKWS,
        vec![("charset", charset)],
        None::<Vec<String>>,
    )
}

#[allow(dead_code)]
pub fn style<T: ToString, S: ToString>(type_: T, contents: Vec<S>) -> String {
    render_element("style", NOKWS, vec![("type", type_)], Some(contents))
}

#[allow(dead_code)]
pub fn link<S: ToString>(rel: S, href: S) -> String {
    render_element(
        "link",
        NOKWS,
        vec![("rel", rel), ("href", href)],
        None::<Vec<String>>,
    )
}

#[allow(dead_code)]
pub fn script<S: ToString, C: ToString>(src: Option<S>, contents: Vec<C>) -> String {
    let attrs = match src {
        Some(s) => vec![("src".to_string(), s.to_string())],
        None => NOATTRS,
    };
    render_element("script", NOKWS, attrs, Some(contents))
}

#[allow(dead_code)]
pub fn body<S: ToString>(contents: Vec<S>) -> String {
    render_element("body", NOKWS, NOATTRS, Some(contents))
}

#[allow(dead_code)]
pub fn h1<S: ToString>(contents: S) -> String {
    format!("<h1>{}<h1>", contents.to_string())
}

#[allow(dead_code)]
pub fn h2<S: ToString>(contents: S) -> String {
    format!("<h2>{}<h2>", contents.to_string())
}

#[allow(dead_code)]
pub fn h3<S: ToString>(contents: S) -> String {
    format!("<h3>{}<h3>", contents.to_string())
}

#[allow(dead_code)]
pub fn div<K: ToString, V: ToString, S: ToString>(
    attributes: Vec<(K, V)>,
    contents: Vec<S>,
) -> String {
    render_element("div", NOKWS, attributes, Some(contents))
}

#[allow(dead_code)]
pub fn span<K: ToString, V: ToString, S: ToString>(
    attributes: Vec<(K, V)>,
    contents: Vec<S>,
) -> String {
    render_element("span", NOKWS, attributes, Some(contents))
}

#[allow(dead_code)]
pub fn a<L: ToString, C: ToString>(link: L, contents: C) -> String {
    format!(
        "<a href=\"{}\">{}</a>",
        link.to_string(),
        contents.to_string()
    )
}

#[allow(dead_code)]
pub fn img<S: ToString>(src: S, width: u64, height: u64) -> String {
    format!(
        "<img src=\"{}\" width={}px height={}px />",
        src.to_string(),
        width,
        height,
    )
}

#[allow(dead_code)]
pub fn audio<S: ToString, K: ToString, V: ToString>(src: Option<S>, attrs: Vec<(K, V)>) -> String {
    let mut attrs = attrs
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<Vec<(String, String)>>();
    match src {
        Some(s) => attrs.push(("src".to_string(), s.to_string())),
        None => (),
    };
    render_element("audio", vec!["controls"], attrs, Some(Vec::<String>::new()))
}

#[allow(dead_code)]
pub fn source<S: ToString, T: ToString, K: ToString, V: ToString, C: ToString>(
    src: Option<S>,
    type_: Option<T>,
    attrs: Vec<(K, V)>,
    contents: Vec<C>,
) -> String {
    let mut attrs = attrs
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<Vec<(String, String)>>();
    match src {
        Some(s) => attrs.push(("src".to_string(), s.to_string())),
        None => (),
    };
    match type_ {
        Some(t) => attrs.push(("type".to_string(), t.to_string())),
        None => (),
    };
    render_element("source", NOKWS, attrs, Some(contents))
}

#[allow(dead_code)]
pub fn table(input: Vec<Vec<String>>) -> String {
    inner_table(false, input)
}

#[allow(dead_code)]
pub fn table_header(input: Vec<Vec<String>>) -> String {
    inner_table(true, input)
}

fn inner_table(has_header: bool, mut input: Vec<Vec<String>>) -> String {
    let headers = if has_header {
        let inner_headers = input.remove(0);
        if inner_headers.len() != input.get(0).unwrap().len() {
            eprintln!(
                "table was passed with headers {} != body {}",
                inner_headers.len(),
                input.get(0).unwrap().len()
            )
        }
        inner_headers
    } else {
        vec![]
    };
    format!(
        "<table><thead><tr>{}</tr></thead><tbody>{}</tbody></table>",
        table_row_header(headers),
        input.into_iter().fold("".to_string(), |total, row| {
            format!("{}<tr>{}</tr>", total, table_row(row))
        }),
    )
}

fn table_row(row: Vec<String>) -> String {
    row.into_iter().fold("".to_string(), |total, value| {
        format!("{}<td>{}</td>", total, value)
    })
}

fn table_row_header(row: Vec<String>) -> String {
    row.into_iter().fold("".to_string(), |total, value| {
        format!("{}<th>{}</th>", total, value)
    })
}

pub fn artist_link(album_artist: &String) -> String {
    a(
        format!("/artists/{}/albums", ID::new(&album_artist).hashed(),),
        album_artist,
    )
}

pub fn artist_album_link(album_artist: &String, album: &String) -> String {
    a(
        format!(
            "/artists/{}/albums/{}/disc_tracks",
            ID::new(&album_artist).hashed(),
            ID::new(&album).hashed(),
        ),
        album,
    )
}

pub fn artist_album_image(album_artist: &String, album: &String) -> String {
    img(
        format!(
            "/artists/{}/albums/{}/cover.jpg",
            ID::new(&album_artist).hashed(),
            ID::new(&album).hashed(),
        ),
        200,
        200,
    )
}

pub fn artist_album_disc_track_link(track: &FullTrackMetadata) -> String {
    a(
        format!(
            "/artists/{}/albums/{}/disc_tracks/{}/tracks/{}/info",
            ID::new(&track.album_artist).hashed(),
            ID::new(&track.album).hashed(),
            track.disc,
            track.track,
        ),
        track.title.clone(),
    )
}

pub fn artist_album_disc_track_download_link(track: &FullTrackMetadata) -> String {
    a(artist_album_disc_track_play_url(&track), "📥")
}

pub fn artist_album_disc_track_play_url(track: &FullTrackMetadata) -> String {
    format!(
        "/artists/{}/albums/{}/disc_tracks/{}/tracks/{}/audio/song.{}",
        ID::new(&track.album_artist).hashed(),
        ID::new(&track.album).hashed(),
        track.disc,
        track.track,
        track.ext,
    )
}

pub fn artist_album_disc_track_play_link(
    track: &FullTrackMetadata,
    play_priority: PlayPriority,
) -> String {
    match play_priority {
        PlayPriority::Now => a(
            format!(
                "javascript:{{ \
                play_songs_next_in_play_queue([{}]); \
                play_next_song_in_queue(); \
            }}",
                track_playback_json(&track),
            ),
            "<img width=15px src=\"/images/play_single.png\"/>",
        ),
        PlayPriority::Next => a(
            format!(
                "javascript:{{ \
                play_songs_next_in_play_queue([{}]); \
                undefined; \
            }}",
                track_playback_json(&track),
            ),
            "<img width=15px src=\"/images/play_single_next.png\"/>",
        ),
        PlayPriority::Append => a(
            format!(
                "javascript:{{ \
                append_songs_to_play_queue([{}]); \
                undefined; \
            }}",
                track_playback_json(&track),
            ),
            "<img width=15px src=\"/images/play_single_append.png\"/>",
        ),
    }
}

pub fn artist_album_play_link(
    tracks: &Vec<FullTrackMetadata>,
    play_priority: PlayPriority,
) -> String {
    match play_priority {
        PlayPriority::Now => a(
            format!(
                "javascript:{{ \
                play_songs_next_in_play_queue({}); \
                play_next_song_in_queue(); \
            }}",
                track_list_playback_json(tracks)
            ),
            "<img width=15px src=\"/images/play_single.png\"/>",
        ),
        PlayPriority::Next => a(
            format!(
                "javascript:{{ \
                play_songs_next_in_play_queue({}); \
                undefined; \
            }}",
                track_list_playback_json(tracks)
            ),
            "<img width=15px src=\"/images/play_single_next.png\"/>",
        ),
        PlayPriority::Append => a(
            format!(
                "javascript:{{ \
                append_songs_to_play_queue({}); \
                undefined; \
            }}",
                track_list_playback_json(tracks)
            ),
            "<img width=15px src=\"/images/play_single_append.png\"/>",
        ),
    }
}

fn track_list_playback_json(tracks: &Vec<FullTrackMetadata>) -> String {
    format!(
        "[{}]",
        tracks
            .iter()
            .map(|track| track_playback_json(track))
            .collect::<Vec<String>>()
            .join(", ")
    )
}

fn track_playback_json(track: &FullTrackMetadata) -> String {
    format!(
        "{{src: '{}', title: '{}', contenttype: '{}'}}",
        artist_album_disc_track_play_url(&track),
        track.title.replace("'", "&quot").replace("\"", "&quot"),
        contenttype::audio_content_type_from_ext(&track.ext),
    )
}
