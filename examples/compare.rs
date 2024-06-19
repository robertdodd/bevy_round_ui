//! This example illustrates the difference between round rect and superellipse materials.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_keys)
        .run();
}

const PANEL_WIDTH: f32 = 400.0;
const PANEL_HEIGHT: f32 = 200.0;
const BORDER_THICKNESS: f32 = 40.;

#[derive(Component)]
pub enum PanelSize {
    Short,
    Square,
    Long,
}

#[derive(Component)]
pub struct PanelBorder(pub bool);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<SuperellipseUiMaterial>>,
    mut old_materials: ResMut<Assets<RoundRectUiMaterial>>,
) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    let border_radius: Vec4 = RoundUiBorder::all(PANEL_WIDTH / 4.).into();
    let background_color = Color::hex("#F76161").unwrap();
    let border_color = Color::WHITE;

    // Add the superellipse material
    let panel_material_superellipse = materials.add(SuperellipseUiMaterial {
        background_color,
        border_color,
        border_radius,
        border_thickness: BORDER_THICKNESS,
    });
    // Add the round rect material
    let panel_material_round_rect = old_materials.add(RoundRectUiMaterial {
        background_color,
        border_color,
        border_radius,
        offset: RoundUiOffset::all(BORDER_THICKNESS / 2.).into(),
    });

    // Spawn help text
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            help_text(p, "Toggle Material:    ENTER");
            help_text(p, "Toggle Height:      SPACE");
            help_text(p, "Toggle Node Border: B");
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
            // Spawn a wrapper around the material node with a blue border
            p.spawn((
                PanelBorder(true),
                NodeBundle {
                    style: Style {
                        border: UiRect::all(Val::Px(1.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    border_color: Color::BLUE.into(),
                    ..default()
                },
            ))
            .with_children(|p| {
                // Spawn a superellipse material, displayed by default
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
                // Spawn a round rect material, hidden by default
                p.spawn((
                    PanelSize::Short,
                    MaterialNodeBundle {
                        material: panel_material_round_rect,
                        style: Style {
                            width: Val::Px(PANEL_WIDTH),
                            height: Val::Px(PANEL_HEIGHT),
                            display: Display::None,
                            ..default()
                        },
                        ..default()
                    },
                ));
            });
        });
}

fn help_text(p: &mut ChildBuilder, val: impl Into<String>) {
    p.spawn(TextBundle::from_section(
        val,
        TextStyle {
            font_size: 24.,
            ..default()
        },
    ));
}

fn handle_keys(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PanelSize, &mut Style)>,
    mut border_query: Query<(&mut PanelBorder, &mut Style), Without<PanelSize>>,
) {
    // Change height when "Space" is pressed
    if keys.just_pressed(KeyCode::Space) {
        for (mut panel_size, mut style) in query.iter_mut() {
            *panel_size = match *panel_size {
                PanelSize::Short => PanelSize::Square,
                PanelSize::Square => PanelSize::Long,
                PanelSize::Long => PanelSize::Short,
            };
            match *panel_size {
                PanelSize::Short => {
                    style.height = Val::Px(PANEL_HEIGHT);
                }
                PanelSize::Square => {
                    style.height = Val::Px(PANEL_HEIGHT + 200.);
                }
                PanelSize::Long => {
                    style.height = Val::Px(PANEL_HEIGHT + 400.);
                }
            };
        }
    }

    // Change material when "Enter" is pressed
    if keys.just_pressed(KeyCode::Enter) {
        for (_, mut style) in query.iter_mut() {
            style.display = match style.display {
                Display::None => Display::Flex,
                _ => Display::None,
            };
        }
    }

    // Toggle border "B" is pressed
    if keys.just_pressed(KeyCode::KeyB) {
        for (mut border, mut style) in border_query.iter_mut() {
            border.0 = !border.0;
            style.border = match border.0 {
                true => UiRect::all(Val::Px(1.)),
                false => UiRect::ZERO,
            };
        }
    }
}
