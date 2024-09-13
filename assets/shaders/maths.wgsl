#define_import_path ray_marching::maths

fn rotate_position(pos: vec3<f32>, rot: vec4<f32>) -> vec3<f32> {
    // Quaternion rotation
    return pos + 2. * cross(rot.xyz, cross(rot.xyz, pos) + rot.w * pos);
}

fn centre_and_scale_uv_positions(uv_pos: vec2<f32>, screen_dim: vec2<f32>) -> vec2<f32> {
    let min_screen_size = min(screen_dim.x, screen_dim.y);
    let max_screen_size = max(screen_dim.x, screen_dim.y);

    let uv = uv_pos / screen_dim;

    // Rescale uv to be screen size independent, and also flip the y-axis to be positive in the upward screen direction
    var coord = vec2<f32>(uv.x, 1. - uv.y) * screen_dim / min_screen_size * 2. - 1.;

    // Readjust to account for the scaling
    let centre_push = (max_screen_size - min_screen_size) / min_screen_size;
    if screen_dim.x > screen_dim.y {
        coord.x -= centre_push;
    } else if screen_dim.x < screen_dim.y {
        coord.y += centre_push;
    };

    return coord;
}

fn smin(a: f32, b: f32, c: f32) -> f32 {
    return min(a, b) - c / 6. * (pow(max(c - abs(a - b), 0.) / c, 3.));
}

