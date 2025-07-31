mod types;

#[cfg(feature = "round_rect")]
mod round_rect;

#[cfg(feature = "superellipse")]
mod superellipse;

pub mod prelude {
    use bevy::prelude::*;

    pub use crate::types::*;

    #[cfg(feature = "round_rect")]
    pub use crate::round_rect::*;
    #[cfg(feature = "superellipse")]
    pub use crate::superellipse::*;

    /// Plugin that will add all material plugins that have been enabled to the app.
    pub struct BevyRoundUiDefaultPlugins;

    impl Plugin for BevyRoundUiDefaultPlugins {
        fn build(&self, app: &mut App) {
            #[cfg(feature = "round_rect")]
            app.add_plugins(RoundRectMaterialPlugin);
            #[cfg(feature = "superellipse")]
            app.add_plugins(SuperellipseMaterialPlugin);

            app.add_systems(Update, timer_update);
        }
    }

    fn timer_update(
        mut rect_materials: ResMut<Assets<RoundRectUiMaterial>>,
        mut ellipse_materials: ResMut<Assets<SuperellipseUiMaterial>>,
        time: Res<Time>,
    ) {
        for (_, material) in rect_materials.iter_mut() {
            material.time = time.elapsed_secs();
        }
        for (_, material) in ellipse_materials.iter_mut() {
            material.time = time.elapsed_secs();
        }
    }
}
