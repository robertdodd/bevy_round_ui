use bevy::{app::AppExit, prelude::*};

use bevy_round_ui::{autosize::*, prelude::*};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RoundUiPlugin))
        .init_resource::<ButtonStyle>()
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_button_interactions, handle_button_actions))
        .run();
}

/// Resource containing material handles for the different button states
#[derive(Resource)]
pub struct ButtonStyle {
    pub width: f32,
    pub height: f32,
    pub default: Handle<RoundUiMaterial>,
    pub hover: Handle<RoundUiMaterial>,
    pub press: Handle<RoundUiMaterial>,
}

impl FromWorld for ButtonStyle {
    fn from_world(world: &mut World) -> Self {
        let cell = world.cell();
        let mut materials = cell
            .get_resource_mut::<Assets<RoundUiMaterial>>()
            .expect("Failed to get Assets<RoundRectMaterial>");

        let width = 200.;
        let height = 40.;
        let offset = 5.;
        let border_radius = RoundUiBorder::all(15.);

        Self {
            width,
            height,
            default: materials.add(RoundUiMaterial {
                background_color: Color::hex("#F76161").unwrap(),
                border_color: Color::hex("#A53A3D").unwrap(),
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::bottom(offset).into(),
            }),
            hover: materials.add(RoundUiMaterial {
                background_color: Color::hex("#F61A39").unwrap(),
                border_color: Color::hex("#A0102A").unwrap(),
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::bottom(offset).into(),
            }),
            press: materials.add(RoundUiMaterial {
                background_color: Color::hex("#A0102A").unwrap(),
                border_color: Color::NONE,
                border_radius: border_radius.into(),
                size: Vec2::new(width, height),
                offset: RoundUiOffset::top(offset).into(),
            }),
        }
    }
}

/// Button actions for handling click events
#[derive(Component, Debug)]
enum ButtonAction {
    Play,
    Settings,
    Quit,
}

/// Marker component to identify round buttons
#[derive(Component)]
pub struct RoundButton;

fn spawn_button(
    parent: &mut ChildBuilder,
    button_style: &Res<ButtonStyle>,
    text: impl Into<String>,
    extras: impl Bundle,
) -> Entity {
    parent
        .spawn((
            RoundButton,
            RoundUiAutosizeNode,
            RoundUiAutosizeNodePadding,
            MaterialNodeBundle {
                material: button_style.default.clone(),
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Px(button_style.width),
                    height: Val::Px(button_style.height),
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

fn setup(
    mut commands: Commands,
    button_style: Res<ButtonStyle>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
) {
    // Camera so we can see UI
    commands.spawn(Camera2dBundle::default());

    // Define a material for the panel.
    // This material looks like it has a border, because we applied an equal offset to all sides.
    let panel_width = 400.0;
    let panel_height = 400.0;
    let panel_material = materials.add(RoundUiMaterial {
        background_color: Color::hex("5cb3af").unwrap(),
        border_color: Color::WHITE,
        border_radius: RoundUiBorder::all(20.0).into(),
        size: Vec2::new(panel_width, panel_height),
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
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
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

/// Updates button materials when their interaction changes
#[allow(clippy::type_complexity)]
fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut Handle<RoundUiMaterial>),
        (Changed<Interaction>, With<RoundButton>),
    >,
    button_style: Res<ButtonStyle>,
) {
    for (interaction, mut material) in &mut interaction_query {
        *material = match *interaction {
            Interaction::Pressed => button_style.press.clone(),
            Interaction::Hovered => button_style.hover.clone(),
            Interaction::None => button_style.default.clone(),
        };
    }
}

/// Handle button click events
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
                ButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}
