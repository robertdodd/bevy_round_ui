use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::*};

use crate::types::*;

pub const ROUND_RECT_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(66552904175742639684);

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

        app.add_plugins(UiMaterialPlugin::<RoundRectUiMaterial>::default());
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
}

impl Default for RoundRectUiMaterial {
    fn default() -> Self {
        Self {
            background_color: LinearRgba::WHITE,
            border_color: LinearRgba::NONE,
            border_radius: Vec4::splat(0.),
            offset: Vec4::splat(0.),
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
