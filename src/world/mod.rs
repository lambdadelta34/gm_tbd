use crate::GameState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BevyPlugin;

impl Plugin for BevyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(setup_world.system())
                .with_system(setup_camera.system()),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 10.;
    camera.transform = Transform::from_xyz(500., 500., 500.).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn_bundle(camera);
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane { size: 20.0 })),
            material: materials.add(Color::TEAL.into()),
            ..bevy::prelude::PbrBundle::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(20.0, 0., 20.0),
            flags: (ActiveEvents::CONTACT_EVENTS | ActiveEvents::INTERSECTION_EVENTS).into(),
            ..bevy_rapier3d::physics::ColliderBundle::default()
        });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(3.0, 8.0, 5.0),
        ..bevy::prelude::LightBundle::default()
    });
}
