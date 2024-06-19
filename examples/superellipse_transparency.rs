//! This example demonstrates spawning a superellipse material node.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

const PANEL_WIDTH: f32 = 400.0;
const PANEL_HEIGHT: f32 = 200.0;
const BORDER_THICKNESS: f32 = 10.;

#[derive(Component)]
pub enum PanelSize {
    Short,
    Square,
    Long,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<SuperellipseUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    let border_radius: Vec4 = RoundUiBorder::all(PANEL_WIDTH / 4.).into();
    let background_color = Color::rgba(0.36078432, 0.7019608, 0.6862745, 0.5);
    let border_color = Color::rgba(1., 1., 1., 0.25);

    // Add the material
    let panel_material_superellipse = materials.add(SuperellipseUiMaterial {
        background_color,
        border_color,
        border_radius,
        border_thickness: BORDER_THICKNESS,
    });

    // Spawn 2 colored columns so we can see the transparency of the material
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            });
            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::ORANGE_RED.into(),
                ..default()
            });
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
            p.spawn((
                PanelSize::Short,
                MaterialNodeBundle {
                    material: panel_material_superellipse,
                    style: Style {
                        width: Val::Px(PANEL_WIDTH),
                        height: Val::Px(PANEL_HEIGHT),
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}
