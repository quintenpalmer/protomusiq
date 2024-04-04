use iced::widget::{Column, Container, Scrollable};

use crate::model;

use crate::gui::message::{self, Message};
use crate::state;

use super::super::super::elements::*;

pub fn artist_info_view_state<'a>(
    library: &'a model::LibraryState,
    state: &'a state::ArtistInfoState,
) -> Container<'a, Message> {
    match state {
        state::ArtistInfoState { artist_id } => {
            let artist = library.get_artist_info(*artist_id);

            let artist_view_button_row =
                line_row()
                    .push(dark_button(dark(h2("Albums"))).on_press(
                        message::ArtistViewType::ArtistAlbumsView.into_message(*artist_id),
                    ))
                    .push(
                        dark_button(dark(h2("Tracks"))).on_press(
                            message::ArtistViewType::ArtistTrackView(
                                model::ArtistTrackSortKey::ByTotalPlayCount,
                                model::ArtistTrackSortKey::ByTotalPlayCount.default_order(),
                            )
                            .into_message(*artist_id),
                        ),
                    )
                    .push(
                        dark_button(dark(h2("Featured"))).on_press(
                            message::ArtistViewType::ArtistFeaturedTrackView(
                                model::ArtistFeaturedTrackSortKey::ByTotalPlayCount,
                                model::ArtistFeaturedTrackSortKey::ByTotalPlayCount.default_order(),
                            )
                            .into_message(*artist_id),
                        ),
                    )
                    .push(
                        dark_button(dark(h2("In Playlists")))
                            .on_press(message::ArtistViewType::InPlaylist.into_message(*artist_id)),
                    )
                    .push(
                        dark_button(h2("Info"))
                            .on_press(message::ArtistViewType::ArtistInfo.into_message(*artist_id)),
                    );

            let maybe_raw_info = library.musicbrainz_library.get_artist_info(artist_id);

            let display_info = match maybe_raw_info {
                Some(raw_info) => {
                    let mut ret = Column::new()
                        .spacing(8)
                        .push(
                            line_row()
                                .spacing(6)
                                .push(h3("External Name:"))
                                .push(h2(raw_info.name.clone())),
                        )
                        .push(
                            line_row()
                                .spacing(6)
                                .push(h3("Sort Name:"))
                                .push(h2(raw_info.sort_name.clone())),
                        );

                    match raw_info.isnis {
                        Some(ref i) => {
                            ret = ret.push(
                                line_row()
                                    .spacing(6)
                                    .push(h3("ISNIs:"))
                                    .push(h2(i.join(", "))),
                            );
                        }
                        None => {
                            ret = ret.push(line_row().spacing(6).push("no ISNI(s)"));
                        }
                    };

                    match raw_info.tags {
                        Some(ref tags) => {
                            ret = ret.push(
                                dark_button(h3("Tags:"))
                                    .on_press(message::MusicGenreNavMessage::Home.into_message()),
                            );
                            let mut row = Column::new();

                            let mut sorted = tags.iter().cloned().collect::<Vec<_>>();
                            sorted.sort_by(|a, b| b.count.cmp(&a.count));

                            for tag in sorted.into_iter() {
                                row = row.push(
                                    line_row()
                                        .spacing(6)
                                        .push(h2(tag.name))
                                        .push(h3(format!("({})", tag.count))),
                                );
                            }
                            ret = ret.push(Scrollable::new(row));
                        }
                        None => {
                            ret = ret.push(line_row().spacing(6).push("No Tags"));
                        }
                    }

                    ret
                }
                None => Column::new().push(h2("No MusicBrainz Info found")),
            };

            let body = Container::new(
                Column::new()
                    .push(h1(artist.artist_name.clone()))
                    .push(artist_view_button_row)
                    .push(display_info),
            );

            body
        }
    }
}
