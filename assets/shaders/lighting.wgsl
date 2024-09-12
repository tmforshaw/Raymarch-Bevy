#define_import_path ray_marching::lighting

#import ray_marching::ray::{get_distance, ray_march, GetDistanceInput};

struct ShaderLight {
    pos: vec3<f32>,
    colour: vec3<f32>,
}

fn get_light(p: vec3<f32>, view_dir: vec3<f32>, light_pos: vec3<f32>, get_dist_input: GetDistanceInput) -> f32 {
    var diffuse_final = 1.;
    var specular_final = 1.;

    let specular_pow = 16.;
    let ambient_strength = 0.1;

    let light = normalize(light_pos - p);
    let normal = get_normal(p, get_dist_input);

    var diffuse = clamp(dot(normal, light), 0., 1.);
    let d = ray_march(p + normal, light, get_dist_input).dist;

    if d < length(light) {
        diffuse *= 0.1;
    }

    diffuse_final *= diffuse;

    let specular = pow(max(dot(view_dir, reflect(-light, normal)), 0.), specular_pow);
    specular_final *= specular;

    return clamp(diffuse_final, 0., 1.) + clamp(specular_final, 0., 1.) + ambient_strength;
}

fn get_normal(p: vec3<f32>, get_dist_input: GetDistanceInput) -> vec3<f32> {
    let distance = get_distance(p, get_dist_input).x;
    let e = vec2<f32>(0.01,0.0); // Epsilon value

    // Sample nearby points, taking their gradient (Grad function approximation)
    let normal = distance - vec3<f32>(
        get_distance(p-e.xyy, get_dist_input).x,
        get_distance(p-e.yxy, get_dist_input).x,
        get_distance(p-e.yyx, get_dist_input).x,
    );

    return normalize(normal);
}
