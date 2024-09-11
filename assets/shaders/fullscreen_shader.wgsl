#import bevy_pbr::{
    mesh_view_bindings::{view, lights}, 
    view_transformations::{position_clip_to_world, direction_clip_to_world},
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_normal_local_to_world, get_world_from_local},
    forward_io::VertexOutput,
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

struct ShaderMat {
    mouse: vec2<f32>,
    shapes: Shapes,
    union_type: u32,
    smoothness_val: f32,
};

@group(2) @binding(0)
var<uniform> material: ShaderMat;

const max_dist: f32 = 80;
const epsilon: f32 = 0.001;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let screen_dim = vec2<f32>(view.viewport.zw);
    let coords = centre_and_scale_uv(in.position.xy / screen_dim, screen_dim);
    let mouse = centre_and_scale_uv(material.mouse.xy, screen_dim);

    var colour: vec3<f32> = vec3<f32>(0., 0., 0.);

    if distance(mouse, coords) < 0.05 {
        colour = vec3<f32>((mouse.yx + 1.) / 4.,  1.0);
    } else {
        colour = ray_march(coords).xyz;
    }

    return vec4<f32>(colour, 1.0);
}

fn ray_march(uv: vec2<f32>) -> vec4<f32> {
    var ray_dist = 0.;
    var ray = CreateCameraRay(uv);

    var min_dist = max_dist;

    var march_steps = 0;
    while(ray_dist < max_dist) {
        march_steps++;

        let dist = get_distance(ray.origin);

        if dist < min_dist {
            min_dist = dist;
        }

        // Have intersected something
        if dist <= epsilon {
            let light_pos = vec3<f32>(-3., -5., -2.);
        
            let point_on_surface: vec3<f32> = ray.origin + ray.dir * dist;
            let normal: vec3<f32> = get_normal(point_on_surface - ray.dir * epsilon);
            let light_dir: vec3<f32> = normalize(light_pos - ray.origin);

            let lighting = saturate(saturate(dot(normal, light_dir)));

            let col = vec3<f32>(1., 0., 1.);

            return vec4<f32>(col * lighting, ray_dist);
        }

        ray.origin += ray.dir * dist;
        ray_dist += dist;
    }

    if min_dist < epsilon * 40. {
        return vec4<f32>(1., 1., 1., ray_dist);
    }

    return vec4<f32>(0., 0., 0., ray_dist);
}

fn get_distance(p: vec3<f32>) -> f32 {
    let shape1_sdf = shape_to_sdf(p, material.shapes.shape1);
    let shape2_sdf = shape_to_sdf(p, material.shapes.shape2);

    switch material.union_type {
        case(0u) {
            return smin(shape1_sdf, shape2_sdf, material.smoothness_val);
        }
        case(1u) {
            return max(shape1_sdf, shape2_sdf);
        }
        case(2u) {
            return max(shape1_sdf, -shape2_sdf);
        }
        case(3u) {
            return max(-shape1_sdf, shape2_sdf);
        }
        default {
            return shape1_sdf;
        }
    }
}

fn shape_to_sdf(p: vec3<f32>, shape: Shape) -> f32 {
    let infinity = 9999999.;
    
    switch shape.shape_type {
        case(0u){
            return sdf_sphere(p, shape.pos, shape.size[0]);
        }
        case(1u){
            return sdf_cube(p, shape.pos, shape.size);
        }
        default {
            return infinity;
        }
    }
}

fn get_normal(p:vec3<f32>) -> vec3<f32> {
    let distance = get_distance(p);
    let e = vec2<f32>(0.01,0.0);

    let normal = distance - vec3<f32>(
        get_distance(p-e.xyy),
        get_distance(p-e.yxy),
        get_distance(p-e.yyx),
    );

    return normalize(normal);
}

// fn get_light(p:vec3<f32>) -> f32 {
//     var light_position = vec3<f32>(2.0,5.0,3.0);
//     let light = normalize(light_position-p);
//     let normal = get_normal(p);

    
//     var dif = clamp(dot(normal,light),0.0,1.0);
//     let d = ray_march(p+normal).w;

//     if d < length(light_position-p) {
//         dif *= 0.1;
//     }

//     return dif;
// }

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>,
};

fn CreateCameraRay(uv: vec2<f32>) -> Ray {
    let origin = vec3<f32>(0., -1.5, -2.);
    let dir = normalize(vec3<f32>(uv, 1.));

    return Ray(origin, dir);
}

fn sdf_sphere(p: vec3<f32>, centre: vec3<f32>, radius: f32) -> f32 {
    return distance(p, centre) - radius;
}

fn sdf_cube(p: vec3<f32>, centre: vec3<f32>, size: vec3<f32>) -> f32 {
    return length(max(abs(p - centre) - size, vec3<f32>(0.0, 0.0, 0.0)));
}

fn smin(a: f32, b: f32, c: f32) -> f32 {
    return min(a, b) - c/6. * (pow(max(c - abs(a - b), 0.) / c, 3.));
}
