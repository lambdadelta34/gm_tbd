mod camera;

use super::debug;
use super::utils::{units_to_graphics, Settings};
use crate::GameState;
use crate::Wall;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(debug::BevyPlugin)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup_world.system())
                    .with_system(camera::setup_camera.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game).with_system(camera::camera_movement.system()),
            );
    }
}

fn setup_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    settings: Res<Settings>,
) {
    let floor = Wall::default();
    let sprite_length = units_to_graphics(floor.length, &settings);
    let sprite_width = units_to_graphics(floor.thickness, &settings);

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(floor.length / 2.0, floor.thickness / 2.0),
        flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
        ..bevy_rapier2d::prelude::ColliderBundle::default()
    };
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(sprite_length, sprite_width)),
            ..bevy::prelude::SpriteBundle::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(floor.length / 2.0, -3.).into(),
            ..bevy_rapier2d::prelude::RigidBodyBundle::default()
        })
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1));
}
