use iced::widget::{Column, Container, Scrollable};

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::elements::*;

pub fn movie_attributes<'a>(
    state: &'a state::MovieAttributeState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::MovieAttributeState {
            attribute,
            attribute_results,
        } => {
            let breadcrumbs = vec![(
                "Movies".to_string(),
                user_nav_message(NavMessage::MovieList(
                    0,
                    model::MovieSortKey::ByTitle,
                    model::SortOrder::Regular,
                )),
            )];

            let (attribute_name, attribute_table) = match attribute {
                model::MovieAttribute::Genres => {
                    let mut table = Column::new().spacing(10);
                    for result in attribute_results.into_iter() {
                        table = table.push(h3(result));
                    }
                    (h2("Genres:"), Scrollable::new(table))
                }
            };

            let body = Container::new(
                Column::new()
                    .spacing(10)
                    .push(h1("Movie Attributes"))
                    .push(attribute_name)
                    .push(attribute_table),
            );

            (breadcrumbs, body)
        }
    }
}
