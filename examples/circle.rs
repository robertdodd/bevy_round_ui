//! This example demonstrates how a circle shape can be achieved by setting [`RoundUiMaterial::offset`] to the same
//! size as the node.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

const CIRCLE_DIAMETER: f32 = 200.;
const CIRCLE_OFFSET_SIZE: f32 = 10.;

const CIRCLE_BACKGROUND_COLOR: &str = "#F76161";
const CIRCLE_BORDER_COLOR: &str = "#A53A3D";

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundRectUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    // Add the material asset.
    // NOTE: To make a circle, the width, height and border radius should be equal
    let circle_material = materials.add(RoundRectUiMaterial {
        background_color: Srgba::hex(CIRCLE_BACKGROUND_COLOR).unwrap().into(),
        border_color: Srgba::hex(CIRCLE_BORDER_COLOR).unwrap().into(),
        border_radius: RoundUiBorder::all(CIRCLE_DIAMETER).into(),
        offset: RoundUiOffset::bottom(CIRCLE_OFFSET_SIZE).into(),
    });

    // Spawn a round material node in the middle of the screen
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
                material: circle_material,
                style: Style {
                    width: Val::Px(CIRCLE_DIAMETER),
                    height: Val::Px(CIRCLE_DIAMETER),
                    ..default()
                },
                ..default()
            });
        });
}
