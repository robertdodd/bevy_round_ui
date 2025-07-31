use bevy::{asset::load_internal_asset, prelude::*, render::render_resource::*};

pub const SUPERELLIPSE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(84071151984186645753);

/// Plugin which adds a `SuperellipseUiMaterial` to the app.
pub struct SuperellipseMaterialPlugin;

impl Plugin for SuperellipseMaterialPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SUPERELLIPSE_SHADER_HANDLE,
            "superellipse.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(UiMaterialPlugin::<SuperellipseUiMaterial>::default())
            .add_systems(PreUpdate, update_superellipse_inverse_scale_factor);
    }
}

pub fn update_superellipse_inverse_scale_factor(
    query: Query<(&ComputedNode, &MaterialNode<SuperellipseUiMaterial>)>,
    mut materials: ResMut<Assets<SuperellipseUiMaterial>>,
) {
    for (computed_node, material) in &query {
        if let Some(mat) = materials.get_mut(material) {
            mat.inverse_scale_factor = computed_node.inverse_scale_factor();
        }
    }
}

/// UI Material that renders a rounded rect using an approximate superellipse SDF function, with an optional border.
///
/// NOTE: A minimum border radius is enforced by the shader, due to a limitation in the approximate superellipse SDF
/// function. If one of your border radii is too small, is will appear larger.
#[derive(AsBindGroup, Asset, Debug, Clone, Reflect)]
#[reflect(Default, Debug)]
pub struct SuperellipseUiMaterial {
    /// The background color of the material
    #[uniform(0)]
    pub background_color: LinearRgba,

    /// The border color of the material
    #[uniform(0)]
    pub border_color: LinearRgba,

    /// The border radius of each corner
    /// E.g. Vec4::new(bottom_right, top_right, bottom_left, top_left)
    ///
    /// NOTE: A minimum border radius is enforced by the shader, due to a limitation in the approximate superellipse
    /// SDF function. If one of your border radii is too small, is will appear larger.
    #[uniform(0)]
    pub border_radius: Vec4,

    /// The thickness of the border
    #[uniform(0)]
    pub border_thickness: f32,

    /// The ComputedNode inverse scale factor
    #[uniform(0)]
    pub inverse_scale_factor: f32,

    /// Time
    #[uniform(0)]
    pub time: f32,
}

impl Default for SuperellipseUiMaterial {
    fn default() -> Self {
        Self {
            background_color: LinearRgba::WHITE,
            border_color: LinearRgba::NONE,
            border_radius: Vec4::splat(0.),
            border_thickness: 0.,
            inverse_scale_factor: 1.,
            time: 0f32,
        }
    }
}

impl UiMaterial for SuperellipseUiMaterial {
    fn fragment_shader() -> ShaderRef {
        SUPERELLIPSE_SHADER_HANDLE.into()
    }
}
