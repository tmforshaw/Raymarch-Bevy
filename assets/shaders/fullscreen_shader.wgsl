#import bevy_pbr::{
    mesh_view_bindings::view, 
    view_transformations::{position_clip_to_world, direction_clip_to_world},
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_normal_local_to_world, get_world_from_local},
    forward_io::VertexOutput,
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let screen_dim = vec2<f32>(view.viewport.zw);
    let coords = centre_and_scale_uv(in.position.xy / screen_dim, screen_dim);
    let mouse = centre_and_scale_uv(material.mouse.xy, screen_dim);

    let camera_pos = material.camera.pos;
    let look_at = material.camera.pos + vec3<f32>(0., 0., 1.);
    let zoom = 1.;
    // var camera = calculate_camera(material.camera.pos, look_at, zoom);

    // camera = rotate_camera(camera, material.camera.rotation);
    
    let ray_dir = get_ray_dir(coords);
    // let ray_dir = get_ray_dir(camera, coords);

    var colour: vec3<f32> = vec3<f32>(0., 0., 0.);

    if distance(mouse, coords) < 0.025 {
        colour = vec3<f32>((mouse.yx + 1.) / 4.,  1.0);
    } else {
        let ray_march_out = ray_march(camera_pos, ray_dir);

        let point_on_surface: vec3<f32> = camera_pos + ray_dir * ray_march_out.dist;
        let light_strength = get_light(point_on_surface, -ray_dir);

        colour = ray_march_out.object_colour * material.light.colour * light_strength;
    }

    return vec4<f32>(colour, 1.0);
}

// Ray Marching -----------------------------------------------------------------------------------------------------------------------------------------------

const max_dist: f32 = 80;
const epsilon: f32 = 0.001;

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>,
};

struct RayMarchOutput {
    object_colour: vec3<f32>,
    dist: f32,
    min_dist: f32,
};

fn ray_march(ray_origin: vec3<f32>, ray_dir: vec3<f32>) -> RayMarchOutput {
    var ray = Ray(ray_origin, ray_dir);
    var ray_dist = 0.;

    var min_dist = max_dist;

    var march_steps = 0;
    while(ray_dist < max_dist) {
        march_steps++;

        let dist_and_colour = get_distance(ray.origin);
        let dist = dist_and_colour.x;
        let object_col = dist_and_colour.yzw;

        if dist < min_dist {
            min_dist = dist;
        }

        // Have intersected something
        if dist <= epsilon {
            return RayMarchOutput(object_col, ray_dist, min_dist);
        }

        ray.origin += ray.dir * dist;
        ray_dist += dist;
    }

    if min_dist < epsilon * 40. {
        return RayMarchOutput(vec3<f32>(1., 1., 1.), ray_dist, min_dist);
    }

    return RayMarchOutput(vec3<f32>(0., 0., 0.), ray_dist, min_dist);
}

fn get_distance(p: vec3<f32>) -> vec4<f32> {
    var shape1 = material.shapes.shape1;
    shape1.pos.y *= 2. * sin(material.time);

    var shape2 = material.shapes.shape2;
    shape2.pos.x = 2. * cos(material.time);

    let shape1_sdf = shape_to_sdf(p, shape1, material.union_type);
    let shape2_sdf = shape_to_sdf(p, shape2, material.union_type);
    let shape3_sdf = shape_to_sdf(p, material.shapes.shape3, material.union_type);
    let shape4_sdf = shape_to_sdf(p, material.shapes.shape4, material.union_type);

    var colour = vec3<f32>(1.0, 0.0, 1.0);

    var dist: f32;
    switch material.union_type {
        case(0u) {
            dist = smin(smin(smin(shape1_sdf, shape2_sdf, material.smoothness_val), shape3_sdf, material.smoothness_val), shape4_sdf, material.smoothness_val);
        }
        case(1u) {
            dist = max(max(max(shape1_sdf, shape2_sdf), shape3_sdf), shape4_sdf);
        }
        case(2u) {
            dist = max(-shape1_sdf, max(max(shape2_sdf, shape3_sdf), shape4_sdf));
        }
        case(3u) {
            dist = max(-shape2_sdf, max(max(shape1_sdf, shape3_sdf), shape4_sdf));
        }
        case(4u) {
            dist = max(-shape3_sdf, max(max(shape1_sdf, shape2_sdf), shape4_sdf));
        }
        case(5u) {
            dist = max(-shape4_sdf, max(max(shape1_sdf, shape2_sdf), shape3_sdf));
        }
        default {
            dist = shape1_sdf;
        }
    }

    return vec4<f32>(dist, colour);
}

// Shader Material (Input) ------------------------------------------------------------------------------------------------------------------------------------

@group(2) @binding(0)
var<uniform> material: ShaderMat;

struct ShaderMat {
    mouse: vec2<f32>,
    shapes: Shapes,
    union_type: u32,
    smoothness_val: f32,
    light: ShaderLight,
    camera: ShaderCamera,
    time: f32,
};

