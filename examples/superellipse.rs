//! This example demonstrates spawning a superellipse material node.

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
const BORDER_THICKNESS: f32 = 4.;

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
    let background_color: LinearRgba = Srgba::hex("#5cb3af").unwrap().into();
    let border_color: LinearRgba = LinearRgba::WHITE;

    // Add the material
    let panel_material_superellipse = materials.add(SuperellipseUiMaterial {
        background_color,
        border_color,
        border_radius,
        border_thickness: BORDER_THICKNESS,
        ..default()
    });

    // Spawn help text
    commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.)),
            ..default()
        },))
        .with_children(|p| {
            help_text(p, "Toggle Height:      SPACE");
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

fn help_text(p: &mut ChildBuilder, val: impl Into<String>) {
    p.spawn((
        Text::new(val),
        TextFont {
            font_size: 24.,
            ..default()
        },
    ));
}

fn handle_keys(keys: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut PanelSize, &mut Node)>) {
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
}
