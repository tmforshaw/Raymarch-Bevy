use bevy::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use camera::{update_camera, update_mouse};
use shader_material::ShaderMatPlugin;

pub mod camera;
pub mod fullscreen_shader;
pub mod light;
pub mod shader_material;
pub mod shape;

// TODO add time to shader so it can automatically move stuff around

pub fn main() {
    App::new()
        .add_plugins(ShaderMatPlugin)
        .add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
            ScreenEntityDiagnosticsPlugin,
        ))
        // .add_plugins(WorldInspectorPlugin::new())
        // .add_systems(Startup, change_resource_inspector_size)
        .add_systems(Update, (update_mouse, update_camera))
        .run();
}

// fn change_resource_inspector_size(mut egui_inspector: EguiContexts) {
//     egui_inspector.ctx_mut()
// .set_style(Arc::from(bevy_inspector_egui::egui::Style {
//     spacing: Spacing {
//         // combo_width

//         default_area_size: bevy_inspector_egui::egui::Vec2::new(2000., 2000.),
//         // window_margin: Margin {
//         //     left: 50.,
//         //     right: 50.,
//         //     top: 50.,
//         //     bottom: 50.,
//         // },
//         ..default()
//     },
//     ..default()
// }));
// }

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
