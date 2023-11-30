use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RoundUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

const SIZE: f32 = 100.;
const MARGIN: f32 = 20.;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    let rect_materials = [
        // Round rect without offset
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_radius: RoundUiBorder::all(20.0).into(),
            size: Vec2::new(SIZE, SIZE),
            ..default()
        }),
        // Round rect with offset
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_color: Color::hex("#A53A3D").unwrap(),
            border_radius: RoundUiBorder::all(20.0).into(),
            size: Vec2::new(SIZE, SIZE),
            offset: RoundUiOffset::bottom(10.0).into(),
        }),
        // Round rect with border
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_color: Color::hex("#FFFFFF").unwrap(),
            border_radius: RoundUiBorder::all(20.0).into(),
            size: Vec2::new(SIZE, SIZE),
            offset: RoundUiOffset::all(4.0).into(),
        }),
        // Round rect with offset to bottom right
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_color: Color::hex("#A53A3D").unwrap(),
            border_radius: RoundUiBorder::all(20.0).into(),
            size: Vec2::new(SIZE, SIZE),
            offset: RoundUiOffset::bottom_right(5.0).into(),
        }),
    ];
    let circle_materials = [
        // Circle without offset
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_radius: RoundUiBorder::all(SIZE).into(),
            size: Vec2::new(SIZE, SIZE),
            ..default()
        }),
        // Circle with offset
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_color: Color::hex("#A53A3D").unwrap(),
            border_radius: RoundUiBorder::all(SIZE).into(),
            size: Vec2::new(SIZE, SIZE),
            offset: RoundUiOffset::bottom(10.0).into(),
        }),
        // Circle with border
        // NOTE: The border is not perfect
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_color: Color::hex("#FFFFFF").unwrap(),
            border_radius: RoundUiBorder::all(SIZE).into(),
            size: Vec2::new(SIZE, SIZE),
            offset: RoundUiOffset::all(4.0).into(),
        }),
        // Circle with border-radius longer than sides
        materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap(),
            border_color: Color::hex("#A53A3D").unwrap(),
            border_radius: RoundUiBorder::all(SIZE * 2.).into(),
            size: Vec2::new(SIZE, SIZE),
            offset: RoundUiOffset::bottom(10.0).into(),
        }),
    ];

    // Spawn the material in the middle of the screen
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
                                margin: UiRect::all(Val::Px(MARGIN)),
                                width: Val::Px(SIZE),
                                height: Val::Px(SIZE),
                                ..default()
                            },
                            ..default()
                        });
                    }
                });
            }
        });
}
