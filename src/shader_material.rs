use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

use crate::{
    camera_controller::{
        get_camera_axes, ShaderCamera, ShaderCameraControllerPlugin, ShaderCameraInspector,
        CAMERA_DEFAULT_ZOOM,
    },
    fullscreen_shader::FullscreenShaderPlugin,
    light::{ShaderLight, ShaderLightInspector},
    shader_loader::ShaderLoaderPlugin,
    shape::{Shape, ShapeInspector, ShapeType},
    UnionType,
};

pub struct ShaderMatPlugin;

impl Plugin for ShaderMatPlugin {
    fn build(&self, app: &mut App) {
        // Initial camera varaibles
        let camera_pos = Vec3::new(0., 0., -5.);
        let camera_rotation = Quat::IDENTITY;
        let (forward, right, up) = get_camera_axes(camera_pos, camera_rotation);

        let shapes = vec![
            Shape {
                shape_type: ShapeType::Sphere.into(),
                pos: Vec3::new(0., 2.5, 0.),
                size: Vec3::new(2.5, 0., 0.),
            },
            Shape {
                shape_type: ShapeType::Cube.into(),
                pos: Vec3::new(0., -0.5, 0.),
                ..default()
            },
            Shape {
                shape_type: ShapeType::Sphere.into(),
                pos: Vec3::new(4., 1., 0.).normalize(),
                size: Vec3::new(1.5, 0.0, 0.),
            },
            Shape {
                shape_type: ShapeType::Cube.into(),
                pos: Vec3::new(4., 1., 0.).normalize(),
                size: Vec3::splat(1.25),
            },
            Shape {
                shape_type: ShapeType::Sphere.into(),
                pos: Vec3::new(4., 1., 0.).normalize(),
                size: Vec3::new(1., 0.0, 0.),
            },
            Shape {
                shape_type: ShapeType::Cube.into(),
                pos: Vec3::new(4., 1., 0.).normalize(),
                size: Vec3::splat(0.75),
            },
            // Shape {
            //     shape_type: ShapeType::Plane.into(),
            //     pos: Vec3::new(0., 1., 0.).normalize(),
            //     size: Vec3::new(-10., 1., 0.),
            // },
        ];

        let shapes_len = shapes.len() as u32;

        let shader_mat = ShaderMat {
            shapes,
            shapes_len,
            union_type: 0,
            smoothness_val: 5.,
            light: ShaderLight {
                pos: Vec3::new(0., 5., 0.),
                colour: Vec3::new(0.8, 0.5, 0.5),
            },
            camera: ShaderCamera {
                pos: camera_pos,
                zoom: CAMERA_DEFAULT_ZOOM,
                rotation: camera_rotation.into(),
                forward,
                right,
                up,
            },
            ..default()
        };

        app.add_plugins(FullscreenShaderPlugin {
            shader: shader_mat.clone(),
        })
        .add_plugins(ShaderCameraControllerPlugin)
        .add_plugins(ShaderLoaderPlugin)
        .insert_resource(ShaderMatInspector::from(shader_mat))
        .register_type::<ShaderMatInspector>()
        .add_plugins(ResourceInspectorPlugin::<ShaderMatInspector>::default())
        .add_systems(
            Update,
            (
                update_shadermat_from_egui.run_if(resource_changed::<ShaderMatInspector>),
                update_time,
            ),
        );
    }
}

fn update_shadermat_from_egui(
    mut shader_mats: ResMut<Assets<ShaderMat>>,
    inspector_mat: Res<ShaderMatInspector>,
) {
    for (_handle, mat) in shader_mats.iter_mut() {
        mat.shapes = inspector_mat
            .shapes
            .clone()
            .into_iter()
            .map(|shape| shape.into())
            .collect::<Vec<_>>();
        mat.union_type = inspector_mat.union_type.into();
        mat.smoothness_val = inspector_mat.smoothness_val;
        mat.light = inspector_mat.light.into();
        mat.camera.modify(inspector_mat.camera);
    }
}

fn update_time(time: Res<Time>, mut shader_mats: ResMut<Assets<ShaderMat>>) {
    for (_handle, mat) in shader_mats.iter_mut() {
        mat.time = time.elapsed_seconds();
    }
}

#[derive(Debug, AsBindGroup, Clone, Asset, TypePath, Default)]
pub struct ShaderTime {
    #[uniform(1)]
    pub time: f32,
}

// Where the fragment shader is in stored in the assets folder
impl Material2d for ShaderMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/fullscreen_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
pub struct ShaderMat {
    #[storage(1, read_only)]
    pub shapes: Vec<Shape>,
    #[uniform(0)]
    pub union_type: u32,
    #[uniform(0)]
    pub smoothness_val: f32,
    #[uniform(0)]
    pub light: ShaderLight,
    #[uniform(0)]
    pub camera: ShaderCamera,
    #[uniform(0)]
    pub time: f32,
    #[uniform(2)]
    pub shapes_len: u32,
}

#[derive(Debug, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct ShaderMatInspector {
    pub shapes: Vec<ShapeInspector>,
    pub union_type: UnionType,
    pub smoothness_val: f32,
    pub light: ShaderLightInspector,
    pub camera: ShaderCameraInspector,
}

impl From<ShaderMat> for ShaderMatInspector {
    fn from(shader_mat: ShaderMat) -> Self {
        Self {
            shapes: shader_mat
                .shapes
                .into_iter()
                .map(|shape| shape.into())
                .collect::<Vec<_>>(),
            union_type: shader_mat.union_type.into(),
            smoothness_val: shader_mat.smoothness_val,
            light: shader_mat.light.into(),
            camera: shader_mat.camera.into(),
        }
    }
}

// // Convert a vector to a sized array, with empty values when the vec is not big enough for the array
// fn vec_to_sized_array<T: Default + Copy, const N: usize>(vec: Vec<T>) -> [T; N] {
//     vec.try_into().unwrap_or_else(|vec: Vec<T>| {
//         vec.into_iter()
//             .enumerate()
//             .take(N)
//             .fold([T::default(); N], |mut acc, (i, elem)| {
//                 acc[i] = elem;

//                 acc
//             })
//     })
// }
