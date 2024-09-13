#import bevy_pbr::{
    mesh_view_bindings::view, 
    view_transformations::{position_clip_to_world, direction_clip_to_world},
    mesh_functions::{get_model_matrix, mesh_position_local_to_clip, mesh_normal_local_to_world, get_world_from_local},
    forward_io::VertexOutput,
}

#import ray_marching::inputs::material;
#import ray_marching::ray::{get_ray_dir, ray_march, GetDistanceInput, get_ray_dir_with_fragment_camera};
#import ray_marching::lighting::get_light;
#import ray_marching::maths::{centre_and_scale_uv_positions, rotate_position};

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let coords = centre_and_scale_uv_positions(in.position.xy, view.viewport.zw);
    let camera_pos = material.camera.pos;

    let ray_dir = get_ray_dir(material.camera, coords);
    let get_dist_input = GetDistanceInput(material.union_type, material.smoothness_val, material.time);

    let ray_march_out = ray_march(camera_pos, ray_dir, get_dist_input);

    let point_on_surface: vec3<f32> = camera_pos + ray_dir * ray_march_out.dist;
    let light_strength = get_light(point_on_surface, -ray_dir, material.light.pos, get_dist_input);

    var colour: vec3<f32> = ray_march_out.object_colour * material.light.colour * light_strength;

    // Gamma correction
    let gamma = 2.2;
    colour = pow(colour, vec3<f32>(1.0 / gamma));

    return vec4<f32>(colour, 1.0);
}
