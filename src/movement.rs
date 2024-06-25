use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    velocity: Vec3,
    drag: f32,
}

impl Velocity {
    pub fn new(drag: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,
            drag,
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.velocity;
    }
}

fn reduce_velocity(mut query: Query<&mut Velocity>) {
    for mut velocity in &mut query {
        let drag = velocity.drag;
        velocity.velocity *= 1. - drag;
    }
}

pub const SPEED: f32 = 1.;

#[derive(Component)]
pub struct Movement;

fn movement(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Movement>>) {
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

    for mut velocity in &mut query {
        velocity.velocity += offset;
    }
}

fn wrap_movement(
    windows: Query<&Window>,
    projections: Query<&OrthographicProjection>,
    mut transforms: Query<&mut Transform, With<Movement>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };
    let Ok(projection) = projections.get_single() else {
        return;
    };

    // TODO: Isn't using projection.scaling_mode
    let width = window.width() * projection.scale;
    let height = window.height() * projection.scale;
    let max_x = width / 2.;
    let min_x = -max_x;
    let max_y = height / 2.;
    let min_y = -max_y;

    for mut transform in &mut transforms {
        let translation = &mut transform.translation;

        if translation.x < min_x {
            translation.x = max_x;
        }

        if translation.x > max_x {
            translation.x = min_x;
        }

        if translation.y < min_y {
            translation.y = max_y;
        }

        if translation.y > max_y {
            translation.y = min_y;
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (movement, apply_velocity, (reduce_velocity, wrap_movement)).chain(),
    );
}
