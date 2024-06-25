use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    movement::{Velocity, WrapMovement},
    Player, TimeToLive,
};

#[derive(Event)]
struct Fire {
    pos: Vec2,
    dir: Vec2,
    speed: f32,
}

#[derive(Component)]
struct Target;

fn create_target(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    commands.spawn((
        Target,
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..Default::default()
            },
            texture: asset_server.load("target.png"),
            transform: Transform::from_scale(Vec3::new(3., 3., 1.)),
            ..Default::default()
        },
    ));

    windows.single_mut().cursor.visible = false;
}

fn update_target(
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<&mut Transform, With<Target>>,
) {
    let (camera, camera_transform) = cameras.single();
    let Some(target_pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor_pos| camera.viewport_to_world_2d(camera_transform, cursor_pos))
    else {
        return;
    };

    for mut transform in &mut query {
        transform.translation.x = target_pos.x;
        transform.translation.y = target_pos.y;
    }
}

#[derive(Resource, Default)]
struct BulletAssets {
    mesh: Option<Handle<Mesh>>,
    material: Option<Handle<ColorMaterial>>,
}

impl BulletAssets {
    pub fn get_or_load_mesh(&mut self, mut meshes: ResMut<Assets<Mesh>>) -> Handle<Mesh> {
        self.mesh
            .get_or_insert_with(|| meshes.add(Circle { radius: 5. }.mesh()))
            .clone()
    }

    pub fn get_or_load_material(
        &mut self,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Handle<ColorMaterial> {
        self.material
            .get_or_insert_with(|| materials.add(Color::rgb(1., 0., 0.)))
            .clone()
    }
}

fn create_bullets(
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut assets: ResMut<BulletAssets>,
    mut commands: Commands,
    mut event_fire: EventReader<Fire>,
) {
    let mesh = assets.get_or_load_mesh(meshes);
    let material = assets.get_or_load_material(materials);

    for &Fire { pos, dir, speed } in event_fire.read() {
        commands.spawn((
            Velocity {
                velocity: dir * speed,
                drag: 0.,
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(mesh.clone()),
                material: material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..Default::default()
            },
            TimeToLive::new(Duration::from_secs(2)),
            WrapMovement,
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
            speed: 1000.,
        });
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<Fire>()
        .insert_resource(BulletAssets::default())
        .add_systems(Startup, create_target)
        .add_systems(Update, update_target)
        .add_systems(Update, (player_shoot, create_bullets).chain());
}
