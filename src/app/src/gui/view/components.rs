use std::collections::BTreeMap;

use iced::{self, Column, Container, Length, Row};

use crate::model;

use crate::gui::message::{Message};

use super::elements::*;
use super::style;

pub fn compute_playlist_thumbnail<'a>(
    library: &'a model::LibraryState,
    tracks: &Vec<musiqlibrary::TrackUniqueIdentifier>,
) -> iced::Container<'a, Message> {
    if tracks.len() > 0 {
        let mut album_counts = BTreeMap::new();
        for track in tracks.iter() {
            *album_counts
                .entry(musiqlibrary::AlbumUniqueIdentifier::new(track.artist_id.clone(), track.album_id.clone()))
                .or_insert(0) += 1;
        }

        let mut counts_for_album = BTreeMap::new();
        for (key, count) in album_counts.into_iter() {
            counts_for_album
                .entry(count)
                .or_insert(Vec::new()).push(key)
        }

        let mut keys_sorted_by_count = Vec::new();
        for (_count, mut keys) in counts_for_album.into_iter().rev() {
            keys_sorted_by_count.append(&mut keys);
        }

        let first_key = keys_sorted_by_count[0].clone();

        let second_key = if keys_sorted_by_count.len() > 1 {
            Some(keys_sorted_by_count[1].clone())
        } else {
            None
        };

        let third_key = if keys_sorted_by_count.len() > 2 {
            Some(keys_sorted_by_count[2].clone())
        } else {
            None
        };

        let fourth_key = if keys_sorted_by_count.len() > 3 {
            Some(keys_sorted_by_count[3].clone())
        } else {
            None
        };

        let row = match (&second_key, &third_key, &fourth_key) {
            (None, None, None) => {
                let mut row = Row::new()
                    .spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Mini,
                        first_key.artist_id.clone(),
                        first_key.album_id.clone(),
                    ),
                    model::AlbumSize::Mini,
                ));
                Column::new().spacing(3).push(row)
            },
            (Some(second_key), None, None) => {
                let mut column = Column::new()
                    .spacing(3);
                let mut row = Row::new()
                    .spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Micro,
                        first_key.artist_id.clone(),
                        first_key.album_id.clone(),
                    ),
                    model::AlbumSize::Micro,
                ));
                row = row.push(empty_album_space(model::AlbumSize::Micro));
                column = column.push(row);

                let mut second_row = Row::new()
                    .spacing(3);
                second_row = second_row.push(empty_album_space(model::AlbumSize::Micro));
                second_row = second_row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Micro,
                        second_key.artist_id.clone(),
                        second_key.album_id.clone(),
                    ),
                    model::AlbumSize::Micro,
                ));
                column = column.push(second_row);

                column
            }
            _ => {
                let mut column = Column::new()
                    .spacing(3);
                let mut row = Row::new()
                    .spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        model::AlbumSize::Micro,
                        first_key.artist_id.clone(),
                        first_key.album_id.clone(),
                    ),
                    model::AlbumSize::Micro,
                ));
                match second_key {
                    Some(second_key) => {
                        row = row.push(album_image(
                            library.get_album_cover(
                                model::AlbumSize::Micro,
                                second_key.artist_id.clone(),
                                second_key.album_id.clone(),
                            ),
                            model::AlbumSize::Micro,
                        ));

                        column = column.push(row);

                        match third_key {
                            Some(third_key) => {
                            let mut second_row = Row::new()
                                .spacing(3);
                                second_row = second_row.push(album_image(
                                    library.get_album_cover(
                                        model::AlbumSize::Micro,
                                        third_key.artist_id.clone(),
                                        third_key.album_id.clone(),
                                    ),
                                    model::AlbumSize::Micro,
                                ));

                                match fourth_key {
                                    Some(fourth_key) => {
                                        second_row = second_row.push(album_image(
                                            library.get_album_cover(
                                                model::AlbumSize::Micro,
                                                fourth_key.artist_id.clone(),
                                                fourth_key.album_id.clone(),
                                            ),
                                            model::AlbumSize::Micro,
                                        ));

                                        column = column.push(second_row);
                                    }
                                    None => {
                                        column = column.push(second_row);
                                    },
                                }
                            }
                            None => (),
                        }
                    }
                    None => (),
                }
                column
            }
        };

        Container::new(row)
    } else {
        Container::new(bright_paragraph("<?>"))
    }
    .align_x(iced::Align::Center)
    .align_y(iced::Align::Center)
    .width(Length::Units(80))
    .height(Length::Units(80))
    .style(style::ContainerDarkInset)
}

fn empty_album_space(album_size: model::AlbumSize) -> iced::Space {
    iced::Space::new(
        iced::Length::Units(album_size.width()),
        iced::Length::Units(album_size.height()),
    )
}
