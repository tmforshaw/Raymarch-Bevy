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

    // Get the direction of the light, and calculate an estimate of the normal using approximate derivatives
    let light = normalize(light_pos - p);
    let normal = get_normal(p, get_dist_input);

    // Diffuse lighting depending on the angle the light makes to the surface
    var diffuse = clamp(dot(normal, light), 0., 1.);

    // Ray march from the surface (with slight pertubation to stop clipping through) to the light, to see if there is anything blocking this ray
    let d = ray_march(p + normal, light, get_dist_input).dist;
    if d < length(light) {
        // Reduce the light if there is something in the way
        diffuse *= 0.1;
    }

    diffuse_final *= diffuse;

    // Calculate the specular highlights by checking how close this ray direction is to a perfectly reflected ray
    let specular = pow(max(dot(view_dir, reflect(-light, normal)), 0.), specular_pow);
    specular_final *= specular;

    // Clamp all the values between 0 and 1, adding the strength of the ambient light
    return clamp(diffuse_final, 0., 1.) + clamp(specular_final, 0., 1.) + ambient_strength;
}

fn get_normal(p: vec3<f32>, get_dist_input: GetDistanceInput) -> vec3<f32> {
    let distance = get_distance(p, get_dist_input).dist;
    let e = vec2<f32>(0.01,0.0); // Epsilon value

    // Sample nearby points, taking their gradient (Grad function approximation)
    let normal = distance - vec3<f32>(
        get_distance(p-e.xyy, get_dist_input).dist,
        get_distance(p-e.yxy, get_dist_input).dist,
        get_distance(p-e.yyx, get_dist_input).dist,
    );

    return normalize(normal);
}
