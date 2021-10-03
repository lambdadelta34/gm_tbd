#![allow(unused)]
#![warn(clippy::all)]

mod core;
mod main_menu;
mod visuals;

use crate::core::prelude::*;
use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use visuals::BevyPlugin;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Gamey".to_string(),
            // width: 1920.,
            // height: 1080.,
            resizable: false,
            ..bevy::prelude::WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(core::BevyPlugin)
        .add_plugin(visuals::BevyPlugin)
        .add_plugin(main_menu::BevyPlugin)
        .add_state(GameState::MainMenu)
        .add_system(quit.system())
        .run();
}

fn quit(
    mut app_exit_events: EventWriter<AppExit>,
    mut app_state: ResMut<State<GameState>>,
    input: Res<Input<KeyCode>>,
) {
    let cmd = input.pressed(KeyCode::LWin) || input.pressed(KeyCode::RWin);

    if cmd && input.just_pressed(KeyCode::Q) {
        app_exit_events.send(AppExit);
        info!("Bye");
    }
}
