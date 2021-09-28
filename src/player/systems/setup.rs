use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

struct Square;

pub fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0.0, 1.0, 0.0).into(),
        ..bevy_rapier3d::physics::RigidBodyBundle::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5),
        flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
        material: ColliderMaterial {
            restitution: 0.7,
            ..bevy_rapier3d::prelude::ColliderMaterial::default()
        },
        ..bevy_rapier3d::physics::ColliderBundle::default()
    };
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1. })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..bevy::prelude::PbrBundle::default()
        })
        .insert(Square)
        .insert_bundle(rigid_body)
        .insert_bundle(collider)
        .insert(RigidBodyPositionSync::Discrete);
}
