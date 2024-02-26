use iced::widget::{Column, Container, Row, Scrollable};
use iced::Length;

use crate::model;

use crate::gui::message::{self, Message};
use crate::state;

use super::super::super::elements::*;

pub fn movie_attributes(
    state: &state::MovieAttributeState,
) -> (Vec<(String, Message)>, Container<Message>) {
    match state {
        state::MovieAttributeState {
            attribute_results: maybe_attribute_results,
        } => {
            let breadcrumbs = vec![
                (
                    "Movie".to_string(),
                    message::MovieNavMessage::MovieHome.into_message(),
                ),
                (
                    "Movie Attributes".to_string(),
                    message::MovieNavMessage::MovieAttributes(None).into_message(),
                ),
            ];

            let (attribute_name, attribute_table) = match maybe_attribute_results {
                Some(attribute_results) => match attribute_results {
                    model::AttributesList::Genre(attribute_results) => {
                        let mut table = Column::new().spacing(10);

                        let mut genre_row = Row::new().spacing(10);
                        let mut genre_row_count = 0;
                        for (index, result) in attribute_results.iter().enumerate() {
                            genre_row = genre_row.push(
                                dark_button(h3(result.clone())).on_press(
                                    message::MovieNavMessage::MovieQuery(Some(
                                        model::MovieQueryParams::Genre(result.clone()),
                                    ))
                                    .into_message(),
                                ),
                            );
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

                        (
                            h2("Genres:"),
                            Scrollable::new(table.width(Length::Fill)).height(Length::Fill),
                        )
                    }
                    model::AttributesList::Production(attribute_results) => {
                        let mut table = Column::new().spacing(10);

                        let mut production_row = Row::new().spacing(10);
                        let mut production_row_count = 0;
                        for (index, result) in attribute_results.iter().enumerate() {
                            production_row = production_row.push(
                                dark_button(h3(result.clone())).on_press(
                                    message::MovieNavMessage::MovieQuery(Some(
                                        model::MovieQueryParams::Production(result.clone()),
                                    ))
                                    .into_message(),
                                ),
                            );
                            production_row_count += 1;

                            if index % 2 == 1 {
                                table = table.push(production_row);
                                production_row = Row::new().spacing(10);
                                production_row_count = 0;
                            }
                        }

                        if production_row_count != 0 {
                            table = table.push(production_row);
                        }

                        (
                            h2("Production Companies:"),
                            Scrollable::new(table.width(Length::Fill)),
                        )
                    }

                    model::AttributesList::Producers(attribute_results) => {
                        let mut table = Column::new().spacing(10);

                        let mut producers_row = Row::new().spacing(10);
                        let mut producers_row_count = 0;
                        for (index, result) in attribute_results.iter().enumerate() {
                            producers_row = producers_row.push(
                                dark_button(h3(result.clone())).on_press(
                                    message::MovieNavMessage::MovieQuery(Some(
                                        model::MovieQueryParams::Producers(result.clone()),
                                    ))
                                    .into_message(),
                                ),
                            );
                            producers_row_count += 1;

                            if index % 2 == 1 {
                                table = table.push(producers_row);
                                producers_row = Row::new().spacing(10);
                                producers_row_count = 0;
                            }
                        }

                        if producers_row_count != 0 {
                            table = table.push(producers_row);
                        }

                        (h2("Producers:"), Scrollable::new(table.width(Length::Fill)))
                    }
                    model::AttributesList::Director(attribute_results) => {
                        let mut table = Column::new().spacing(10);

                        let mut result_row = Row::new().spacing(10);
                        let mut result_row_count = 0;
                        for (index, result) in attribute_results.iter().enumerate() {
                            result_row = result_row.push(
                                dark_button(h3(result.clone())).on_press(
                                    message::MovieNavMessage::MovieQuery(Some(
                                        model::MovieQueryParams::Director(result.clone()),
                                    ))
                                    .into_message(),
                                ),
                            );
                            result_row_count += 1;

                            if index % 3 == 2 {
                                table = table.push(result_row);
                                result_row = Row::new().spacing(10);
                                result_row_count = 0;
                            }
                        }

                        if result_row_count != 0 {
                            table = table.push(result_row);
                        }

                        (h2("Directors:"), Scrollable::new(table.width(Length::Fill)))
                    }
                    model::AttributesList::Screenplay(attribute_results) => {
                        let mut table = Column::new().spacing(10);

                        let mut result_row = Row::new().spacing(10);
                        let mut result_row_count = 0;
                        for (index, result) in attribute_results.iter().enumerate() {
                            result_row = result_row.push(
                                dark_button(h3(result.clone())).on_press(
                                    message::MovieNavMessage::MovieQuery(Some(
                                        model::MovieQueryParams::Screenplay(result.clone()),
                                    ))
                                    .into_message(),
                                ),
                            );
                            result_row_count += 1;

                            if index % 3 == 2 {
                                table = table.push(result_row);
                                result_row = Row::new().spacing(10);
                                result_row_count = 0;
                            }
                        }

                        if result_row_count != 0 {
                            table = table.push(result_row);
                        }

                        (
                            h2("Screenplay Writers:"),
                            Scrollable::new(table.width(Length::Fill)),
                        )
                    }
                    model::AttributesList::CastMember(attribute_results) => {
                        let mut table = Column::new().spacing(10);

                        let mut result_row = Row::new().spacing(10);
                        let mut result_row_count = 0;
                        for (index, result) in attribute_results.iter().enumerate() {
                            result_row = result_row.push(
                                dark_button(h3(result.clone())).on_press(
                                    message::MovieNavMessage::MovieQuery(Some(
                                        model::MovieQueryParams::CastMember(result.clone()),
                                    ))
                                    .into_message(),
                                ),
                            );
                            result_row_count += 1;

                            if index % 3 == 2 {
                                table = table.push(result_row);
                                result_row = Row::new().spacing(10);
                                result_row_count = 0;
                            }
                        }

                        if result_row_count != 0 {
                            table = table.push(result_row);
                        }

                        (
                            h2("Cast Member:"),
                            Scrollable::new(table.width(Length::Fill)),
                        )
                    }
                },
                None => {
                    let attribute_list = Column::new()
                        .spacing(10)
                        .padding(10)
                        .push(
                            dark_button(h2("Genres")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Genres,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(
                            dark_button(h2("Production Companies")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Production,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(
                            dark_button(h2("Producers")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Producers,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(
                            dark_button(h2("Directors")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Directors,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(
                            dark_button(h2("Screenplay Writers")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::Screenplay,
                                ))
                                .into_message(),
                            ),
                        )
                        .push(
                            dark_button(h2("Cast Members")).on_press(
                                message::MovieNavMessage::MovieAttributes(Some(
                                    model::MovieAttribute::CastMembers,
                                ))
                                .into_message(),
                            ),
                        );
                    (h2("Select Attribute"), Scrollable::new(attribute_list))
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
