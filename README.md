# Bevy Round UI

This [Bevy](https://bevyengine.org/) plugin contains a shader for rendering rounded rect UI elements, with an
adjustable offset that can be used to add borders or make the node appear 3D.

## Examples

![Example Screenshot Image](https://raw.githubusercontent.com/robertdodd/bevy_round_ui/master/images/screenshot.png)

The [simple](https://github.com/robertdodd/bevy_round_ui/blob/master/examples/simple.rs) example shows how to use it
in its simplest form. The [buttons](https://github.com/robertdodd/bevy_round_ui/blob/master/examples/buttons.rs)
example shows how to use it with interactive buttons and a panel. The
[autosize](https://github.com/robertdodd/bevy_round_ui/blob/master/examples/autosize.rs) example demonstrates what it
looks like when nodes resize. The [circle](https://github.com/robertdodd/bevy_round_ui/blob/master/examples/circle.rs)
example shows how to make a circle shape. The
[shapes](https://github.com/robertdodd/bevy_round_ui/blob/master/examples/shapes.rs) example shows a number of possible
shapes.

## Basic Usage

1. Add the `RoundUiPlugin` to the app.
2. Create a `RoundUiMaterial` material.
3. Spawn a regular `MaterialNodeBundle` using the material.

```rust
use bevy::prelude::*;
use bevy_round_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RoundUiPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    let panel_width = 200.0;
    let panel_height = 200.0;

    commands.spawn(MaterialNodeBundle {
        material: materials.add(RoundUiMaterial {
            background_color: Color::hex("#F76161").unwrap().into(),
            border_color: Color::hex("#A53A3D").unwrap().into(),
            border_radius: RoundUiBorder::all(20.0).into(),
            offset: RoundUiOffset::bottom(10.0).into(),
        }),
        style: Style {
            width: Val::Px(panel_width),
            height: Val::Px(panel_height),
            ..default()
        },
        ..default()
    });
}
```

## Shapes

The [shapes](https://github.com/robertdodd/bevy_round_ui/blob/master/examples/shapes.rs) example demonstrates a number
of possible shapes that can be achieved by changing the `RoundUiMaterial::border_radius` property.

> NOTE: Bordered circles are not perfect.

![Shapes Example Image](https://raw.githubusercontent.com/robertdodd/bevy_round_ui/master/images/shapes.png)

## Compatible Bevy versions

| `bevy_round_ui` | `bevy` |
|:----------------|:-------|
| `1.0`           | `0.13` |
| `0.2`           | `0.13` |
| `0.1`           | `0.12` |

## License

Dual-licensed under either of

- Apache License, Version 2.0,
  ([LICENSE-APACHE](https://github.com/robertdodd/bevy_round_ui/blob/master/LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](https://github.com/robertdodd/bevy_round_ui/blob/master/LICENSE-MIT) or
  https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
