use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

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
        Movement,
    ));
}

fn add_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(),));
}

pub const SPEED: f32 = 5.;

#[derive(Component)]
struct Movement;

fn movement(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Movement>>) {
    let mut offset = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        offset.y += SPEED;
    }

    if keys.pressed(KeyCode::KeyS) {
        offset.y -= SPEED;
    }

    if keys.pressed(KeyCode::KeyD) {
        offset.x += SPEED;
    }

    if keys.pressed(KeyCode::KeyA) {
        offset.x -= SPEED;
    }

    for mut transform in &mut query {
        transform.translation += offset;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_player)
        .add_systems(Startup, add_camera)
        .add_systems(Update, movement)
        .run();
}
