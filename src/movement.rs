use crate::Player;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}

fn check_keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>) -> Vec3 {
    let mut movement_vector = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::KeyA) {
        movement_vector.x -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        movement_vector.x += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        movement_vector.y += 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        movement_vector.y -= 1.0;
    }

    return movement_vector;
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let movement_vector = check_keyboard_input(keyboard_input);

    if movement_vector != Vec3::ZERO {
        for mut transform in &mut query {
            transform.translation += movement_vector.normalize() * 200.0 * time.delta_secs();
        }
    }
}
