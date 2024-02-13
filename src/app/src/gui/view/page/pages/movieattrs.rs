use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message::{user_nav_message, Message, NavMessage};
use crate::state;

use super::super::super::elements::*;

pub fn movie_attributes<'a>(
    state: &'a state::MovieAttributeState,
) -> (Vec<(String, Message)>, Container<'a, Message>) {
    match state {
        state::MovieAttributeState { attribute_results } => {
            let breadcrumbs = vec![(
                "Movies".to_string(),
                user_nav_message(NavMessage::MovieList(
                    0,
                    model::MovieSortKey::ByTitle,
                    model::SortOrder::Regular,
                )),
            )];

            let (attribute_name, attribute_table) = match attribute_results {
                model::AttributesList::Genre(attribute_results) => {
                    let mut table = Column::new().spacing(10);

                    let mut genre_row = Row::new().spacing(10);
                    let mut genre_row_count = 0;
                    for (index, result) in attribute_results.into_iter().enumerate() {
                        genre_row = genre_row.push(dark_button(h3(result.clone())).on_press(
                            user_nav_message(NavMessage::MovieQuery(
                                model::MovieQueryParams::Genre(result.clone()),
                            )),
                        ));
                        genre_row_count += 1;

                        if index % 5 == 4 {
                            table = table.push(genre_row);
                            genre_row = Row::new().spacing(10);
                            genre_row_count = 0;
                        }
                    }

                    if genre_row_count != 0 {
                        table = table.push(genre_row);
                    }

                    (h2("Genres:"), Scrollable::new(table.width(Length::Fill)))
                }
            };

            let body = Container::new(
                Column::new().spacing(10).push(h1("Movie Attributes")).push(
                    Column::new()
                        .padding(20)
                        .spacing(10)
                        .push(attribute_name)
                        .push(attribute_table),
                ),
            );

            (breadcrumbs, body)
        }
    }
}
