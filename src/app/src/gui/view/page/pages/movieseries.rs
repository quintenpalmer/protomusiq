use iced::widget::{Column, Container};

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::Message;
//use crate::state;

use super::super::super::elements::*;

pub fn movie_series<'a>(
    movie_library: &'a model::VideoLibraryState,
    _app_images: &embedded::AppImages,
) -> Container<'a, Message> {
    let mut series = Column::new();

    for (series_name, _movies) in movie_library.get_series().iter() {
        series = series.push(h3(series_name));
    }

    let body = Container::new(
        Column::new()
            .spacing(10)
            .push(h1("Movie Series"))
            .push(series),
    );

    body
}
