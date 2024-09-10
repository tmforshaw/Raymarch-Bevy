#import bevy_pbr::{
    mesh_view_bindings::view, 
    forward_io::VertexOutput,
}

struct ShaderMat {
    mouse: vec2<f32>,
};

@group(2) @binding(0)
var<uniform> material: ShaderMat;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let screen_dim = vec2<f32>(view.viewport.zw);
    let coords = centre_and_scale_uv(in.position.xy / screen_dim, screen_dim);
    let mouse = centre_and_scale_uv(material.mouse.xy, screen_dim);

    var colour: vec3<f32>;

    if distance(mouse, coords) < 0.05 {
        colour = vec3<f32>((mouse.yx + 1.) / 4.,  1.0);
    } else if coords.x * coords.x + coords.y * coords.y < 1. {
        colour = vec3<f32>(coords, 1.0);
    } else {
        colour = vec3<f32>(0.,0.,0.);
    }

    return vec4<f32>(colour, 1.0);
}

fn centre_and_scale_uv(uv: vec2<f32>, screen_dim: vec2<f32>) -> vec2<f32> {
    let min_screen_size = min(screen_dim.x, screen_dim.y);
    let max_screen_size = max(screen_dim.x, screen_dim.y);

    var coord = (uv * screen_dim / min_screen_size * 2.0) - 1.0; // Centre the UV coords and scale to resolution size

    // Readjust to account for the scaling
    let centre_push = (max_screen_size - min_screen_size) / min_screen_size;
    if screen_dim.x > screen_dim.y {
        coord.x -= centre_push;
    } else if screen_dim.x < screen_dim.y {
        coord.y -= centre_push;
    };

    return coord;
}
