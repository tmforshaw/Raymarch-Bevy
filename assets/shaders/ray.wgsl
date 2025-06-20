#define_import_path ray_marching::ray

#import ray_marching::shapes::{Shape, shape_to_sdf, SDFOutput};
#import ray_marching::maths::smin;

@group(2) @binding(1)
var<storage> shapes: array<Shape>;

@group(2) @binding(2)
var<uniform> shapes_len: u32;

const max_dist: f32 = 300.;
const max_steps = 150;
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

struct DistanceOutput {
    dist: f32,
    colour: vec3<f32>,
    shape_type: u32,
}

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
    var total_ray_dist = ray_dist;
    var march_steps = 0;

    while(total_ray_dist < max_dist) {
        march_steps++;

        let dist_output = get_distance(ray.origin, get_dist_input);
        var dist = dist_output.dist;
        let object_col = dist_output.colour;
        let shape_type = dist_output.shape_type;

        // Set the minimum distance reached if this distance is smaller
        if dist < min_dist {
            min_dist = dist;
        }

        // Exit the loop if we have traversed for too many iterations
        if march_steps > max_steps {
            break;
        }

        // Have intersected something
        if dist <= epsilon {
            // Intersected Portal
            if shape_type == 4 {
                ray.dir = vec3<f32>(0., 0., -1.);
                ray.origin = vec3<f32>(2., 0., 2.) + ray_dir * 0.5;
                // dist = 0.5;

                // total_ray_dist += dist;
                // ray_dist = 0.1;

                min_dist = 0.15;
                continue;
                // return RayMarchOutput(vec3<f32>(0., 0., 1.), 1,0.001);
            }
        
            return RayMarchOutput(object_col, ray_dist, min_dist);
        }

        // Move the ray
        ray.origin += ray.dir * dist;
        ray_dist += dist;
        total_ray_dist += dist;
    }

    // Draws an outline of shapes where the ray missed by only a small amount
    if min_dist < 0.1 {
        return RayMarchOutput(vec3<f32>(1., 1., 1.), ray_dist, min_dist);
    }

    let sky_col = vec3<f32>(0.1, 0.2, 0.7);
    let bottom_sky_col = vec3<f32>(0.3, 0.2, 0.5);

    let background = mix(bottom_sky_col, sky_col,  (1.5 + dot(vec3<f32>(0.,1.,0.), ray.dir)));
 
    // let background = vec3<f32>(0.0, 0.0, 0.);
    // let background = (ray_dir + 1.) / 2.;

    return RayMarchOutput(background, ray_dist, min_dist);
}

fn get_distance(p: vec3<f32>, get_dist_input: GetDistanceInput) -> DistanceOutput {
    var dist: f32;
    var colour = vec3<f32>(0.);
    var shape_type: u32 = 0;

    var closest_or_furthest: f32;
    if get_dist_input.union_type == 0 {
        closest_or_furthest = 9999.;
    } else {
        closest_or_furthest = -9999.;
    }
    
    for (var i = 0u; i < shapes_len; i++) {
        var shape_modified = shapes[i];


        if shape_modified.shape_type != 3 { // Isn't a plane
            // Give different motion depending on index in shapes array
            if i == 0 {
                shape_modified.pos.y += 2. * sin(get_dist_input.time);
            } else if i == 1 {
                shape_modified.pos.x += 2. * cos(get_dist_input.time * 2.);
            } else if i == 2 {
                shape_modified.pos.x += f32(i) * 3.5 * sin(get_dist_input.time * 1.5 / f32(i) + f32(i) * 0.5); 
                shape_modified.pos.y += f32(i) * 3.5 * cos(get_dist_input.time * 2.5 / f32(i) + f32(i) * 0.5); 
            } else {
                // Portal

            }
        }

        // Get the distance to this shape, and its colour
        let sdf_out = shape_to_sdf(p, shape_modified, get_dist_input.union_type, get_dist_input.time);

        // If we are finding the minimum of all the shapes, then find closest, otherwise, find furthest
        if get_dist_input.union_type == 0 {
            if sdf_out.dist < closest_or_furthest {
                closest_or_furthest = sdf_out.dist;
                colour = sdf_out.colour;
                shape_type = shape_modified.shape_type;
            }
        } else if sdf_out.dist > closest_or_furthest {
            closest_or_furthest = sdf_out.dist;
            colour = sdf_out.colour;
            shape_type = shape_modified.shape_type;
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

    return DistanceOutput(dist, colour, shape_type);
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
