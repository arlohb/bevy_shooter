use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{movement::Velocity, Player, TimeToLive};

#[derive(Event)]
struct Fire {
    pos: Vec2,
    dir: Vec2,
    speed: f32,
}

#[derive(Resource, Default)]
struct BulletAssets {
    mesh: Option<Handle<Mesh>>,
    material: Option<Handle<ColorMaterial>>,
}

fn create_bullets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut assets: ResMut<BulletAssets>,
    mut commands: Commands,
    mut event_fire: EventReader<Fire>,
) {
    let mesh = assets
        .mesh
        .get_or_insert_with(|| meshes.add(Circle { radius: 5. }.mesh()))
        .clone();

    let material = assets
        .material
        .get_or_insert_with(|| materials.add(Color::rgb(1., 0., 0.)));

    for &Fire { pos, dir, speed } in event_fire.read() {
        commands.spawn((
            Velocity {
                velocity: Vec3::new(dir.x, dir.y, 0.) * speed,
                drag: 0.,
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh.clone()),
                material: material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..Default::default()
            },
            TimeToLive::new(Duration::from_secs(5)),
        ));
    }
}

fn player_shoot(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut event_fire: EventWriter<Fire>,
    mouse_btns: Res<ButtonInput<MouseButton>>,
    players: Query<&Transform, With<Player>>,
) {
    let (camera, camera_transform) = cameras.single();
    let Some(target) = windows
        .single()
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_transform, cursor_pos))
    else {
        return;
    };

    let player = players.single();
    let pos = Vec2::new(player.translation.x, player.translation.y);

    if mouse_btns.just_pressed(MouseButton::Left) {
        event_fire.send(Fire {
            pos,
            dir: (target - pos).normalize(),
            speed: 5.,
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<Fire>()
        .insert_resource(BulletAssets::default())
        .add_systems(Update, (player_shoot, create_bullets).chain());
}
