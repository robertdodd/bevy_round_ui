use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RoundUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

const CIRCLE_DIAMETER: f32 = 200.;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    // Add the material
    // NOTE: To make a circle, the width, height and border radius should be equal
    let circle_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("#F76161").unwrap(),
        border_color: Color::hex("#A53A3D").unwrap(),
        border_radius: RoundUiBorder::all(CIRCLE_DIAMETER).into(),
        size: Vec2::new(CIRCLE_DIAMETER, CIRCLE_DIAMETER),
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
