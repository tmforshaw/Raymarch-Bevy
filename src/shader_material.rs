use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

use crate::{
    camera::{ShaderCamera, ShaderCameraInspector},
    fullscreen_shader::FullscreenShaderPlugin,
    light::{ShaderLight, ShaderLightInspector},
    shape::{Shape, ShapeType, Shapes, ShapesInspector},
    UnionType,
};

pub struct ShaderMatPlugin;

impl Plugin for ShaderMatPlugin {
    fn build(&self, app: &mut App) {
        let shader_mat = ShaderMat {
            shapes: Shapes {
                shape1: Shape {
                    shape_type: ShapeType::Sphere.into(),
                    pos: Vec3::new(0., 1., 0.),
                    ..default()
                },
                shape2: Shape {
                    shape_type: ShapeType::Cube.into(),
                    pos: Vec3::new(0., 0., 0.),
                    ..default()
                },
                ..default()
            },
            union_type: 0,
            smoothness_val: 0.5,
            light: ShaderLight {
                pos: Vec3::new(0., 5., 0.),
                colour: Vec3::new(0.8, 0.5, 0.5),
            },
            camera: ShaderCamera {
                pos: Vec3::new(0., 1.5, -5.),
                rotation: Quat::IDENTITY.into(),
                zoom: 1.,
            },
            ..default()
        };

        app.add_plugins(FullscreenShaderPlugin {
            shader: shader_mat.clone(),
        })
        .insert_resource(ShaderMatInspector::from(shader_mat))
        .register_type::<ShaderMatInspector>()
        .add_plugins(ResourceInspectorPlugin::<ShaderMatInspector>::default())
        .add_systems(
            Update,
            update_shadermat_from_egui.run_if(resource_changed::<ShaderMatInspector>),
        );
    }
}

impl Material2d for ShaderMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/fullscreen_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
#[repr(C)]
pub struct ShaderMat {
    #[uniform(0)]
    pub mouse: Vec2,
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
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct ShaderMatInspector {
    pub mouse: Vec2,
    pub shapes: ShapesInspector,
    pub union_type: UnionType,
    pub smoothness_val: f32,
    pub light: ShaderLightInspector,
    pub camera: ShaderCameraInspector,
}

impl From<ShaderMat> for ShaderMatInspector {
    fn from(shader_mat: ShaderMat) -> Self {
        Self {
            mouse: shader_mat.mouse,
            shapes: shader_mat.shapes.into(),
            union_type: shader_mat.union_type.into(),
            smoothness_val: shader_mat.smoothness_val,
            light: shader_mat.light.into(),
            camera: shader_mat.camera.into(),
        }
    }
}

fn update_shadermat_from_egui(
    mut materials: ResMut<Assets<ShaderMat>>,
    inspector_mat: Res<ShaderMatInspector>,
) {
    for (_handle, mat) in materials.iter_mut() {
        mat.shapes = inspector_mat.shapes.into();
        mat.union_type = inspector_mat.union_type.into();
        mat.smoothness_val = inspector_mat.smoothness_val;
        mat.light = inspector_mat.light.into();
        mat.camera = inspector_mat.camera.into();
    }
}
