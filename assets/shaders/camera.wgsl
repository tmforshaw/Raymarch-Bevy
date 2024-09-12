#define_import_path ray_marching::camera

#import ray_marching::maths::rotate_position;

struct ShaderCamera {
    pos: vec3<f32>,
    zoom: f32,
    look_at: vec3<f32>,
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

fn move_camera(camera: Camera, pos: vec3<f32>) -> Camera {
    return calculate_camera(pos, camera.look_at + pos, camera.zoom);
}

fn rotate_camera(camera: Camera, rot: vec4<f32>) -> Camera {
    let new_look_at = rotate_position(camera.look_at - camera.pos, rot) + camera.pos;
    
    return calculate_camera(camera.pos, new_look_at, camera.zoom);
}
