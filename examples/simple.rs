//! This example demonstrates spawning a single rounded rect masterial node in the center of the screen.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

const PANEL_WIDTH: f32 = 200.0;
const PANEL_HEIGHT: f32 = 200.0;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundRectUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2d);

    // Add the material
    let panel_material = materials.add(RoundRectUiMaterial {
        background_color: Srgba::hex("#F76161").unwrap().into(),
        border_color: Srgba::hex("#A53A3D").unwrap().into(),
        border_radius: RoundUiBorder::all(20.0).into(),
        offset: RoundUiOffset::bottom(10.0).into(),
        ..default()
    });

    // Spawn the material in the middle of the screen
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Px(PANEL_WIDTH),
                    height: Val::Px(PANEL_HEIGHT),
                    ..default()
                },
                MaterialNode(panel_material),
            ));
        });
}
