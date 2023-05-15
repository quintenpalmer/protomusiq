use iced::{button, container, Background, Color};

use musiqlibrary;

use crate::model;

pub struct DarkTextLikeButton {}

impl button::StyleSheet for DarkTextLikeButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0x30, 0x30, 0x30))),
            border_radius: 0.0,
            border_width: 0.0,
            //shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xd8, 0xd8, 0xd8),
            ..button::Style::default()
        }
    }
}

pub struct DarkButton {}

impl button::StyleSheet for DarkButton {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(Color::from_rgb8(0x30, 0x30, 0x30))),
            border_radius: 1.0,
            border_width: 1.0,
            //shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xd8, 0xd8, 0xd8),
            ..button::Style::default()
        }
    }
}

pub fn get_stripe_style(stripe_marker: bool) -> Box<dyn container::StyleSheet> {
    if stripe_marker {
        Box::new(ContainerStripeOne)
    } else {
        Box::new(ContainerStripeTwo)
    }
}

pub fn get_potential_current_stripe_style(
    stripe_marker: bool,
    display_track: &model::AugmentedTrack,
    maybe_currently_playing: &Option<model::AugmentedTrack>,
    maybe_selected: &Option<musiqlibrary::TrackUniqueIdentifier>,
) -> Box<dyn container::StyleSheet> {
    let is_playing = match maybe_currently_playing {
        Some(currently_playing) => currently_playing == display_track,
        None => false,
    };
    let is_selected = match maybe_selected {
        Some(selected) => &musiqlibrary::TrackUniqueIdentifier::from_track(&display_track.metadata) == selected,
        None => false,
    };

    if is_playing {
        if is_selected {
            Box::new(ContainerStripeHighlightAndSelected)
        } else {
            Box::new(ContainerStripeHighlight)
        }
    } else {
        if is_selected {
            Box::new(ContainerStripeSelected)
        } else {
            get_stripe_style(stripe_marker)
        }
    }
}

pub struct ContainerStripeOne;

impl container::StyleSheet for ContainerStripeOne {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x20, 0x20, 0x20))),
            //text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct ContainerStripeTwo;

impl container::StyleSheet for ContainerStripeTwo {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x28, 0x28, 0x28))),
            //text_color: Some(Color::WHITE),
            ..container::Style::default()
        }
    }
}

pub struct ContainerStripeHighlight;

impl container::StyleSheet for ContainerStripeHighlight {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x01, 0x7f, 0x82))),
            text_color: Some(Color::from_rgb8(0x06, 0xda, 0xdd)),
            ..container::Style::default()
        }
    }
}

pub struct ContainerStripeSelected;

impl container::StyleSheet for ContainerStripeSelected {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x01, 0x7f, 0xc2))),
            text_color: Some(Color::from_rgb8(0x06, 0xda, 0xdd)),
            ..container::Style::default()
        }
    }
}

pub struct ContainerStripeHighlightAndSelected;

impl container::StyleSheet for ContainerStripeHighlightAndSelected {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x01, 0x7f, 0xa2))),
            text_color: Some(Color::from_rgb8(0x06, 0xda, 0xdd)),
            ..container::Style::default()
        }
    }
}

pub struct ContainerPlaybackPlayedThrough;

impl container::StyleSheet for ContainerPlaybackPlayedThrough {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x00, 0xd8, 0xd8))),
            ..container::Style::default()
        }
    }
}

pub struct ContainerPlaybackToPlayThrough;

impl container::StyleSheet for ContainerPlaybackToPlayThrough {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x10, 0x10, 0x10))),
            ..container::Style::default()
        }
    }
}

pub struct ContainerPopForward;

impl container::StyleSheet for ContainerPopForward {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x20, 0x20, 0x20))),
            ..container::Style::default()
        }
    }
}

pub struct ContainerPopMidForward;

impl container::StyleSheet for ContainerPopMidForward {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x18, 0x18, 0x18))),
            ..container::Style::default()
        }
    }
}

pub struct ContainerDarkInset;

impl container::StyleSheet for ContainerDarkInset {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(Color::from_rgb8(0x13, 0x13, 0x13))),
            ..container::Style::default()
        }
    }
}
