use super::elements::*;

pub fn main_html_wrapper<K: ToString, V: ToString, B: ToString>(
    crumbs: Vec<(K, V)>,
    contents: B,
) -> String {
    let mut total_crumbs = vec![("/".to_string(), "Home".to_string())];
    total_crumbs.append(
        &mut crumbs
            .into_iter()
            .map(|(u, d)| (u.to_string(), d.to_string()))
            .collect(),
    );
    html(
        "en-US",
        vec![
            head(vec![
                meta("utf-8"),
                link("stylesheet", "/static/stylesheet.css"),
                script(Some("/static/main.js"), Vec::<String>::new()),
            ]),
            body(vec![
                div(
                    vec![("id", "topbar")],
                    vec![div(NOATTRS, vec![breadcrumbs(total_crumbs)])],
                ),
                div(
                    vec![("id", "playlist-content-container")],
                    vec![
                        div(
                            vec![("id", "main-content")],
                            vec![span(NOATTRS, vec![contents.to_string()])],
                        ),
                        div(
                            vec![("id", "playlist")],
                            vec![span(NOATTRS, vec![playlist()])],
                        ),
                    ],
                ),
                div(vec![("id", "bottombar")], vec![playback_controls()]),
            ]),
        ],
    )
}

pub fn breadcrumbs<S: ToString>(crumbs: Vec<(S, S)>) -> String {
    crumbs
        .into_iter()
        .map(|(path, name)| a(path.to_string(), name.to_string()))
        .collect::<Vec<String>>()
        .join(" &gt ")
}

fn playback_controls() -> String {
    span(
        NOATTRS,
        vec![table(vec![vec![
            audio(None::<String>, vec![("id", "main-playback")]),
            a("javascript:{ play_prev_song_in_history(); }", "&lt&lt"),
            a("javascript:{ play_next_song_in_queue(); }", "&gt&gt"),
            div(vec![("id", "main-playback-info")], Vec::<String>::new()),
        ]])],
    )
}

fn playlist() -> String {
    div(NOATTRS, vec!["The Playlist"])
}
