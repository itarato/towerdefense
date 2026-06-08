mod deletable;
mod dragged;
mod enemy;
mod game;
mod health;
mod math;
mod tower;
mod util;

use crate::{
    game::GamePlugin,
    util::{WIN_H, WIN_W},
};
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (WIN_W, WIN_H).into(),
                    title: "Tower Defense".into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .add_plugins(GamePlugin)
        .run();
}
