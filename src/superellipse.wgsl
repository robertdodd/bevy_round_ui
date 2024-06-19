#import bevy_ui::ui_vertex_output::UiVertexOutput

struct SuperellipseUiMaterial {
    /// Background color
    @location(0) background_color: vec4<f32>,
    /// Border color
    @location(1) border_color: vec4<f32>,
    /// border-radius of each corner:
    // (bottom-right, top-right, bottom-left, top-left)
    @location(2) border_radius: vec4<f32>,
    /// Border thickness: ignored if `border_color.a == 0.0`
    @location(3) border_thickness: f32,
}

@group(1) @binding(0)
var<uniform> input: SuperellipseUiMaterial;

/// Adapted from:
///   https://www.shadertoy.com/view/4cG3R1
///
/// Related article:
///   https://iquilezles.org/articles/roundedboxes/
fn approx_sd_super_ellipse(p: vec2f, b: vec2f, r: vec4f) -> f32 {
    // select corner radius
    var r_xy = select(r.zw, r.xy, p.x > 0.);
    var n = select(r_xy.y, r_xy.x, p.y > 0.);

    let abs_p = abs(p);

    // really bad, cheap linearliation of the basic implicit formula
    n = 2.0 / n;
    let w = pow(abs_p.x / b.x, n) + pow(abs_p.y / b.y, n);
    let kb = 2.0 * n - 2.0;
    let ka = 1.0 - 1.0 / n;
    let kc = 2.0 * n;
    return (w - pow(w, ka)) * inverseSqrt(pow(abs_p.x, kb) / pow(b.x, kc) + pow(abs_p.y, kb) / pow(b.y, kc));
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // compute whether we should display the border
    let is_border = input.border_thickness > 0. && input.border_color.a > 0.;

    // adjust size by subtracting the border thickness
    var size = in.size;
    if is_border {
        size -= vec2f(input.border_thickness);
    }

    // adjust UVs around the middle of the rect, and convert to pixel
    // coordinates.
    let uv = in.uv * in.size * 2.0 - in.size;

    // define the shortest length of the image, as we need it to adjust the UV,
    // size and border coordinates.
    let min_size = min(in.size.x, in.size.y);

    // IMPORTANT: Minimum border radius of 0.2, otherwise the approximation
    // behaves strangely.
    let border_radius = max(input.border_radius / min_size, vec4f(0.2));

    // Compute signed distance
    let d = approx_sd_super_ellipse(
        uv / vec2f(min_size),
        size / vec2f(min_size),
        border_radius,
    );

    // define the alpha and color values depending on the distance sign.
    let alpha = select(1., 0., d > 0.);
    var col: vec3f = select(input.background_color.rgb, vec3f(0.), d > 0.0);

    // // Debug: Show distance
    // col *= 1.0 - exp(-6.0 * abs(d));
    // col *= 0.8 + 0.2 * cos(150.0 * d);

    // Apply border color if `a > 0.0` and return
    var result = vec4f(col, alpha);
    if is_border {
        let border_thickness_uv = input.border_thickness / min_size;
        result = mix(result, input.border_color, 1.0 - smoothstep(0.0, border_thickness_uv, abs(d)));
    }

    return result;
}
