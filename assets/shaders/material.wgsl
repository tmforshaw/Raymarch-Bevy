#define_import_path ray_marching::material

#import ray_marching::shapes::Shape;
#import ray_marching::lighting::ShaderLight;
#import ray_marching::camera::ShaderCamera;

@group(2) @binding(0)
var<uniform> material: ShaderMat;

struct ShaderMat {
    union_type: u32,
    smoothness_val: f32,
    light: ShaderLight,
    camera: ShaderCamera,
    time: f32,
};

