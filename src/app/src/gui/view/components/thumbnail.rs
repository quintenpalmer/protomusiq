use std::collections::BTreeMap;

use iced::widget::{Column, Container, Row, Space};

use crate::model;

use crate::gui::message;

use super::super::elements::*;
use super::super::style;

pub enum PlaylistIconSize {
    Small,
    Large,
}

pub fn compute_playlist_thumbnail<'a>(
    library: &'a model::LibraryState,
    tracks: &Vec<musiqlibrary::TrackUniqueIdentifier>,
    icon_size: PlaylistIconSize,
) -> Container<'a, message::Message> {
    let (small_album_size, large_album_size) = match icon_size {
        PlaylistIconSize::Small => (model::AlbumSize::Micro, model::AlbumSize::Mini),
        PlaylistIconSize::Large => (model::AlbumSize::Mini, model::AlbumSize::Small),
    };

    if tracks.len() > 0 {
        let mut album_counts = BTreeMap::new();
        for track in tracks.iter() {
            *album_counts
                .entry(musiqlibrary::AlbumUniqueIdentifier::new(
                    track.artist_id,
                    track.album_id,
                ))
                .or_insert(0) += 1;
        }

        let mut counts_for_album = BTreeMap::new();
        for (key, count) in album_counts.into_iter() {
            counts_for_album
                .entry(count)
                .or_insert(Vec::new())
                .push(key)
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
                let mut row = Row::new().spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        large_album_size.clone(),
                        first_key.artist_id,
                        first_key.album_id,
                    ),
                    large_album_size.clone(),
                ));
                Column::new().spacing(3).push(row)
            }
            (Some(second_key), None, None) => {
                let mut column = Column::new().spacing(3);
                let mut row = Row::new().spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        small_album_size.clone(),
                        first_key.artist_id,
                        first_key.album_id,
                    ),
                    small_album_size.clone(),
                ));
                row = row.push(empty_album_space(small_album_size.clone()));
                column = column.push(row);

                let mut second_row = Row::new().spacing(3);
                second_row = second_row.push(empty_album_space(small_album_size.clone()));
                second_row = second_row.push(album_image(
                    library.get_album_cover(
                        small_album_size.clone(),
                        second_key.artist_id,
                        second_key.album_id,
                    ),
                    small_album_size.clone(),
                ));
                column = column.push(second_row);

                column
            }
            _ => {
                let mut column = Column::new().spacing(3);
                let mut row = Row::new().spacing(3);
                row = row.push(album_image(
                    library.get_album_cover(
                        small_album_size.clone(),
                        first_key.artist_id,
                        first_key.album_id,
                    ),
                    small_album_size.clone(),
                ));
                match second_key {
                    Some(second_key) => {
                        row = row.push(album_image(
                            library.get_album_cover(
                                small_album_size.clone(),
                                second_key.artist_id,
                                second_key.album_id,
                            ),
                            small_album_size.clone(),
                        ));

                        column = column.push(row);

                        match third_key {
                            Some(third_key) => {
                                let mut second_row = Row::new().spacing(3);
                                second_row = second_row.push(album_image(
                                    library.get_album_cover(
                                        small_album_size.clone(),
                                        third_key.artist_id,
                                        third_key.album_id,
                                    ),
                                    small_album_size.clone(),
                                ));

                                match fourth_key {
                                    Some(fourth_key) => {
                                        second_row = second_row.push(album_image(
                                            library.get_album_cover(
                                                small_album_size.clone(),
                                                fourth_key.artist_id,
                                                fourth_key.album_id,
                                            ),
                                            small_album_size.clone(),
                                        ));

                                        column = column.push(second_row);
                                    }
                                    None => {
                                        column = column.push(second_row);
                                    }
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
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .width(iced::Length::Fixed((large_album_size.width() + 10) as f32))
    .height(iced::Length::Fixed((large_album_size.height() + 10) as f32))
    .style(iced::theme::Container::Custom(Box::new(
        style::ContainerDarkInset,
    )))
}

fn empty_album_space(album_size: model::AlbumSize) -> Space {
    Space::new(
        iced::Length::Fixed(album_size.width() as f32),
        iced::Length::Fixed(album_size.height() as f32),
    )
}
