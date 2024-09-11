use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
use bevy_inspector_egui::InspectorOptions;

#[derive(Debug, AsBindGroup, Clone, Asset, TypePath, ShaderType, Default)]
pub struct ShaderLight {
    pub pos: Vec3,
    pub colour: Vec3,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
pub struct ShaderLightInspector {
    pub pos: Vec3,
    pub colour: Vec3,
}

impl From<ShaderLightInspector> for ShaderLight {
    fn from(shader_light: ShaderLightInspector) -> Self {
        Self {
            pos: shader_light.pos,
            colour: shader_light.colour,
        }
    }
}

impl From<ShaderLight> for ShaderLightInspector {
    fn from(shader_light: ShaderLight) -> Self {
        Self {
            pos: shader_light.pos,
            colour: shader_light.colour,
        }
    }
}
