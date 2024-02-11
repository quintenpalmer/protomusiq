use iced::widget::Container;

use crate::model;

use crate::datastore::staticassets::embedded;
use crate::gui::message::Message;
use crate::state;

use super::super::super::elements::*;

pub fn movie_query<'a>(
    _movie_library: &'a model::VideoLibraryState,
    _state: &'a state::MovieQueryState,
    _app_images: &embedded::AppImages,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    (vec![], Container::new(h1("Movie Query")))
}
