use iced::widget::{button, container};
use iced::{Background, Color};

use crate::model;

pub struct DarkTextLikeButton {}

impl button::StyleSheet for DarkTextLikeButton {
    type Style = iced::Theme;

    fn active(&self, _theme: &iced::Theme) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x30, 0x30, 0x30))),
            border: iced::Border::with_radius(0.0),
            //shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xd8, 0xd8, 0xd8),
            ..button::Appearance::default()
        }
    }
}

pub struct DarkButton {}

impl button::StyleSheet for DarkButton {
    type Style = iced::Theme;

    fn active(&self, _theme: &iced::Theme) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x30, 0x30, 0x30))),
            border: iced::Border {
                width: 1.0,
                radius: 1.0.into(),
                ..iced::Border::default()
            },
            //shadow_offset: Vector::new(1.0, 1.0),
            text_color: Color::from_rgb8(0xd8, 0xd8, 0xd8),
            ..button::Appearance::default()
        }
    }
}

pub fn get_stripe_style(
    stripe_marker: bool,
) -> Box<dyn container::StyleSheet<Style = iced::Theme>> {
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
) -> Box<dyn container::StyleSheet<Style = iced::Theme>> {
    let is_playing = match maybe_currently_playing {
        Some(currently_playing) => currently_playing == display_track,
        None => false,
    };
    let is_selected = match maybe_selected {
        Some(selected) => {
            &musiqlibrary::TrackUniqueIdentifier::from_track(&display_track.metadata) == selected
        }
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
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x20, 0x20, 0x20))),
            //text_color: Some(Color::WHITE),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerStripeTwo;

impl container::StyleSheet for ContainerStripeTwo {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x28, 0x28, 0x28))),
            //text_color: Some(Color::WHITE),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerStripeHighlight;

impl container::StyleSheet for ContainerStripeHighlight {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x01, 0x7f, 0x82))),
            text_color: Some(Color::from_rgb8(0x06, 0xda, 0xdd)),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerStripeSelected;

impl container::StyleSheet for ContainerStripeSelected {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x01, 0x7f, 0xc2))),
            text_color: Some(Color::from_rgb8(0x06, 0xda, 0xdd)),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerStripeHighlightAndSelected;

impl container::StyleSheet for ContainerStripeHighlightAndSelected {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x01, 0x7f, 0xa2))),
            text_color: Some(Color::from_rgb8(0x06, 0xda, 0xdd)),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerPlaybackPlayedThrough;

impl container::StyleSheet for ContainerPlaybackPlayedThrough {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x00, 0xd8, 0xd8))),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerPlaybackToPlayThrough;

impl container::StyleSheet for ContainerPlaybackToPlayThrough {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x10, 0x10, 0x10))),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerPopForward;

impl container::StyleSheet for ContainerPopForward {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x20, 0x20, 0x20))),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerPopMidForward;

impl container::StyleSheet for ContainerPopMidForward {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x18, 0x18, 0x18))),
            ..container::Appearance::default()
        }
    }
}

pub struct ContainerDarkInset;

impl container::StyleSheet for ContainerDarkInset {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x13, 0x13, 0x13))),
            ..container::Appearance::default()
        }
    }
}

pub struct NotificationGreyBackground;

impl container::StyleSheet for NotificationGreyBackground {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x13, 0x13, 0x13))),
            text_color: Some(Color::from_rgb8(0x30, 0x30, 0x30)),
            border: iced::Border {
                width: 1.0,
                radius: 1.0.into(),
                color: iced::Color::from_rgb8(0x30, 0x30, 0x30),
            },
            ..container::Appearance::default()
        }
    }
}

pub struct NotificationGreenBackground;

impl container::StyleSheet for NotificationGreenBackground {
    type Style = iced::Theme;

    fn appearance(&self, _theme: &iced::Theme) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(0x13, 0x53, 0x13))),
            text_color: Some(Color::from_rgb8(0x30, 0xa0, 0x30)),
            border: iced::Border {
                width: 1.0,
                radius: 1.0.into(),
                color: Color::from_rgb8(0x30, 0xa0, 0x30),
            },
            ..container::Appearance::default()
        }
    }
}
