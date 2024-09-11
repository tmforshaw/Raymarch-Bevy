use std::f32::consts::PI;

use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::shader_material::{ShaderMat, ShaderMatInspector};

pub const CAMERA_MAX_FOV: f32 = 180.;
pub const CAMERA_MAX_ZOOM_LEVEL: f32 = 10.;
pub const CAMERA_DEFAULT_ZOOM: f32 = 18.;

#[derive(Debug, AsBindGroup, Clone, Asset, TypePath, ShaderType, Default)]
pub struct ShaderCamera {
    pub pos: Vec3,
    pub zoom: f32,
    pub rotation: Vec4,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
#[reflect(InspectorOptions)]
pub struct ShaderCameraInspector {
    pub pos: Vec3,
    pub rotation: Quat,
    #[inspector(min=0., max=CAMERA_MAX_FOV)]
    pub zoom: f32,
}

pub fn update_camera(
    mut key_events: EventReader<KeyboardInput>,
    mut shader_mats: ResMut<Assets<ShaderMat>>,
    mut inspector_mat: ResMut<ShaderMatInspector>,
) {
    let speed = 0.25;

    for event in key_events.read() {
        for (_handle, mat) in shader_mats.iter_mut() {
            match event.state {
                ButtonState::Pressed => match event.key_code {
                    KeyCode::KeyW => mat.camera.pos += speed * mat.camera.forward,
                    KeyCode::KeyS => mat.camera.pos -= speed * mat.camera.forward,
                    KeyCode::KeyD => mat.camera.pos += speed * mat.camera.right,
                    KeyCode::KeyA => mat.camera.pos -= speed * mat.camera.right,
                    KeyCode::Space => mat.camera.pos += speed * mat.camera.up,
                    KeyCode::ControlLeft => mat.camera.pos -= speed * mat.camera.up,
                    // KeyCode::KeyQ => {
                    //     let new_rotation =
                    //         Quat::from_vec4(mat.camera.rotation) * Quat::from_rotation_y(PI / 4.);

                    //     let (forward, right, up) = get_camera_axes(mat.camera.pos, new_rotation);

                    //     mat.camera.rotation = new_rotation.into();
                    //     mat.camera.forward = forward;
                    //     mat.camera.right = right;
                    //     mat.camera.up = up;
                    // }
                    // KeyCode::KeyE => {
                    //     let new_rotation = Quat::from_vec4(mat.camera.rotation)
                    //         * Quat::from_rotation_x(PI * speed);

                    //     let (forward, right, up) = get_camera_axes(mat.camera.pos, new_rotation);

                    //     mat.camera.rotation = new_rotation.into();
                    //     mat.camera.forward = forward;
                    //     mat.camera.right = right;
                    //     mat.camera.up = up;
                    // }
                    _ => {}
                },
                ButtonState::Released => {}
            }

            // Update the inspector camera position
            inspector_mat.camera.pos = mat.clone().camera.pos;
        }
    }
}

pub fn update_mouse(
    window: Query<&Window, Changed<Window>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut shader_mats: ResMut<Assets<ShaderMat>>,
) {
    if window.is_empty() {
        return;
    };
    let resolution = &window.single().resolution;
    for event in cursor_moved_events.read() {
        for (_handle, mat) in shader_mats.iter_mut() {
            mat.mouse = Vec2::new(
                event.position.x / resolution.width(),
                event.position.y / resolution.height(),
            );
        }
    }
}

// Returns forward, right, up directions for the current camera position and its look_at pos
// pub fn get_camera_axes(pos: Vec3, rotation: Quat, forward: Option<Vec3>) -> (Vec3, Vec3, Vec3) {
//     let forward_dir = forward.unwrap_or(Vec3::Z);
pub fn get_camera_axes(pos: Vec3, rotation: Quat) -> (Vec3, Vec3, Vec3) {
    let forward_dir = Vec3::Z;
    let forward_dir_quat =
        Quat::from_vec4(Vec4::new(forward_dir.x, forward_dir.y, forward_dir.z, 0.));

    let rotated_dir = forward_dir_quat * rotation;

    let look_at = Vec3::new(rotated_dir.x, rotated_dir.y, rotated_dir.z) + pos;

    let forward = (look_at - pos).normalize();
    let right = Vec3::Y.cross(forward); // Cross between world up-vector and forward
    let up = forward.cross(right);

    (forward, right, up)
}

impl From<ShaderCameraInspector> for ShaderCamera {
    fn from(shader_camera: ShaderCameraInspector) -> Self {
        let (forward, right, up) = get_camera_axes(
            shader_camera.pos,
            shader_camera.rotation,
            // (shader_camera.forward),
        );

        Self {
            pos: shader_camera.pos,
            zoom: shader_camera.zoom * (CAMERA_MAX_ZOOM_LEVEL / CAMERA_MAX_FOV),
            rotation: shader_camera.rotation.into(),
            forward,
            right,
            up,
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
