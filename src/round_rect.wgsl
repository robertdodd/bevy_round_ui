#import bevy_ui::ui_vertex_output::UiVertexOutput

struct RoundUiMaterial {
    @location(0) background_color: vec4<f32>,
    @location(1) border_color: vec4<f32>,
    /// border-radius of each corner:
    // (bottom-right, top-right, bottom-left, top-left)
    @location(2) border_radius: vec4<f32>,
    /// border offset: (top, left, bottom, right)
    @location(3) offset: vec4<f32>,
    /// Inverse scale factor: must be updated to match the ComputedNode
    @location(4) inverse_scale_factor: f32,

    @location(5) turbulence_color: vec4<f32>,
    @location(6) power: f32,
    @location(7) time: f32,
    @location(8) resolution: vec2<f32>,
    @location(9) value: f32,
}

@group(1) @binding(0)
var<uniform> input: RoundUiMaterial;

@group(1) @binding(1)
var texture0: texture_2d<f32>;
@group(1) @binding(2)
var sampler0: sampler;
@group(1) @binding(3)
var texture1: texture_2d<f32>;
@group(1) @binding(4)
var sampler1: sampler;
@group(1) @binding(5)
var texture2: texture_2d<f32>;
@group(1) @binding(6)
var sampler2: sampler;

// MIT License. © 2023 Inigo Quilez, Munrocket
// https://gist.github.com/munrocket/30e645d584b5300ee69295e54674b3e4
// https://compute.toys/view/398
fn sdf_rounded_rect(p: vec2f, b: vec2f, r: vec4f) -> f32 {
    var x = r.x;
    var y = r.y;
    x = select(r.z, r.x, p.x > 0.);
    y = select(r.w, r.y, p.x > 0.);
    x = select(y, x, p.y > 0.);
    let q = abs(p) - b + x;
    return min(max(q.x, q.y), 0.) + length(max(q, vec2f(0.))) - x;
}

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    // adjust UVs around the middle of the rect, and convert to pixel
    // coordinates
    let uv = in.uv * in.size * 2.0 - in.size;

    // position offset to account for border
    let border_offset = vec2<f32>(
        (input.offset.w - input.offset.y) / input.inverse_scale_factor, // right - left
        (input.offset.z - input.offset.x) / input.inverse_scale_factor, // bottom - top
    );

    // SDF distance in the inner button area
    // The inner button size is equal to actual size - offset size
    let size = in.size - vec2<f32>(
        (input.offset.y + input.offset.w) / input.inverse_scale_factor, // left + right
        (input.offset.x + input.offset.z) / input.inverse_scale_factor, // top + bottom
    );
    let d_shape = sdf_rounded_rect(
        uv + border_offset,
        size,
        input.border_radius / input.inverse_scale_factor,
    );

    // SDF distance in border area
    let d_border = sdf_rounded_rect(uv, in.size, input.border_radius / input.inverse_scale_factor);

    // define the alpha value. Opaque if within the button or border area,
    // transparent otherwise.
    let alpha = select(1., 0., (d_shape > 0. && d_border > 0.));
    // define the final color. Use `input.border_color` if within border
    // radius, otherwise `input.background_color`.
    var color = select(
        turbulence(in, input.turbulence_color, input.background_color),
//        water(in, input.background_color),
        input.border_color,
        (d_shape > 0. && d_border <= 0.),
    );

    // TODO: Add color smoothing
    //color = (input.background_color * 7 + color) / 8.0;
    if input.value < in.uv.x {
      let color_factor = 1.0;
      let gray = (0.21 * color.r + 0.71 * color.g + 0.07 * color.b) * 0.1;
      //let gray = 0.21 * input.background_color.r + 0.71 * input.background_color.g + 0.07 * input.background_color.b;
      return vec4<f32>(color.rgb * (1.0 - color_factor) + (gray * color_factor), alpha * color.a);
    }

    return vec4<f32>(color.rgb, alpha * color.a);
}

fn rgba_shift(color: vec4<f32>) -> vec4<f32> {
	let shift = color.a - min(color.r, min(color.g, color.b)) - max(color.r, max(color.g, color.b));
	return vec4(shift + color.r, shift + color.g, shift + color.b, color.a);
}

// Found this on GLSL sandbox. I really liked it, changed a few things and made it tileable.
// :)
// by David Hoskins.
// Original water turbulence effect by joltz0r


// Redefine below to see the tiling...
//#define SHOW_TILING

const TAU = 6.28318530718;
const MAX_ITER = 5;

