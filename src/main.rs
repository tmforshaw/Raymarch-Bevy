use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};
use fullscreen_shader::FullscreenShaderPlugin;

pub mod fullscreen_shader;

pub fn main() {
    App::new()
        .add_plugins((FullscreenShaderPlugin {
            shader: ShaderMat {
                mouse: Vec2::splat(0.),
            },
        },))
        .add_systems(Update, update_mouse)
        .run();
}

fn update_mouse(
    window: Query<&Window, Changed<Window>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut materials: ResMut<Assets<ShaderMat>>,
) {
    if window.is_empty() {
        return;
    };
    let resolution = &window.single().resolution;
    for event in cursor_moved_events.read() {
        for (_handle, mat) in materials.iter_mut() {
            mat.mouse = Vec2::new(
                event.position.x / resolution.width(),
                event.position.y / resolution.height(),
            );
        }
    }
}

impl Material2d for ShaderMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/fullscreen_shader.wgsl".into()
    }
}

#[derive(AsBindGroup, TypePath, Debug, Clone, Asset)]
pub struct ShaderMat {
    #[uniform(0)]
    mouse: Vec2,
}
