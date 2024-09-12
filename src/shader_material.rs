use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

use crate::{
    camera::{
        get_camera_axes, ShaderCamera, ShaderCameraControllerPlugin, ShaderCameraInspector,
        CAMERA_DEFAULT_ZOOM,
    },
    fullscreen_shader::FullscreenShaderPlugin,
    light::{ShaderLight, ShaderLightInspector},
    shape::{Shape, ShapeType, Shapes, ShapesInspector},
    UnionType,
};

pub struct ShaderMatPlugin;

impl Plugin for ShaderMatPlugin {
    fn build(&self, app: &mut App) {
        let camera_pos = Vec3::new(0., 0., -5.);
        let camera_rotation = Quat::IDENTITY;
        let look_at = Vec3::ZERO;
        let (forward, right, up) = get_camera_axes(camera_pos, look_at);

        let shader_mat = ShaderMat {
            shapes: Shapes {
                shape1: Shape {
                    shape_type: ShapeType::Sphere.into(),
                    pos: Vec3::new(0., 2.5, 0.),
                    size: Vec3::new(2.5, 0., 0.),
                },
                shape2: Shape {
                    shape_type: ShapeType::Cube.into(),
                    pos: Vec3::new(0., -0.5, 0.),
                    ..default()
                },
                ..default()
            },
            union_type: 0,
            smoothness_val: 1.,
            light: ShaderLight {
                pos: Vec3::new(0., 5., 0.),
                colour: Vec3::new(0.8, 0.5, 0.5),
            },
            camera: ShaderCamera {
                pos: camera_pos,
                zoom: CAMERA_DEFAULT_ZOOM,
                look_at,
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
        mat.shapes = inspector_mat.shapes.into();
        mat.union_type = inspector_mat.union_type.into();
        mat.smoothness_val = inspector_mat.smoothness_val;
        mat.light = inspector_mat.light.into();
        mat.camera = inspector_mat.camera.into();
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

impl Material2d for ShaderMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/fullscreen_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
pub struct ShaderMat {
    #[uniform(0)]
    pub shapes: Shapes,
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
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct ShaderMatInspector {
    pub shapes: ShapesInspector,
    pub union_type: UnionType,
    pub smoothness_val: f32,
    pub light: ShaderLightInspector,
    pub camera: ShaderCameraInspector,
}

impl From<ShaderMat> for ShaderMatInspector {
    fn from(shader_mat: ShaderMat) -> Self {
        Self {
            shapes: shader_mat.shapes.into(),
            union_type: shader_mat.union_type.into(),
            smoothness_val: shader_mat.smoothness_val,
            light: shader_mat.light.into(),
            camera: shader_mat.camera.into(),
        }
    }
}
