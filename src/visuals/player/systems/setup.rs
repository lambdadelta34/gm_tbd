use super::super::super::utils::{units_to_graphics, Settings};
use crate::Player;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    settings: Res<Settings>,
) {
    let player = Player::default();
    let sprite_width = units_to_graphics(player.width, &settings);
    let sprite_height = units_to_graphics(player.height, &settings);

    let rigid_body = RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        mass_properties: RigidBodyMassProps {
            local_mprops: MassProperties::new(Vec2::new(0.0, -1.0).into(), 8.0, 1.0).into(),
            ..RigidBodyMassProps::default()
        },
        position: player.position.into(),
        ..RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(player.width / 2.0, player.height / 2.0),
        flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
        mass_properties: ColliderMassProps::Density(1.0),
        material: ColliderMaterial {
            restitution: 0.1,
            friction: 0.1,
            ..ColliderMaterial::default()
        },
        ..ColliderBundle::default()
    };

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite::new(Vec2::new(sprite_width, sprite_height)),
            material: materials.add(Color::BLACK.into()),
            ..SpriteBundle::default()
        })
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete)
        .insert(player)
        .insert(ColliderDebugRender::with_id(1));
}
