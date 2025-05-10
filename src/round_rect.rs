use bevy::{
    asset::{load_internal_asset, weak_handle},
    prelude::*,
    render::render_resource::*,
};

use crate::types::*;

#[rustfmt::skip]
pub const ROUND_RECT_SHADER_HANDLE: Handle<Shader> = weak_handle!("0196b79a-6b39-71f0-a57f-87912ea368a9");

/// Plugin which adds a `RoundRectUiMaterial` to the app.
pub struct RoundRectMaterialPlugin;

impl Plugin for RoundRectMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            ROUND_RECT_SHADER_HANDLE,
            "round_rect.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(UiMaterialPlugin::<RoundRectUiMaterial>::default())
            .add_systems(PreUpdate, update_round_rect_inverse_scale_factor);
    }
}

pub fn update_round_rect_inverse_scale_factor(
    query: Query<(&ComputedNode, &MaterialNode<RoundRectUiMaterial>)>,
    mut materials: ResMut<Assets<RoundRectUiMaterial>>,
) {
    for (computed_node, material) in &query {
        if let Some(mat) = materials.get_mut(material) {
            mat.inverse_scale_factor = computed_node.inverse_scale_factor();
        }
    }
}

/// UI Material that renders a rounded rect with an optional offset color and position.
#[derive(AsBindGroup, Asset, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
pub struct RoundRectUiMaterial {
    /// The background color of the material
    #[uniform(0)]
    pub background_color: LinearRgba,

    /// The border color of the material
    #[uniform(0)]
    pub border_color: LinearRgba,

    /// The border radius of each corner
    /// E.g. Vec4::new(bottom_right, top_right, bottom_left, top_left)
    #[uniform(0)]
    pub border_radius: Vec4,

    /// The border offset along each side of the rect
    /// E.g. Vec4::new((top, left, bottom, right)
    #[uniform(0)]
    pub offset: Vec4,

    /// The ComputedNode inverse scale factor
    #[uniform(0)]
    pub inverse_scale_factor: f32,
}

impl Default for RoundRectUiMaterial {
    fn default() -> Self {
        Self {
            background_color: LinearRgba::WHITE,
            border_color: LinearRgba::NONE,
            border_radius: Vec4::splat(0.),
            offset: Vec4::splat(0.),
            inverse_scale_factor: 1.,
        }
    }
}

impl UiMaterial for RoundRectUiMaterial {
    fn fragment_shader() -> ShaderRef {
        ROUND_RECT_SHADER_HANDLE.into()
    }
}

impl RoundRectUiMaterial {
    pub fn get_padding(&self) -> UiRect {
        let offset: RoundUiOffset = self.offset.into();
        let border: RoundUiBorder = self.border_radius.into();
        UiRect {
            left: Val::Px(offset.left + border.top_left.max(border.bottom_left)),
            right: Val::Px(offset.right + border.top_right.max(border.bottom_right)),
            top: Val::Px(offset.top + border.top_left.max(border.top_right)),
            bottom: Val::Px(offset.bottom + border.bottom_left.max(border.bottom_right)),
        }
    }
}
