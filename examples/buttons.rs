//! This example demonstrates using the plugin with interactive UI elements, such as buttons.

use bevy::{app::AppExit, prelude::*};

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .init_resource::<ButtonStyle>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_button_interactions, handle_button_actions))
        .run();
}

const PANEL_BACKGROUND_COLOR: &str = "#5cb3af";
const PANEL_BORDER_COLOR: &str = "#ffffff";

const PANEL_WIDTH: f32 = 400.0;
const PANEL_HEIGHT: f32 = 400.0;

const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 40.0;
const BUTTON_OFFSET_SIZE: f32 = 5.0;

/// Resource containing styles for different button states.
#[derive(Resource)]
pub struct ButtonStyle {
    pub default_material: Handle<RoundRectUiMaterial>,
    pub default_padding: UiRect,
    pub hover_material: Handle<RoundRectUiMaterial>,
    pub hover_padding: UiRect,
    pub press_material: Handle<RoundRectUiMaterial>,
    pub press_padding: UiRect,
}

impl FromWorld for ButtonStyle {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell
            .get_resource_mut::<Assets<RoundRectUiMaterial>>()
            .expect("Failed to get Assets<RoundRectMaterial>");

        let border_radius = RoundUiBorder::all(15.);

        Self {
            default_material: materials.add(RoundRectUiMaterial {
                background_color: Color::hex("#F76161").unwrap(),
                border_color: Color::hex("#A53A3D").unwrap(),
                border_radius: border_radius.into(),
                offset: RoundUiOffset::bottom(BUTTON_OFFSET_SIZE).into(),
            }),
            default_padding: UiRect::bottom(Val::Px(BUTTON_OFFSET_SIZE)),
            hover_material: materials.add(RoundRectUiMaterial {
                background_color: Color::hex("#F61A39").unwrap(),
                border_color: Color::hex("#A0102A").unwrap(),
                border_radius: border_radius.into(),
                offset: RoundUiOffset::bottom(BUTTON_OFFSET_SIZE).into(),
            }),
            hover_padding: UiRect::bottom(Val::Px(BUTTON_OFFSET_SIZE)),
            press_material: materials.add(RoundRectUiMaterial {
                background_color: Color::hex("#A0102A").unwrap(),
                border_color: Color::NONE,
                border_radius: border_radius.into(),
                offset: RoundUiOffset::top(BUTTON_OFFSET_SIZE).into(),
            }),
            press_padding: UiRect::top(Val::Px(BUTTON_OFFSET_SIZE)),
        }
    }
}

/// Component defining button actions for handling click events
#[derive(Component, Debug)]
enum ButtonAction {
    Play,
    Settings,
    Quit,
}

/// Component identifying round buttons.
#[derive(Component)]
pub struct RoundButton;

/// System that initializes the example.
fn setup(
    mut commands: Commands,
    button_style: Res<ButtonStyle>,
    mut materials: ResMut<Assets<RoundRectUiMaterial>>,
) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    // Define a material for the panel.
    // This material looks like it has a border, because we applied an equal offset to all sides.
    let panel_material = materials.add(RoundRectUiMaterial {
        background_color: Color::hex(PANEL_BACKGROUND_COLOR).unwrap(),
        border_color: Color::hex(PANEL_BORDER_COLOR).unwrap(),
        border_radius: RoundUiBorder::all(20.0).into(),
        offset: RoundUiOffset::all(6.0).into(),
    });

    // Spawn the screen layout, containing a centered panel with menu items
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
                    width: Val::Px(PANEL_WIDTH),
                    height: Val::Px(PANEL_HEIGHT),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|p| {
                // Spawn the title
                p.spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::bottom(Val::Px(30.)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "MENU",
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 40.,
                            ..default()
                        },
                    ));
                });

                // Spawn the buttons
                spawn_button(p, &button_style, "Play", ButtonAction::Play);
                spawn_button(p, &button_style, "Settings", ButtonAction::Settings);
                spawn_button(p, &button_style, "Quit", ButtonAction::Quit);
            });
        });
}

/// Utility that spawns a new button.
fn spawn_button(
    parent: &mut ChildBuilder,
    button_style: &ButtonStyle,
    text: impl Into<String>,
    extras: impl Bundle,
) -> Entity {
    parent
        .spawn((
            RoundButton,
            MaterialNodeBundle {
                material: button_style.default_material.clone(),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(BUTTON_WIDTH),
                    height: Val::Px(BUTTON_HEIGHT),
                    margin: UiRect::bottom(Val::Px(10.)),
                    ..default()
                },
                ..default()
            },
            extras,
            Interaction::default(),
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    color: Color::WHITE,
                    font_size: 20.,
                    ..default()
                },
            ));
        })
        .id()
}

/// System that updates button materials and styles when their interaction state changes
#[allow(clippy::type_complexity)]
fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut Handle<RoundRectUiMaterial>, &mut Style),
        (Changed<Interaction>, With<RoundButton>),
    >,
    button_style: Res<ButtonStyle>,
) {
    for (interaction, mut material, mut style) in &mut interaction_query {
        let (material_handle, padding) = match *interaction {
            Interaction::Pressed => (
                button_style.press_material.clone(),
                button_style.press_padding,
            ),
            Interaction::Hovered => (
                button_style.hover_material.clone(),
                button_style.hover_padding,
            ),
            Interaction::None => (
                button_style.default_material.clone(),
                button_style.default_padding,
            ),
        };
        *material = material_handle;
        style.padding = padding;
    }
}

/// System that handles button click events
fn handle_button_actions(
    interaction_query: Query<(&Interaction, &ButtonAction), Changed<Interaction>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Button pressed: {action:?}");
            match action {
                ButtonAction::Play => (),
                ButtonAction::Settings => (),
                ButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}
