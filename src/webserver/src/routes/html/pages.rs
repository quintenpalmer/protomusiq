use crate::ds::Datastore;
use crate::model::{ArtistInfo, PlayPriority};

use super::elements::*;

use crate::model;

pub fn not_found() -> String {
    html(
        "en-US",
        vec![
            head(vec![meta("utf-8")]),
            body(vec![
                div(NOATTRS, vec![a("/", "Back Home")]),
                div(NOATTRS, vec!["Page Not Found".to_string()]),
            ]),
        ],
    )
}

pub fn index() -> (Vec<(String, String)>, String) {
    (
        Vec::new(),
        table(vec![vec![
            h1(a("/artists/", "Artists")),
            h1(a("/albums/", "Albums")),
            h1(a("/tracks/", "Tracks")),
            h1(a("/tree/", "Tree")),
        ]]),
    )
}

pub fn artists(ds: Datastore) -> (Vec<(String, String)>, String) {
    (
        vec![("/artists".to_string(), "Artists".to_string())],
        table(
            ds.list_artists()
                .into_iter()
                .map(|album_artist| {
                    vec![
                        artist_album_image(
                            &album_artist.artist_name,
                            &ds.list_artist_albums(album_artist.artist_id)
                                .first()
                                .unwrap()
                                .album_name,
                        ),
                        artist_link(&album_artist.artist_name),
                    ]
                })
                .collect(),
        ),
    )
}

pub fn artist_albums(artist_id: model::ID, ds: Datastore) -> (Vec<(String, String)>, String) {
    let album_artist = ds.get_artist_from_id(artist_id.clone());
    (
        vec![
            ("/artists".to_string(), "Artists".to_string()),
            (
                format!("/artists/{}/albums", artist_id.hashed()),
                album_artist.artist_name.clone(),
            ),
        ],
        table(
            ds.list_artist_albums(artist_id)
                .into_iter()
                .map(|album_info| {
                    let album_tracks = ds.list_artist_album_tracks(
                        album_artist.artist_id.clone(),
                        album_info.album_id.clone(),
                    );
                    vec![
                        artist_album_image(&album_artist.artist_name, &album_info.album_name),
                        artist_album_play_link(&album_tracks, PlayPriority::Now),
                        artist_album_play_link(&album_tracks, PlayPriority::Next),
                        artist_album_play_link(&album_tracks, PlayPriority::Append),
                        artist_album_link(&album_artist.artist_name, &album_info.album_name),
                    ]
                })
                .collect(),
        ),
    )
}

pub fn artist_album_tracks(
    artist_id: model::ID,
    album_id: model::ID,
    ds: Datastore,
) -> (Vec<(String, String)>, String) {
    let album_artist = ds.get_artist_from_id(artist_id.clone());
    let album = ds.get_artist_album_from_id(artist_id.clone(), album_id.clone());
    (
        vec![
            ("/artists".to_string(), "Artists".to_string()),
            (
                format!("/artists/{}/albums", artist_id.hashed()),
                album_artist.artist_name.clone(),
            ),
            (
                format!(
                    "/artists/{}/albums/{}/disc_tracks",
                    artist_id.hashed(),
                    album_id.hashed()
                ),
                album.album_name.clone(),
            ),
        ],
        {
            let album_tracks = ds.list_artist_album_tracks(artist_id, album_id);

            table(vec![vec![
                artist_album_image(&album_artist.artist_name, &album.album_name),
                table_header({
                    let mut trs = vec![vec![
                        artist_album_play_link(&album_tracks, PlayPriority::Now),
                        artist_album_play_link(&album_tracks, PlayPriority::Next),
                        artist_album_play_link(&album_tracks, PlayPriority::Append),
                        "".to_string(),
                    ]];
                    trs.append(
                        &mut album_tracks
                            .into_iter()
                            .map(|track| {
                                vec![
                                    artist_album_disc_track_play_link(&track, PlayPriority::Now),
                                    artist_album_disc_track_play_link(&track, PlayPriority::Next),
                                    artist_album_disc_track_play_link(&track, PlayPriority::Append),
                                    format!("{}", track.track),
                                    artist_album_disc_track_link(&track),
                                    format!("{}", track.disc),
                                ]
                            })
                            .collect(),
                    );
                    trs
                }),
            ]])
        },
    )
}

