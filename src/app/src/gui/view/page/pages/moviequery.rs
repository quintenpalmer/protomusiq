use iced::widget::{Column, Container};

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::Message;
use crate::state;

use super::super::super::elements::*;

pub fn movie_query<'a>(
    _movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieQueryState,
    _app_images: &embedded::AppImages,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    let breadcrumbs = vec![];

    let input_query_element = match state.query {
        model::MovieQueryParams::Genre(ref genre) => {
            Column::new().spacing(10).push(h2("Params:")).push(
                line_row()
                    .spacing(5)
                    .push(h3("Genre:"))
                    .push(h3(genre.clone())),
            )
        }
    };

    let body = Container::new(
        Column::new()
            .spacing(10)
            .push(h1("Movie Query"))
            .push(input_query_element),
    );

    (breadcrumbs, body)
}
