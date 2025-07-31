//! This example demonstrates spawning a single rounded rect masterial node in the center of the screen.

use bevy::prelude::*;

use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BevyRoundUiDefaultPlugins))
        .add_systems(Startup, setup)
        .add_systems(Update, inc_value)
        .run();
}

fn inc_value(mut rect_materials: ResMut<Assets<RoundRectUiMaterial>>, time: Res<Time>) {
    for (_, mat) in rect_materials.iter_mut() {
        mat.value = time.elapsed_secs() / 10.0;
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<RoundRectUiMaterial>>,
    mut windows: Query<&mut Window>,
) {
    let window = windows.single_mut();

    // Camera so we can see UI
    commands.spawn(Camera2d);

    let panel_width = 200.0;
    let panel_height = 200.0;
    //let panel_width = window.width();
    //let panel_height = window.height();

    // Add the material
    let panel_material = materials.add(RoundRectUiMaterial {
        //background_color: Srgba::hex("#F76161").unwrap().into(),
        background_color: LinearRgba::GREEN,
        border_color: Srgba::hex("#A53A3D").unwrap().into(),
        border_radius: RoundUiBorder::all(20.0).into(),
        offset: RoundUiOffset::bottom(10.0).into(),
        turbulence_color: LinearRgba::BLUE,
        power: 1f32,
        time: 0f32,
        resolution: Vec2::new(window.width(), window.height()),
        value: 0f32,
        /*
        texture0: Some(asset_server.load("textures/tex0.png")),
        texture1: Some(asset_server.load("textures/tex1.png")),
        texture2: Some(asset_server.load("textures/tex2.png")),
        */
        ..default()
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
                Node {
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    ..default()
                },
                MaterialNode(panel_material.clone()),
            ));
            p.spawn((
                Node {
                    width: Val::Px(panel_width / 2.0),
                    height: Val::Px(panel_height / 2.0),
                    ..default()
                },
                MaterialNode(panel_material.clone()),
            ));
            p.spawn((
                Node {
                    width: Val::Px(panel_width),
                    height: Val::Px(panel_height),
                    ..default()
                },
                MaterialNode(panel_material.clone()),
            ));
        });

    // Spawn a progress bar
    let bar_width = 600.0;
    let bar_height = 10.0;
    let bar_material = materials.add(RoundRectUiMaterial {
        //background_color: Srgba::hex("#F76161").unwrap().into(),
        background_color: LinearRgba::RED,
        border_color: Srgba::hex("#A53A3D").unwrap().into(),
        border_radius: RoundUiBorder::all(0.0).into(),
        offset: RoundUiOffset::all(1.0).into(),
        turbulence_color: LinearRgba::WHITE,
        power: 1f32,
        time: 0f32,
        resolution: Vec2::new(window.width(), window.height()),
        value: 0f32,
        ..default()
    });
    commands
        .spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            //margin: UiRect::all(Val::Px(50.)),
            align_items: AlignItems::End,
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|p| {
            p.spawn((
                Node {
                    width: Val::Px(bar_width),
                    height: Val::Px(bar_height),
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::End,
                    //position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                MaterialNode(bar_material.clone()),
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("0.00%"),
                    TextFont {
                        font_size: 10.,
                        ..default()
                    },
                    TextColor::BLACK,
                ));
            });
        });
}
