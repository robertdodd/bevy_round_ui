//! This example demonstrates different shapes that can be achieved by updating the [`RoundUiMaterial::border_radius`]
//! property.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

const SHAPE_SIZE: f32 = 100.;
const SPACER_SIZE: f32 = 20.;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundRectUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    let rect_materials = [
        // Round rect without offset
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_radius: RoundUiBorder::all(20.0).into(),
            ..default()
        }),
        // Round rect with offset
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_color: Srgba::hex("#A53A3D").unwrap().into(),
            border_radius: RoundUiBorder::all(20.0).into(),
            offset: RoundUiOffset::bottom(10.0).into(),
        }),
        // Round rect with border
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_color: Srgba::hex("#FFFFFF").unwrap().into(),
            border_radius: RoundUiBorder::all(20.0).into(),
            offset: RoundUiOffset::all(4.0).into(),
        }),
        // Round rect with offset to bottom right
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_color: Srgba::hex("#A53A3D").unwrap().into(),
            border_radius: RoundUiBorder::all(20.0).into(),
            offset: RoundUiOffset::bottom_right(5.0).into(),
        }),
    ];
    let circle_materials = [
        // Circle without offset
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_radius: RoundUiBorder::all(SHAPE_SIZE).into(),
            ..default()
        }),
        // Circle with offset
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_color: Srgba::hex("#A53A3D").unwrap().into(),
            border_radius: RoundUiBorder::all(SHAPE_SIZE).into(),
            offset: RoundUiOffset::bottom(10.0).into(),
        }),
        // Circle with border
        // NOTE: The border is not perfect
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_color: Srgba::hex("#FFFFFF").unwrap().into(),
            border_radius: RoundUiBorder::all(SHAPE_SIZE).into(),
            offset: RoundUiOffset::all(4.0).into(),
        }),
        // Circle with border-radius longer than sides
        materials.add(RoundRectUiMaterial {
            background_color: Srgba::hex("#F76161").unwrap().into(),
            border_color: Srgba::hex("#A53A3D").unwrap().into(),
            border_radius: RoundUiBorder::all(SHAPE_SIZE * 2.).into(),
            offset: RoundUiOffset::bottom(10.0).into(),
        }),
    ];

    // Spawn two rows of material nodes in the center of the screen
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            for row in [rect_materials.iter(), circle_materials.iter()] {
                p.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    for material in row {
                        p.spawn(MaterialNodeBundle {
                            material: material.clone(),
                            style: Style {
                                margin: UiRect::all(Val::Px(SPACER_SIZE)),
                                width: Val::Px(SHAPE_SIZE),
                                height: Val::Px(SHAPE_SIZE),
                                ..default()
                            },
                            ..default()
                        });
                    }
                });
            }
        });
}
