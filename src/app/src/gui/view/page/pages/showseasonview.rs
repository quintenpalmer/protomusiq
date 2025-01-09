use iced::widget::{Column, Container, Scrollable, Space};
use iced::Length;

use crate::gui::message::{ExternalSpawn, Message};

use super::super::super::elements::*;

pub fn show_season_view<'a>(
    show_library_state: &'a musiqcore::model::shows::ShowLibraryState,
    series_key: &musiqlibrary::shows::ShowKey,
    season_id: &u32,
) -> Container<'a, Message> {
    let mut body_column = Column::new();

    match show_library_state.get_shows_if_exists() {
        Some(show_library) => {
            let show = show_library
                .get_structured_shows()
                .get_show(series_key)
                .unwrap();

            body_column = body_column.push(h1(show.get_name()));

            let season = show.get_season(season_id).unwrap();

            body_column = body_column.push(h2(season.pretty_display()));

            let mut episodes = Column::new().padding(3).spacing(1);

            for episode in season.get_episodes().values() {
                let play_button =
                    Container::new(dark_button(h2(">")).on_press(Message::ExternalSpawn(
                        ExternalSpawn::Mpv(episode.full_path.clone().to_path_buf()),
                    )));

                episodes = episodes.push(
                    line_row()
                        .padding(3)
                        .push(play_button)
                        .push(Space::with_width(10))
                        .push(h3(format!("{}", episode.episode_sort)).width(Length::Fixed(40.0)))
                        .push(h3(episode.local_display_name()).width(Length::Fill)),
                );
            }

            body_column = body_column.push(episodes);
        }
        None => body_column = body_column.push(bright_paragraph("no show path")),
    }

    let body = Container::new(
        Column::new()
            .padding(10)
            .spacing(10)
            .push(h2("Show Season Episode(s):"))
            .push(Scrollable::new(body_column).height(Length::Fill)),
    );

    body
}
