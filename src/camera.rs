use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
use bevy_inspector_egui::InspectorOptions;

#[derive(Debug, AsBindGroup, Clone, Asset, TypePath, ShaderType, Default)]
pub struct ShaderCamera {
    pub pos: Vec3,
    pub rotation: Vec4,
    pub zoom: f32,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
pub struct ShaderCameraInspector {
    pub pos: Vec3,
    pub rotation: Quat,
    pub zoom: f32,
}

// pub fn calculate_camera() -> ShaderCamera {
//     todo!()
// }

impl From<ShaderCameraInspector> for ShaderCamera {
    fn from(shader_camera: ShaderCameraInspector) -> Self {
        Self {
            pos: shader_camera.pos,
            rotation: shader_camera.rotation.into(),
            zoom: shader_camera.zoom,
        }
    }
}

impl From<ShaderCamera> for ShaderCameraInspector {
    fn from(shader_camera: ShaderCamera) -> Self {
        Self {
            pos: shader_camera.pos,
            rotation: Quat::from_vec4(shader_camera.rotation),
            zoom: shader_camera.zoom,
        }
    }
}
