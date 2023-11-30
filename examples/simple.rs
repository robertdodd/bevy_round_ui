use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RoundUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    let panel_width = 200.0;
    let panel_height = 200.0;

    // Add the material
    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("#F76161").unwrap(),
        border_color: Color::hex("#A53A3D").unwrap(),
        border_radius: RoundUiBorder::all(20.0).into(),
        size: Vec2::new(panel_width, panel_height),
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
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    ..default()
                },
                ..default()
            });
        });
}
