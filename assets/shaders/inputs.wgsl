#define_import_path ray_marching::inputs

#import ray_marching::shapes::Shape;
#import ray_marching::lighting::ShaderLight;
#import ray_marching::ray::ShaderCamera;
// #import ray_marching::maths::rotate_position;

@group(2) @binding(0)
var<uniform> material: ShaderMat;

struct ShaderMat {
    union_type: u32,
    smoothness_val: f32,
    light: ShaderLight,
    camera: ShaderCamera,
    time: f32,
};


// struct Camera {
//     pos: vec3<f32>,
//     look_at: vec3<f32>,
//     zoom: f32,
//     forward: vec3<f32>,
//     right: vec3<f32>,
//     up: vec3<f32>,
// }

// fn calculate_camera(pos: vec3<f32>, look_at: vec3<f32>, zoom: f32) -> Camera {
//     let forward = normalize(look_at - pos);
//     let right = cross(vec3<f32>(0., 1., 0.), forward); // Cross between world up-vector and forward
//     let up = cross(forward, right);

//     return Camera(pos, look_at, zoom, forward, right, up);
// }

// fn move_camera(camera: Camera, pos: vec3<f32>) -> Camera {
//     return calculate_camera(pos, camera.look_at + pos, camera.zoom);
// }

// fn rotate_camera(camera: Camera, rot: vec4<f32>) -> Camera {
//     let new_look_at = rotate_position(camera.look_at - camera.pos, rot) + camera.pos;
    
//     return calculate_camera(camera.pos, new_look_at, camera.zoom);
// }