fn turbulence(in: UiVertexOutput, turbulence_color: vec4<f32>, base_color: vec4<f32>) -> vec4<f32> {

    let time = input.time * 0.5 + 23.0;
    // uv should be the 0-1 uv of texture...
    //var uv = in.uv;
    //var uv = in.uv + vec2(in.position.x / 1280.0, in.position.y / 720.0);
    //var uv = in.uv + in.position.xy / input.resolution.xy;
    var uv = in.uv / in.size.yx + in.position.xy / input.resolution.xy;

#ifdef SHOW_TILING
    let p = ((uv * TAU * 2.0) % TAU) - 250.0;
#else
    let p = ((uv * TAU) % TAU) - 250.0;
#endif
    var i = vec2(p);
    var c = 1.0;
    let inten = 0.005;

    for (var n = 0; n < MAX_ITER; n++) {
        let t = time * (1.0 - (3.5 / (f32(n) + 1.0)));
        i = p + vec2(cos(t - i.x) + sin(t + i.y), sin(t - i.y) + cos(t + i.x));
        c += 1.0 / length(vec2(p.x / (sin(i.x + t) / inten), p.y / (cos(i.y + t) / inten)));
    }
    c = c / f32(MAX_ITER);
    c = 1.17 - pow(c, 1.4);
    var color = vec3(pow(abs(c), 8.0));
    color = color * input.power;
	let t_color = color * turbulence_color.rgb;
	color = rgba_shift(vec4(color, 1.0)).rgb;
	color = mix(color * base_color.rgb, t_color, 0.5);

#ifdef SHOW_TILING
    // Flash tile borders...
    let pixel = 2.0 / vec2(input.resolution.x, input.resolution.y);
    uv *= 2.0;
    let f = floor(((input.time * 0.5) % 2.0)); 	// Flash value.
    let first = step(pixel, uv) * f;			// Rule out first screen pixels and flash.
    uv = step(fract(uv), pixel);				// Add one line of pixels per tile.
    color = mix(color, vec3(1.0, 1.0, 0.0), (uv.x + uv.y) * first.x * first.y); // Yellow line
#endif

    return vec4(color, base_color.a);
}

fn avg(color: vec4<f32>) -> f32 {
    return (color.r + color.g + color.b) / 3.0;
}

fn water(in: UiVertexOutput, base_color: vec4<f32>) -> vec4<f32> {

    // Flow Speed, increase to make the water flow faster.
    let speed = 1.0;

    // Water Scale, scales the water, not the background.
    let scale = 0.8;

    // Water opacity, higher opacity means the water reflects more light.
    let opacity = 0.025;


    // Normalized pixel coordinates (from 0 to 1)
    //let uv = fragCoord / resolution;
    let uv = in.uv;
    let scaled_uv = uv * scale;

    // Water layers, layered on top of eachother to produce the reflective effect
    // Add 0.1 to both uv vectors to avoid the layers stacking perfectly and creating a huge unnatural highlight
    var water1 = textureSample(texture0, sampler0, scaled_uv + input.time * 0.02 * speed - 0.1);
    //let water1 = texture(iChannel0, scaled_uv + time * 0.02 * speed - 0.1);
    var water2 = textureSample(texture0, sampler0, scaled_uv + input.time * speed * vec2(-0.02, -0.02) + 0.1);
    //let water2 = texture(iChannel0, scaled_uv.xy + time * speed * vec2(-0.02, -0.02) + 0.1);

    // Water highlights
    var highlights1 = textureSample(texture2, sampler2, scaled_uv + input.time * speed / vec2(-10, 100));
    //let highlights1 = texture(iChannel2, scaled_uv.xy + time * speed / vec2(-10, 100));
    var highlights2 = textureSample(texture2, sampler2, scaled_uv + input.time * speed / vec2( 10, 100));
    //let highlights2 = texture(iChannel2, scaled_uv.xy + time * speed / vec2( 10, 100));

    // Background image
    let background = textureSample(texture1, sampler1, uv + avg(water1) * 0.05);
    //let background = texture(iChannel1, vec2(uv) + avg(water1) * 0.05);

    // Average the colors of the water layers (convert from 1 channel to 4 channel
    water1 = vec4(vec3(avg(water1)), water1.a);
    water2 = vec4(vec3(avg(water2)), water2.a);

    // Average and smooth the colors of the highlight layers
    highlights1 = vec4(vec3(avg(highlights1) / 1.5), highlights1.a);
    highlights2 = vec4(vec3(avg(highlights2) / 1.5), highlights2.a);

    var alpha = opacity;

    if (avg(water1 + water2) > 0.3) {
        alpha = 0.0;
    }

    if (avg(water1 + water2 + highlights1 + highlights2) > 0.75) {
        alpha = 5.0 * opacity;
    }

    // Output to screen
    return (water1 + water2) * alpha + background;
}
