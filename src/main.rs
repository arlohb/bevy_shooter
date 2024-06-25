mod movement;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use movement::{Movement, Velocity};

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 10. })),
            material: materials.add(Color::rgb(1., 0., 0.)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Velocity {
            velocity: Vec3::ZERO,
            drag: 0.03,
        },
        Movement { speed: 1. },
    ));
}

fn add_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(),));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(movement::plugin)
        .add_systems(Startup, add_player)
        .add_systems(Startup, add_camera)
        .run();
}