pub fn artist_album_track_info(
    artist_id: model::ID,
    album_id: model::ID,
    disc_no: u64,
    track_no: u64,
    ds: Datastore,
) -> (Vec<(String, String)>, String) {
    let album_artist = ds.get_artist_from_id(artist_id.clone());
    let album = ds.get_artist_album_from_id(artist_id.clone(), album_id.clone());
    let track = ds.get_artist_album_track_name_from_id(
        artist_id.clone(),
        album_id.clone(),
        disc_no,
        track_no,
    );
    (
        vec![
            ("/artists".to_string(), "Artists".to_string()),
            (
                format!("/artists/{}/albums", artist_id.hashed()),
                album_artist.artist_name.clone(),
            ),
            (
                format!(
                    "/artists/{}/albums/{}/disc_tracks",
                    artist_id.hashed(),
                    album_id.hashed()
                ),
                album.album_name.clone(),
            ),
            (
                format!(
                    "/artists/{}/albums/{}/disc_tracks/{}/tracks/{}/info",
                    artist_id.hashed(),
                    album_id.hashed(),
                    disc_no,
                    track_no
                ),
                track.title.clone(),
            ),
        ],
        table(vec![vec![
            artist_album_image(&album_artist.artist_name, &album.album_name),
            table(vec![
                vec![
                    "".to_string(),
                    "".to_string(),
                    "".to_string(),
                    "#".to_string(),
                    "Title".to_string(),
                    "Disc".to_string(),
                    "Album".to_string(),
                    "Artist".to_string(),
                    "Album Artist".to_string(),
                ],
                vec![
                    artist_album_disc_track_play_link(&track, PlayPriority::Now),
                    artist_album_disc_track_play_link(&track, PlayPriority::Next),
                    artist_album_disc_track_play_link(&track, PlayPriority::Append),
                    format!("{}", track.track),
                    artist_album_disc_track_link(&track),
                    format!("{}", track.disc),
                    artist_album_link(&track.album_artist, &track.album),
                    track.track_artist,
                    artist_link(&track.album_artist),
                ],
            ]),
        ]]),
    )
}

pub fn albums(ds: Datastore) -> (Vec<(String, String)>, String) {
    (
        vec![("/albums".to_string(), "Albums".to_string())],
        table(
            ds.list_albums()
                .into_iter()
                .map(|(album_artist, album)| {
                    vec![
                        artist_album_image(&album_artist.artist_name, &album.album_name),
                        artist_album_link(&album_artist.artist_name, &album.album_name),
                        artist_link(&album_artist.artist_name),
                    ]
                })
                .collect(),
        ),
    )
}

pub fn tracks(ds: Datastore) -> (Vec<(String, String)>, String) {
    (
        vec![("/tracks".to_string(), "Tracks".to_string())],
        table({
            let mut trs = vec![vec![
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "#".to_string(),
                "Title".to_string(),
                "Disc".to_string(),
                "Album".to_string(),
                "Artist".to_string(),
                "Album Artist".to_string(),
            ]];
            trs.append(
                &mut ds
                    .list_tracks()
                    .into_iter()
                    .map(|track| {
                        vec![
                            artist_album_disc_track_play_link(&track, PlayPriority::Now),
                            artist_album_disc_track_play_link(&track, PlayPriority::Next),
                            artist_album_disc_track_play_link(&track, PlayPriority::Append),
                            format!("{}", track.track),
                            artist_album_disc_track_link(&track),
                            format!("{}", track.disc),
                            artist_album_link(&track.album_artist, &track.album),
                            track.track_artist,
                            artist_link(&track.album_artist),
                        ]
                    })
                    .collect(),
            );
            trs
        }),
    )
}

