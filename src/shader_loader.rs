use bevy::{asset::load_internal_asset, prelude::*};

pub const INPUT_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567890);
pub const SHAPE_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567891);
pub const RAY_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567892);
pub const LIGHTING_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567893);
pub const MATHS_SHADER: Handle<Shader> = Handle::weak_from_u128(12345678901234567894);

pub struct ShaderLoaderPlugin;

// Load the shaders into Bevy, allowing them to be used as imports (Path relative to src/ folder)
impl Plugin for ShaderLoaderPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            INPUT_SHADER,
            "../assets/shaders/inputs.wgsl",
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
