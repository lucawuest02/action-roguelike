use std::time::Duration;

use bevy::prelude::*;

use crate::Player;

pub struct EnemiePlugin;

impl Plugin for EnemiePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawning, movement));
    }
}

#[derive(Component)]
struct Enemy;

fn spawning(
    time: Res<Time>,
    enemies: Query<(), With<Enemy>>,
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut cooldown: Local<Duration>,
) {
    *cooldown = cooldown.saturating_sub(time.delta());
    if !cooldown.is_zero() { return }
    if enemies.count() > 5 { return }

    command.spawn((
        Mesh2d(meshes.add(Circle::new(30.0))),
        MeshMaterial2d(materials.add(Color::hsl(0.0, 1.0, 0.5))),
        Transform::from_xyz(200.0, 0.0, 0.0),
        Enemy,
    ));

    *cooldown = Duration::from_secs(5);
}

fn movement(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let Ok(target) = player.single() else { return };
    let target = target.translation;

    for mut enemy in enemies.iter_mut() {
        let mut dir = target - enemy.translation;
        dir = dir.clamp_length_max(1.0);

        enemy.translation += dir;
    }
}
