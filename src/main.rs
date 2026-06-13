mod enemies;
mod movement;
use bevy::{ecs::system::command, prelude::*};
use bevy_egui::{EguiContext, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui};
use enemies::EnemiePlugin;
use movement::MovementPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, MovementPlugin, EnemiePlugin));
    app.add_plugins(EguiPlugin::default());
    app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);
    app.add_systems(EguiPrimaryContextPass, inspector_ui);
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
        Name::new("Player"),
    ));
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("Inspector").default_open(false).show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });
        });
    });
}

#[derive(Component)]
pub struct Player;
