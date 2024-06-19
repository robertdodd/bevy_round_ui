//! This example demonstrates how RoundUiMaterial nodes automaticall adjust when their node size changes.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundRectUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    // Spawn two nested panels with flexible sizes in the middle of the window
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
                material: materials.add(RoundRectUiMaterial {
                    background_color: Color::PINK,
                    border_color: Color::WHITE,
                    border_radius: RoundUiBorder::all(20.).into(),
                    offset: RoundUiOffset::all(6.).into(),
                }),
                style: Style {
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    padding: UiRect::all(Val::Px(12.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                p.spawn(MaterialNodeBundle {
                    material: materials.add(RoundRectUiMaterial {
                        background_color: Color::hex("5cb3af").unwrap(),
                        border_color: Color::WHITE,
                        border_radius: RoundUiBorder::all(20.0).into(),
                        offset: RoundUiOffset::all(6.0).into(),
                    }),
                    style: Style {
                        width: Val::Percent(50.),
                        height: Val::Percent(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        overflow: Overflow::clip(),
                        padding: UiRect::all(Val::Px(12.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "Resize the window to see how flexible I am",
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 20.,
                            ..default()
                        },
                    ));
                });
            });
        });
}
