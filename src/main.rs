use bevy::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use shader_material::ShaderMatPlugin;

// pub mod camera;
pub mod camera_controller;
pub mod fullscreen_shader;
pub mod light;
// pub mod mouse;
pub mod shader_loader;
pub mod shader_material;
pub mod shape;

pub fn main() {
    App::new()
        .add_plugins(ShaderMatPlugin)
        .add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
            ScreenEntityDiagnosticsPlugin,
        ))
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}

#[derive(Debug, Copy, Clone, Default, Reflect)]
pub enum UnionType {
    #[default]
    MinAll,
    MaxAll,
    Max1,
    Max2,
    Max3,
    Max4,
}

impl From<UnionType> for u32 {
    fn from(union_type: UnionType) -> Self {
        match union_type {
            UnionType::MinAll => 0,
            UnionType::MaxAll => 1,
            UnionType::Max1 => 2,
            UnionType::Max2 => 3,
            UnionType::Max3 => 4,
            UnionType::Max4 => 5,
        }
    }
}

impl From<u32> for UnionType {
    fn from(union_type: u32) -> Self {
        match union_type {
            1 => UnionType::MaxAll,
            2 => UnionType::Max1,
            3 => UnionType::Max2,
            4 => UnionType::Max3,
            5 => UnionType::Max4,
            _ => UnionType::MinAll,
        }
    }
}
