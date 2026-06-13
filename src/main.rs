mod enemies;
mod movement;
use bevy::{ecs::system::command, prelude::*};
use enemies::EnemiePlugin;
use movement::MovementPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, MovementPlugin, EnemiePlugin));
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::hsl(180.0, 1.0, 1.0))),
        Transform::IDENTITY,
        Player,
    ));
}

#[derive(Component)]
struct Player;