pub fn tree(ds: Datastore) -> (Vec<(String, String)>, String) {
    (vec![("/tree".to_string(), "Tree".to_string())], {
        table(
            ds.list_artists()
                .into_iter()
                .fold(
                    Vec::<(char, Vec<ArtistInfo>)>::new(),
                    |mut total, artist| {
                        let first_char = artist
                            .artist_name
                            .chars()
                            .next()
                            .unwrap()
                            .to_uppercase()
                            .next()
                            .unwrap();
                        match total.last_mut() {
                            Some((last_found_first_char, artists_with_last_letter)) => {
                                if *last_found_first_char == first_char {
                                    artists_with_last_letter.push(artist);
                                    return total;
                                }
                            }
                            None => (),
                        }
                        total.push((first_char, vec![artist]));
                        return total;
                    },
                )
                .into_iter()
                .map(|(first_letter, album_artists)| {
                    vec![table(vec![
                        vec![h1(format!("#{}", first_letter))],
                        vec![table(
                            album_artists
                                .into_iter()
                                .map(|album_artist| {
                                    vec![table(vec![
                                        vec![h2(artist_link(&album_artist.artist_name))],
                                        vec![table(
                                            ds.list_artist_albums(album_artist.artist_id.clone())
                                                .into_iter()
                                                .map(|album| {
                                                    let tracks = ds.list_artist_album_tracks(
                                                        album_artist.artist_id.clone(),
                                                        album.album_id.clone(),
                                                    );
                                                    vec![table(vec![
                                                        vec![table(vec![vec![
                                                            artist_album_play_link(
                                                                &tracks,
                                                                PlayPriority::Now,
                                                            ),
                                                            artist_album_play_link(
                                                                &tracks,
                                                                PlayPriority::Next,
                                                            ),
                                                            artist_album_play_link(
                                                                &tracks,
                                                                PlayPriority::Append,
                                                            ),
                                                            h3(artist_album_link(
                                                                &album_artist.artist_name,
                                                                &album.album_name,
                                                            )),
                                                        ]])],
                                                        vec![table(vec![vec![
                                                            artist_album_image(
                                                                &album_artist.artist_name,
                                                                &album.album_name,
                                                            ),
                                                            table(
                                                                tracks
                                                                    .into_iter()
                                                                    .map(|track| {
                                                                        vec![
                                                                artist_album_disc_track_play_link(
                                                                    &track,
                                                                    PlayPriority::Now,
                                                                ),
                                                                artist_album_disc_track_play_link(
                                                                    &track,
                                                                    PlayPriority::Next,
                                                                ),
                                                                artist_album_disc_track_play_link(
                                                                    &track,
                                                                    PlayPriority::Append,
                                                                ),
                                                                format!("{}", track.track),
                                                                artist_album_disc_track_link(
                                                                    &track,
                                                                ),
                                                                format!("({})", track.disc),
                                                            ]
                                                                    })
                                                                    .collect(),
                                                            ),
                                                        ]])],
                                                    ])]
                                                })
                                                .collect(),
                                        )],
                                    ])]
                                })
                                .collect(),
                        )],
                    ])]
                })
                .collect(),
        )
    })
}

pub fn old_tree(ds: Datastore) -> (Vec<(String, String)>, String) {
    (vec![("/tree".to_string(), "Tree".to_string())], {
        let mut last_seen = ' ';
        let mut seen_first_chars = Vec::new();
        let mut total = "".to_string();
        for artist_info in ds.tree().into_iter() {
            let first_char = artist_info
                .artist_info
                .artist_name
                .chars()
                .next()
                .unwrap()
                .to_uppercase()
                .next()
                .unwrap();
            if last_seen != first_char {
                total = format!(
                    "{}<h1 id=\"{}\" class=\"tree tree-alphabet\"><a href=\"#{}\">#{}</a></h1>",
                    total, first_char, first_char, first_char
                );
                last_seen = first_char;
                seen_first_chars.push(first_char);
            }

            total = format!(
                "{}<h2 class=\"tree tree-artist\">{}</h2>",
                total,
                artist_link(&artist_info.artist_info.artist_name)
            );
            for album_info in artist_info.albums.into_iter() {
                total = format!(
                    "{}<h3 class=\"tree tree-album\">{}{}{}{}</h3>{}",
                    total,
                    artist_album_play_link(&fold_discs(&album_info.discs), PlayPriority::Now),
                    artist_album_play_link(&fold_discs(&album_info.discs), PlayPriority::Next),
                    artist_album_play_link(&fold_discs(&album_info.discs), PlayPriority::Append),
                    artist_album_link(
                        &artist_info.artist_info.artist_name,
                        &album_info.album_info.album_name
                    ),
                    artist_album_image(
                        &artist_info.artist_info.artist_name,
                        &album_info.album_info.album_name
                    )
                );
                total = format!(
                        "{}<table class=\"tree tree-track\"><thead><tr><th></th><th></th><th></th><th>#</th><th>Title</th><th>📥</th></tr></thead><tbody>",
                        total
                    );
                for disc in album_info.discs.into_iter() {
                    for track_info in disc.tracks.into_iter() {
                        total = format!(
                            "{}<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                            total,
                            artist_album_disc_track_play_link(&track_info, PlayPriority::Now),
                            artist_album_disc_track_play_link(&track_info, PlayPriority::Next),
                            artist_album_disc_track_play_link(&track_info, PlayPriority::Append),
                            track_info.track,
                            artist_album_disc_track_link(&track_info),
                            artist_album_disc_track_download_link(&track_info),
                        );
                    }
                }
                total = format!("{}</tbody></table>", total);
            }
        }
        total
    })
}

fn fold_discs(
    discs: &Vec<model::SortedDiscTracks<model::FullTrackMetadata>>,
) -> Vec<model::FullTrackMetadata> {
    let mut tracks = Vec::new();
    for disc in discs.iter() {
        for track in disc.tracks.iter() {
            tracks.push(track.clone());
        }
    }
    tracks
}
