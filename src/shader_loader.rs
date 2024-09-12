use bevy::{asset::load_internal_asset, prelude::*};

pub const CAMERA_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567890);
pub const SHAPE_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567891);
pub const RAY_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567892);
pub const MATERIAL_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567893);
pub const LIGHTING_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567894);
pub const MATHS_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567895);

pub struct ShaderLoaderPlugin;

impl Plugin for ShaderLoaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            CAMERA_SHADER,
            "../assets/shaders/camera.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SHAPE_SHADER,
            "../assets/shaders/shapes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            RAY_SHADER,
            "../assets/shaders/ray.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            MATERIAL_SHADER,
            "../assets/shaders/material.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            LIGHTING_SHADER,
            "../assets/shaders/lighting.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            MATHS_SHADER,
            "../assets/shaders/maths.wgsl",
            Shader::from_wgsl
        );
    }
}