// Camera -----------------------------------------------------------------------------------------------------------------------------------------------------

struct ShaderCamera {
    pos: vec3<f32>,
    zoom: f32,
    rotation: vec4<f32>,
    forward: vec3<f32>,
    right: vec3<f32>,
    up: vec3<f32>,
}

struct Camera {
    pos: vec3<f32>,
    look_at: vec3<f32>,
    zoom: f32,
    forward: vec3<f32>,
    right: vec3<f32>,
    up: vec3<f32>,
}

fn calculate_camera(pos: vec3<f32>, look_at: vec3<f32>, zoom: f32) -> Camera {
    let forward = normalize(look_at - pos);
    let right = cross(vec3<f32>(0., 1., 0.), forward); // Cross between world up-vector and forward
    let up = cross(forward, right);

    return Camera(pos, look_at, zoom, forward, right, up);
}

fn get_ray_dir(uv: vec2<f32>) -> vec3<f32> {
    let camera = material.camera;
   
    let screen_centre = camera.pos + camera.forward * camera.zoom;
    let intersection_point = screen_centre + uv.x * camera.right + uv.y * camera.up;

    return normalize(intersection_point - camera.pos);
}

fn move_camera(camera: Camera, pos: vec3<f32>) -> Camera {
    return calculate_camera(pos, camera.look_at + pos, camera.zoom);
}

fn rotate_camera(camera: Camera, rot: vec4<f32>) -> Camera {
    let new_look_at = rotate_position(camera.look_at - camera.pos, rot) + camera.pos;
    
    return calculate_camera(camera.pos, new_look_at, camera.zoom);
}

// Lighting ---------------------------------------------------------------------------------------------------------------------------------------------------

struct ShaderLight {
    pos: vec3<f32>,
    colour: vec3<f32>,
}

fn get_light(p: vec3<f32>, view_dir: vec3<f32>) -> f32 {
    var diffuse_final = 1.;
    var specular_final = 1.;

    let specular_pow = 32.;
    let ambient_strength = 0.1;

    let light = normalize(material.light.pos - p);
    let normal = get_normal(p);

    var diffuse = clamp(dot(normal, light), 0., 1.);
    let d = ray_march(p + normal, light).dist;

    if d < length(light) {
        diffuse *= 0.1;
    }

    diffuse_final *= diffuse;

    let specular = pow(max(dot(view_dir, reflect(-light, normal)), 0.), specular_pow);
    specular_final *= specular;

    return clamp(diffuse_final, 0., 1.) + clamp(specular_final, 0., 1.) + ambient_strength;
}

// Shapes -----------------------------------------------------------------------------------------------------------------------------------------------------

struct Shape {
    shape_type: u32,
    pos: vec3<f32>,
    size: vec3<f32>,
};

struct Shapes {
    shape1: Shape,
    shape2: Shape,
    shape3: Shape,
    shape4: Shape,
};

fn shape_to_sdf(p: vec3<f32>, shape: Shape, union_type: u32) -> f32 {
    var infinity: f32;
    if union_type == 0 {
        // Min union type
        infinity = 9999999.;
    } else {
        // Max union type
        infinity = -9999999.;
    }
    
    switch shape.shape_type {
        case(1u){
            return sdf_sphere(p, shape.pos, shape.size[0]);
        }
        case(2u){
            return sdf_cube(p, shape.pos, shape.size);
        }
        default {
            return infinity;
        }
    }
}

fn sdf_sphere(p: vec3<f32>, centre: vec3<f32>, radius: f32) -> f32 {
    return distance(p, centre) - radius;
}

fn sdf_cube(p: vec3<f32>, centre: vec3<f32>, size: vec3<f32>) -> f32 {
    return length(max(abs(p - centre) - size, vec3<f32>(0.0, 0.0, 0.0)));
}

// Maths functions --------------------------------------------------------------------------------------------------------------------------------------------

fn get_normal(p:vec3<f32>) -> vec3<f32> {
    let distance = get_distance(p).x;
    let e = vec2<f32>(0.01,0.0); // Epsilon value

    // Sample nearby points, taking their gradient (Grad function approximation)
    let normal = distance - vec3<f32>(
        get_distance(p-e.xyy).x,
        get_distance(p-e.yxy).x,
        get_distance(p-e.yyx).x,
    );

    return normalize(normal);
}

fn rotate_position(pos: vec3<f32>, rot: vec4<f32>) -> vec3<f32> {
    return pos + 2. * cross(rot.xyz, cross(rot.xyz, pos) + rot.w * pos);
}

fn centre_and_scale_uv(uv: vec2<f32>, screen_dim: vec2<f32>) -> vec2<f32> {
    let min_screen_size = min(screen_dim.x, screen_dim.y);
    let max_screen_size = max(screen_dim.x, screen_dim.y);

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
