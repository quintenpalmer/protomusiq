use iced::widget::Container;

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::elements::*;

pub fn movie_list<'a>(
    _movie_library: &'a model::VideoLibraryState,
    state: &'a state::MovieListState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::MovieListState {
            page: _page,
            sort_key,
            sort_order,
        } => {
            let breadcrumbs = vec![(
                "Tracks".to_string(),
                user_nav_message(NavMessage::MovieList(
                    0,
                    sort_key.clone(),
                    sort_order.clone(),
                )),
            )];
            let body = Container::new(h1("Movies"));
            (breadcrumbs, body)
        }
    }
}
