use std::hash::{self, Hash};

use iced_futures::futures::stream::BoxStream;
use iced_futures::subscription::Recipe;

use crate::shared;

use super::message;
use super::state;

pub fn mpris_callback(app: &state::App) -> iced::Subscription<message::Message> {
    match app {
        state::App::Loaded(loaded) => match loaded
            .rest
            .player_info
            .rest
            .mpris_callback_recv
            .replace(None)
        {
            Some(callback) => iced::Subscription::from_recipe(MprisCallbackRecipe {
                id: 0,
                player_callback: callback,
            })
            .map(message::Message::MprisCallback),
            None => {
                println!("CALLBACK:\tmpris callback recv was already consumed");
                iced::Subscription::none()
            }
        },
        _ => {
            println!("CALLBACK:\tmpris subscription not started yet, app is not loaded");
            iced::Subscription::none()
        }
    }
}

pub struct MprisCallbackRecipe<T> {
    pub id: T,
    pub player_callback: shared::Callback<shared::MprisCallbackMessage>,
}

impl<H, I, T> Recipe<H, I> for MprisCallbackRecipe<T>
where
    H: hash::Hasher,
    T: 'static + hash::Hash + Copy + Send + std::marker::Sync,
{
    type Output = shared::MprisCallbackMessage;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.id.hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        Box::pin(iced::futures::stream::unfold(
            self.player_callback,
            move |callback| async move {
                println!("CALLBACK:\tblocking on callback from mpris");
                match callback.recv() {
                    Ok(v) => {
                        println!("CALLBACK:\tmpris found a recv: {:?}", v);
                        Some((v, callback))
                    }
                    Err(e) => {
                        println!(
                            "CALLBACK:\tmpris closed subscription callback from error: {:?}",
                            e
                        );
                        None
                    }
                }
            },
        ))
    }
}

pub fn sink_callback(app: &state::App) -> iced::Subscription<message::Message> {
    match app {
        state::App::Loaded(loaded) => match loaded
            .rest
            .player_info
            .rest
            .sink_callback_recv
            .replace(None)
        {
            Some(callback) => iced::Subscription::from_recipe(SinkCallbackRecipe {
                id: 0,
                player_callback: callback,
            })
            .map(message::Message::SinkCallback),
            None => {
                println!("CALLBACK:\tsink callback recv was already consumed");
                iced::Subscription::none()
            }
        },
        _ => {
            println!("CALLBACK:\tsink subscription not started yet, app is not loaded");
            iced::Subscription::none()
        }
    }
}

pub struct SinkCallbackRecipe<T> {
    pub id: T,
    pub player_callback: shared::Callback<shared::SinkCallbackMessage>,
}

impl<H, I, T> Recipe<H, I> for SinkCallbackRecipe<T>
where
    H: hash::Hasher,
    T: 'static + hash::Hash + Copy + Send + std::marker::Sync,
{
    type Output = shared::SinkCallbackMessage;

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.id.hash(state);
    }

    fn stream(self: Box<Self>, _input: BoxStream<'static, I>) -> BoxStream<'static, Self::Output> {
        Box::pin(iced::futures::stream::unfold(
            self.player_callback,
            move |callback| async move {
                println!("CALLBACK:\tblocking on callback from sink");
                match callback.recv() {
                    Ok(v) => {
                        println!("CALLBACK:\tsink found a recv: {:?}", v);
                        Some((v, callback))
                    }
                    Err(e) => {
                        println!(
                            "CALLBACK:\tsink closed subscription callback from error: {:?}",
                            e
                        );
                        None
                    }
                }
            },
        ))
    }
}
