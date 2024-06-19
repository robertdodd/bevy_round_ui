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
        }
    }
}
