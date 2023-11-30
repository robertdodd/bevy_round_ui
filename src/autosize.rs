use bevy::prelude::*;

use crate::prelude::*;

/// Use this set to control when the autosize systems run
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct RoundUiAutosizeSet;

/// A plugin which adds auto-sizing functionality for the RoundUiPlugin
pub(crate) struct RoundUiAutosizePlugin;

impl Plugin for RoundUiAutosizePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_material_size, update_node_padding, update_node_size)
                .in_set(RoundUiAutosizeSet),
        );
    }
}

/// Automatically update the material to match the size of the node.
/// Should be added to an entity with `MaterialNodeBundle<RoundUiMaterial>`.
#[derive(Component)]
pub struct RoundUiAutosizeMaterial;

/// Automatically update the padding of a `MaterialNodeBundle<RoundUiMaterial>` to match its material
#[derive(Component)]
pub struct RoundUiAutosizeNodePadding;

/// Automatically update the size of nodes to match the material size
#[derive(Component, Default)]
pub struct RoundUiAutosizeNode;

/// Updates the material when it's node size changes
#[allow(clippy::type_complexity)]
fn update_material_size(
    query: Query<(&Node, &Handle<RoundUiMaterial>), (With<RoundUiAutosizeMaterial>, Changed<Node>)>,
    mut materials: ResMut<Assets<RoundUiMaterial>>,
) {
    for (node, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle.id()) {
            material.size = node.size();
        }
    }
}

/// Updates the padding of a material node to match it's material
#[allow(clippy::type_complexity)]
fn update_node_padding(
    mut query: Query<
        (&mut Style, &Handle<RoundUiMaterial>),
        (
            With<RoundUiAutosizeNodePadding>,
            Changed<Handle<RoundUiMaterial>>,
        ),
    >,
    materials: Res<Assets<RoundUiMaterial>>,
) {
    for (mut style, material_handle) in query.iter_mut() {
        if let Some(material) = materials.get(material_handle.id()) {
            style.padding = material.get_padding();
        }
    }
}

/// Update node padding and size when the material changes
#[allow(clippy::type_complexity)]
fn update_node_size(
    materials: Res<Assets<RoundUiMaterial>>,
    mut query: Query<
        (&mut Style, &Handle<RoundUiMaterial>),
        (Changed<Handle<RoundUiMaterial>>, With<RoundUiAutosizeNode>),
    >,
) {
    for (mut style, handle) in query.iter_mut() {
        if let Some(material) = materials.get(handle.id()) {
            style.width = Val::Px(material.size.x);
            style.height = Val::Px(material.size.y);
        }
    }
}
