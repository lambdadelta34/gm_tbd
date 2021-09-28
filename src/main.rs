#![allow(unused)]
#![warn(clippy::all, clippy::pedantic)]

mod debug;
mod main_menu;
mod player;
mod world;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::keyboard::KeyboardInput,
    prelude::*,
};
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Game,
    MainMenu,
    OptionsMenu,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Gamey".to_string(),
            // width: 1920.,
            // height: 1080.,
            ..bevy::prelude::WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(debug::BevyPlugin)
        .add_plugin(player::BevyPlugin)
        .add_plugin(world::BevyPlugin)
        .add_plugin(main_menu::BevyPlugin)
        .add_state(GameState::MainMenu)
        .run();
}
