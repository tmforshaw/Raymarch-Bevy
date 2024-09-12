#define_import_path ray_marching::ray

#import ray_marching::shapes::{Shapes, shape_to_sdf};
#import ray_marching::maths::smin;
#import ray_marching::camera::ShaderCamera;

const max_dist: f32 = 80;
const epsilon: f32 = 0.001;

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>,
};

struct GetDistanceInput {
    shapes: Shapes,
    union_type: u32,
    smoothness_val: f32,
    time: f32
};

struct RayMarchOutput {
    object_colour: vec3<f32>,
    dist: f32,
    min_dist: f32,
};

fn ray_march(ray_origin: vec3<f32>, ray_dir: vec3<f32>, get_dist_input: GetDistanceInput) -> RayMarchOutput {
    var ray = Ray(ray_origin, ray_dir);
    var ray_dist = 0.;

    var min_dist = max_dist;

    var march_steps = 0;
    while(ray_dist < max_dist) {
        march_steps++;

        let dist_and_colour = get_distance(ray.origin, get_dist_input);
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

fn get_distance(p: vec3<f32>, get_dist_input: GetDistanceInput) -> vec4<f32> {
    var shape1 = get_dist_input.shapes.shape_one;
    shape1.pos.y *= 2. * sin(get_dist_input.time);

    var shape2 = get_dist_input.shapes.shape_two;
    shape2.pos.x = 2. * cos(get_dist_input.time);

    let shape1_sdf = shape_to_sdf(p, shape1, get_dist_input.union_type);
    let shape2_sdf = shape_to_sdf(p, shape2, get_dist_input.union_type);
    let shape3_sdf = shape_to_sdf(p, get_dist_input.shapes.shape_three, get_dist_input.union_type);
    let shape4_sdf = shape_to_sdf(p, get_dist_input.shapes.shape_four, get_dist_input.union_type);

    var colour = vec3<f32>(1.0, 0.0, 1.0);

    var dist: f32;
    switch get_dist_input.union_type {
        case(0u) {
            dist = smin(smin(smin(shape1_sdf, shape2_sdf, get_dist_input.smoothness_val), shape3_sdf, get_dist_input.smoothness_val), shape4_sdf, get_dist_input.smoothness_val);
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

fn get_ray_dir(camera: ShaderCamera, uv: vec2<f32>) -> vec3<f32> {
    // let camera = material.camera;
   
    let screen_centre = camera.pos + camera.forward * camera.zoom;
    let intersection_point = screen_centre + uv.x * camera.right + uv.y * camera.up;

    return normalize(intersection_point - camera.pos);
}
