#define_import_path ray_marching::shapes

struct Shape {
    shape_type: u32,
    pos: vec3<f32>,
    size: vec3<f32>,
};

struct Shapes {
    shape_one: Shape,
    shape_two: Shape,
    shape_three: Shape,
    shape_four: Shape,
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
    // let displacement = (perlin_noise_3d(p) + 1.) / 2.;
    
    return distance(p, centre) - radius;
}

fn sdf_cube(p: vec3<f32>, centre: vec3<f32>, size: vec3<f32>) -> f32 {
    return length(max(abs(p - centre) - size, vec3<f32>(0.0, 0.0, 0.0)));
}
