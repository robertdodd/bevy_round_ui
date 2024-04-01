#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CustomUiMaterial {
    @location(0) background_color: vec4<f32>,
    @location(1) border_color: vec4<f32>,
    /// border-radius of each corner:
    // (bottom-right, top-right, bottom-left, top-left)
    @location(2) border_radius: vec4<f32>,
    /// border offset: (top, left, bottom, right)
    @location(3) offset: vec4<f32>,
    /// width, height of button
    @location(4) size: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> input: CustomUiMaterial;

// MIT License. © 2023 Inigo Quilez, Munrocket
// https://gist.github.com/munrocket/30e645d584b5300ee69295e54674b3e4
// https://compute.toys/view/398
fn sdfRoundedBox(p: vec2f, b: vec2f, r: vec4f) -> f32 {
  var x = r.x;
  var y = r.y;
  x = select(r.z, r.x, p.x > 0.);
  y = select(r.w, r.y, p.x > 0.);
  x  = select(y, x, p.y > 0.);
  let q = abs(p) - b + x;
  return min(max(q.x, q.y), 0.) + length(max(q, vec2f(0.))) - x;
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // adjust UVs around the middle of the rect, and convert to pixel
    // coordinates
    let uv = in.uv * input.size * 2.0 - input.size;

    // position offset to account for border
    let borderOffset = vec2<f32>(
        input.offset.w - input.offset.y, // right - left
        input.offset.z - input.offset.x, // bottom - top
    );

    // SDF distance in the inner button area
    // The inner button size is equal to actual size - offset size
    let buttonSize = input.size - vec2<f32>(
        input.offset.y + input.offset.w, // left + right
        input.offset.x + input.offset.z, // top + bottom
    );
    let dButton = sdfRoundedBox(
        uv + borderOffset,
        buttonSize,
        input.border_radius,
    );

    // SDF distance in border area
    let dBorder = sdfRoundedBox(uv, input.size, input.border_radius);

    // define the alpha value. Smoothly blend to avoid aliasing
    let alpha = min(-min(dButton, dBorder), 1.0);
    
    // define the final color. Use `input.background_color` if not within border
    // radius, otherwise smoothly blend from `input.background_color` to `input.border_color`.
    let color = select(
        input.background_color,
        mix(input.background_color, input.border_color, clamp(dButton, 0.0, 1.0)),
        (dButton > 0. && dBorder <= 0.),
    );

    return vec4<f32>(color.rgb, alpha * color.a);
}
