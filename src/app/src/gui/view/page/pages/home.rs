use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::{self, user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::common;
use super::super::super::consts;
use super::super::super::elements::*;

pub fn home_page<'a>(
    app_images: &embedded::AppImages,
    state: &'a state::HomeState,
) -> Container<'a, Message> {
    match state {
        state::HomeState {} => {
            let music_list = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_albums_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Music".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(
                message::MusicNavMessage::AlbumList(
                    0,
                    model::AlbumSortKey::preferred_home(),
                    model::AlbumSortKey::preferred_home().default_order(),
                )
                .into_message(),
            );
            let search = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_search_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Search".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(user_nav_message(NavMessage::SearchPage(
                "".to_string(),
                model::SearchDomain::Music,
                false,
            )));
            let settings = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_settings_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Settings".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(user_nav_message(NavMessage::Config));

            let dvd = dark_button(Container::new(bottom_label(
                album_image(app_images.get_dvd_image().clone(), model::AlbumSize::Small).into(),
                bright_paragraph(common::abr_str(
                    "Movies".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(
                message::MovieNavMessage::MovieList(
                    0,
                    model::MovieSortKey::ByTitle,
                    model::MovieSortKey::ByTitle.default_order(),
                )
                .into_message(),
            );

            let game_home = dark_button(Container::new(bottom_label(
                album_image(
                    app_images.get_game_controller_image().clone(),
                    model::AlbumSize::Small,
                )
                .into(),
                bright_paragraph(common::abr_str(
                    "Games".to_string(),
                    consts::ICON_STR_LENGTH,
                )),
            )))
            .on_press(message::GameNavMessage::GameHome.into_message());

            let page = Container::new(
                Scrollable::new(
                    Column::new()
                        .width(Length::Fill)
                        .push(h1("Home"))
                        .push(Row::new().push(music_list).push(dvd).push(game_home))
                        .push(Row::new().push(search).push(settings)),
                )
                .height(Length::Fill),
            );

            page
        }
    }
}
