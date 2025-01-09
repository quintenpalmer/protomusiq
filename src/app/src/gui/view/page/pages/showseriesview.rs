use iced::widget::{Column, Container, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, Message};

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn show_series_view<'a>(
    show_library_state: &'a musiqcore::model::shows::ShowLibraryState,
    app_images: &embedded::AppImages,
    series_key: &musiqlibrary::shows::ShowKey,
) -> Container<'a, Message> {
    let mut body_column = Column::new();

    match show_library_state.get_shows_if_exists() {
        Some(show_library) => {
            let show = show_library
                .get_structured_shows()
                .get_show(series_key)
                .unwrap();

            body_column = body_column.push(h1(show.get_name()));

            for season in show.get_seasons().values() {
                body_column = body_column.push(
                    dark_button(Container::new(bottom_label(
                        album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Small)
                            .into(),
                        bright_paragraph(common::abr_str(
                            season.pretty_display(),
                            consts::ICON_STR_LENGTH,
                        )),
                    )))
                    .on_press(
                        message::ShowNavMessage::ShowSeason(
                            series_key.clone(),
                            season.get_season_number(),
                        )
                        .into_message(),
                    ),
                );
            }
        }
        None => body_column = body_column.push(bright_paragraph("no show path")),
    }

    let body = Container::new(
        Column::new()
            .padding(10)
            .spacing(10)
            .push(h2("Show Season(s):"))
            .push(Scrollable::new(body_column).height(Length::Fill)),
    );

    body
}
