//! This example demonstrates how RoundUiMaterial nodes automaticall adjust when their node size changes.

use bevy::{color::palettes::css, prelude::*};

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundRectUiMaterial>>) {
    // Camera so we can see UI
    commands.spawn(Camera2d);

    // Spawn two nested panels with flexible sizes in the middle of the window
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
                    width: Val::Percent(50.),
                    height: Val::Percent(50.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    overflow: Overflow::clip(),
                    padding: UiRect::all(Val::Px(12.)),
                    ..default()
                },
                MaterialNode(materials.add(RoundRectUiMaterial {
                    background_color: css::PINK.into(),
                    border_color: LinearRgba::WHITE,
                    border_radius: RoundUiBorder::all(20.).into(),
                    offset: RoundUiOffset::all(6.).into(),
                    ..default()
                })),
            ))
            .with_children(|p| {
                p.spawn((
                    Node {
                        width: Val::Percent(50.),
                        height: Val::Percent(50.),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        overflow: Overflow::clip(),
                        padding: UiRect::all(Val::Px(12.)),
                        ..default()
                    },
                    MaterialNode(materials.add(RoundRectUiMaterial {
                        background_color: Srgba::hex("5cb3af").unwrap().into(),
                        border_color: LinearRgba::WHITE,
                        border_radius: RoundUiBorder::all(20.0).into(),
                        offset: RoundUiOffset::all(6.0).into(),
                        ..default()
                    })),
                ))
                .with_children(|p| {
                    p.spawn((
                        Text::new("Resize the window to see how flexible I am"),
                        TextFont {
                            font_size: 20.,
                            ..default()
                        },
                        TextColor::WHITE,
                    ));
                });
            });
        });
}
