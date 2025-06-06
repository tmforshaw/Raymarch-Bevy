#define_import_path ray_marching::shapes

struct Shape {
    shape_type: u32,
    pos: vec3<f32>,
    size: vec3<f32>,
};

struct SDFOutput {
    dist: f32,
    colour: vec3<f32>,
    shape_type: u32,
};

fn repeat(p: vec3<f32>, c: f32, r: f32) -> vec3<f32> {
    return fract(p / c) - r / 2. * c;
}

fn shape_to_sdf(p: vec3<f32>, shape: Shape, union_type: u32, time: f32) -> SDFOutput {
    var infinity: f32;
    if union_type == 0 {
        // Min union type
        infinity = 9999.;
    } else {
        // Max union type
        infinity = -9999.;
    }

    // Depending on the shape type, return its SDF
    switch shape.shape_type {
        case(1u){
            return sdf_sphere(p, shape.pos, shape.size.x);
        }
        case(2u){
            return sdf_cube(p, shape.pos, shape.size);
        }
        case(3u) {
            return sdf_plane(p, shape.pos, shape.size.xy);
        }
        case (4u) {
            return sdf_portal(p, shape.pos, shape.size);
        }
        default {
            return SDFOutput(infinity, vec3<f32>(0., 0., 0.), 0);
        }
    }
}

fn sdf_sphere(p: vec3<f32>, centre: vec3<f32>, radius: f32) -> SDFOutput {
    let colour = vec3<f32>(1., 0., 1.);
    
    return SDFOutput(distance(p, centre) - radius, colour, 1);
}

fn sdf_cube(p: vec3<f32>, centre: vec3<f32>, size: vec3<f32>) -> SDFOutput {
    let colour = vec3<f32>(0., 1., 1.);

    return SDFOutput(length(max(abs(p - centre) - size, vec3<f32>(0.0, 0.0, 0.0))), colour, 2);
}

fn sdf_plane(p: vec3<f32>, normal: vec3<f32>, size: vec2<f32>) -> SDFOutput {
    let colour = vec3<f32>(0.1, 0.5, 0.1);

    return SDFOutput(abs(dot(p, normal) - size[0]) - size[1], colour, 3);
}

fn sdf_portal(p: vec3<f32>, centre: vec3<f32>, size: vec3<f32>) -> SDFOutput {
    let colour = vec3<f32>(1., 0., 0.);

    return SDFOutput(length(max(abs(p - centre) - size, vec3<f32>(0.0, 0.0, 0.0))), colour, 4);
}
