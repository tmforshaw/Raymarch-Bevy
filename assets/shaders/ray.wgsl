#define_import_path ray_marching::ray

#import ray_marching::shapes::{Shape, shape_to_sdf, SDFOutput};
#import ray_marching::maths::smin;

@group(2) @binding(1)
var<storage> shapes: array<Shape>;

@group(2) @binding(2)
var<uniform> shapes_len: u32;

const max_dist: f32 = 80.;
const max_steps = 50;
const epsilon: f32 = 0.01;

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>,
};

struct ShaderCamera {
    pos: vec3<f32>,
    zoom: f32,
    rotation: vec4<f32>,
    forward: vec3<f32>,
    right: vec3<f32>,
    up: vec3<f32>,
};

struct GetDistanceInput {
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

    // Keep track of the minimum distance that the ray reached
    var min_dist = max_dist;

    var ray_dist = 0.;
    var march_steps = 0;
    while(ray_dist < max_dist) {
        march_steps++;

        let dist_and_colour = get_distance(ray.origin, get_dist_input);
        let dist = dist_and_colour.x;
        let object_col = dist_and_colour.yzw;

        // Exit the loop if we have traversed too far, or for too many iterations
        if dist < min_dist {
            min_dist = dist;
        }
        if march_steps > max_steps {
            break;
        }

        // Have intersected something
        if dist <= epsilon {
            return RayMarchOutput(object_col, ray_dist, min_dist);
        }

        // Move the ray
        ray.origin += ray.dir * dist;
        ray_dist += dist;
    }

    // Draws an outline of shapes where the ray missed by only a small amount
    if min_dist < 0.1 {
        return RayMarchOutput(vec3<f32>(0.1, 1., 0.7), ray_dist, min_dist);
    }

    // let background = vec3<f32>(0.0, 0.0, 0.);
    let background = (ray_dir + 1.) / 2.;

    return RayMarchOutput(background, ray_dist, min_dist);
}

fn get_distance(p: vec3<f32>, get_dist_input: GetDistanceInput) -> vec4<f32> {
    var dist: f32;
    var closest_or_furthest: f32;

    if get_dist_input.union_type == 0 {
        closest_or_furthest = 9999.;
    } else {
        closest_or_furthest = -9999.;
    }
    
    var colour = vec3<f32>(0.);
    for (var i = 0u; i < shapes_len; i++) {
        var shape_modified = shapes[i];

        if shape_modified.shape_type != 3 { // Isn't a plane
            // Give different motion depending on index in shapes array
            if i == 0 {
                shape_modified.pos.y += 2. * sin(get_dist_input.time);
            } else if i == 1{
                shape_modified.pos.x += 2. * cos(get_dist_input.time * 2.);
            } else {
                shape_modified.pos.x += f32(i) * 3.5 * sin(get_dist_input.time * 1.5 / f32(i) + f32(i) * 0.5); 
                shape_modified.pos.y += f32(i) * 3.5 * cos(get_dist_input.time * 2.5 / f32(i) + f32(i) * 0.5); 
            }
        }

        // Get the distance to this shape, and its colour
        let sdf_out = shape_to_sdf(p, shape_modified, get_dist_input.union_type);

        // If we are finding the minimum of all the shapes, then find closest, otherwise, find furthest
        if get_dist_input.union_type == 0 {
            if sdf_out.dist < closest_or_furthest {
                closest_or_furthest = sdf_out.dist;
                colour = sdf_out.colour;
            }
        } else if sdf_out.dist > closest_or_furthest {
            closest_or_furthest = sdf_out.dist;
            colour = sdf_out.colour;
        }

        // Min or Max the distances, unless this is the first shape
        if i == 0 {
            dist = sdf_out.dist;
        } else {
            switch get_dist_input.union_type {
                case(1u) {
                    dist = max(dist, sdf_out.dist);
                }
                default {
                    dist = smin(dist, sdf_out.dist, get_dist_input.smoothness_val);
                 }
            }
        }
    }

    return vec4<f32>(dist, colour);
}

fn get_ray_dir(camera: ShaderCamera, uv: vec2<f32>) -> vec3<f32> {
    let screen_centre = camera.pos + camera.forward * camera.zoom;
    let intersection_point = screen_centre + uv.x * camera.right + uv.y * camera.up;

    return normalize(intersection_point - camera.pos);
}

// fn get_ray_dir_with_fragment_camera(camera: Camera, uv: vec2<f32>) -> vec3<f32> {
//     let screen_centre = camera.pos + camera.forward * camera.zoom;
//     let intersection_point = screen_centre + uv.x * camera.right + uv.y * camera.up;

//     return normalize(intersection_point - camera.pos);
// }
