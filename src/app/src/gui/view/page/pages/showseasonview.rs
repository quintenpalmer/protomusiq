use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::gui::message::Message;

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

            for episode in season.get_episodes().values() {
                body_column = body_column.push(
                    Row::new()
                        .push(h3(format!("{}", episode.episode_sort)))
                        .push(h3(episode.local_display_name())),
                );
            }
        }
        None => body_column = body_column.push(bright_paragraph("no show path")),
    }

    let body = Container::new(
        Column::new()
            .push(h2("Show Season(s):"))
            .push(Scrollable::new(body_column).height(Length::Fill)),
    );

    body
}
