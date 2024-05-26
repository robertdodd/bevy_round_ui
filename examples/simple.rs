//! This example demonstrates spawning a single rounded rect masterial node in the center of the screen.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RoundUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

const PANEL_WIDTH: f32 = 200.0;
const PANEL_HEIGHT: f32 = 200.0;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    // Add the material
    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("#F76161").unwrap(),
        border_color: Color::hex("#A53A3D").unwrap(),
        border_radius: RoundUiBorder::all(20.0).into(),
        offset: RoundUiOffset::bottom(10.0).into(),
    });

    // Spawn the material in the middle of the screen
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(MaterialNodeBundle {
                material: panel_material,
                style: Style {
                    width: Val::Px(PANEL_WIDTH),
                    height: Val::Px(PANEL_HEIGHT),
                    ..default()
                },
                ..default()
            });
        });
}
