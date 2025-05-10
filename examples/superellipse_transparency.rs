//! This example demonstrates spawning a superellipse material node.

use bevy::{color::palettes::css, prelude::*};

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
    commands.spawn(Camera2d);

    let border_radius: Vec4 = RoundUiBorder::all(PANEL_WIDTH / 4.).into();
    let background_color: LinearRgba = Color::srgba(0.36078432, 0.7019608, 0.6862745, 0.5).into();
    let border_color: LinearRgba = Color::srgba(1., 1., 1., 0.25).into();

    // Add the material
    let panel_material_superellipse = materials.add(SuperellipseUiMaterial {
        background_color,
        border_color,
        border_radius,
        border_thickness: BORDER_THICKNESS,
        ..default()
    });

    // Spawn 2 colored columns so we can see the transparency of the material
    commands
        .spawn((Node {
            flex_direction: FlexDirection::Row,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },))
        .with_children(|p| {
            p.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(css::DARK_GRAY.into()),
            ));
            p.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(50.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(css::ORANGE_RED.into()),
            ));
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
                PanelSize::Short,
                Node {
                    width: Val::Px(PANEL_WIDTH),
                    height: Val::Px(PANEL_HEIGHT),
                    ..default()
                },
                MaterialNode(panel_material_superellipse),
            ));
        });
}
