use bevy::{asset::load_internal_asset, prelude::*};

use crate::prelude::*;

/// A plugin which adds a round rect UI material to the app
pub struct RoundUiPlugin;

impl Plugin for RoundUiPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, SHADER_HANDLE, "shader.wgsl", Shader::from_wgsl);

        app.add_plugins(UiMaterialPlugin::<RoundUiMaterial>::default());
    }
}
