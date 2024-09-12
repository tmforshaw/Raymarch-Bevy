use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

use crate::{
    mouse::{
        handle_mouse_button_events, handle_mouse_grab_events, update_mouse, MouseGrabEvent,
        MouseMotionReader, MOUSE_SENSITIVITY,
    },
    shader_material::ShaderMat,
    // shader_material::{ShaderMat, ShaderMatInspector},
};

pub const CAMERA_MAX_FOV: f32 = 1.;
pub const CAMERA_MAX_ZOOM_LEVEL: f32 = 1.;
pub const CAMERA_DEFAULT_ZOOM: f32 = 1.;

pub const CAMERA_MOVEMENT_SPEED: f32 = 10.;

#[derive(Resource, Reflect)]
pub struct ShaderCameraControllerSettings {
    pub speed: f32,
    pub mouse_sensitivity: f32,
}

#[derive(Debug, AsBindGroup, Clone, Asset, TypePath, ShaderType, Default)]
pub struct ShaderCamera {
    pub pos: Vec3,
    pub zoom: f32,
    pub look_at: Vec3,
    pub rotation: Vec4,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
#[reflect(InspectorOptions)]
pub struct ShaderCameraInspector {
    pub pos: Vec3,
    pub look_at: Vec3,
    pub rotation: Quat,
    #[inspector(min=0., max=CAMERA_MAX_FOV)]
    pub zoom: f32,
}

pub fn update_camera(
    // mut key_events: EventReader<KeyboardInput>,
    mut shader_mats: ResMut<Assets<ShaderMat>>,
    // mut inspector_mat: ResMut<ShaderMatInspector>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_grab_event_writer: EventWriter<MouseGrabEvent>,
    time: Res<Time>,
    controller_settings: Res<ShaderCameraControllerSettings>,
) {
    // for event in key_events.read() {
    for (_handle, mat) in shader_mats.iter_mut() {
        let (forward, right, up) = (mat.camera.forward, mat.camera.right, mat.camera.up);

        let mut velocity = Vec3::ZERO;

        for key in keys.get_pressed() {
            match key {
                // Movement
                KeyCode::KeyW => velocity += forward,
                KeyCode::KeyS => velocity -= forward,
                KeyCode::KeyD => velocity += right,
                KeyCode::KeyA => velocity -= right,
                KeyCode::Space => velocity += up,
                KeyCode::ControlLeft => velocity -= up,

                // Escape from cursor grab
                KeyCode::Escape => {
                    // Escape from cursor grab
                    mouse_grab_event_writer.send(MouseGrabEvent { is_grab: false });
                }
                _ => {}
            }

            let displacement =
                velocity.normalize_or_zero() * time.delta_seconds() * controller_settings.speed;

            move_camera(&mut mat.camera, displacement);

            // TODO fix this so the inspector can see these values
            // Pointless because this alters the ShaderMat camera as well
            // move_camera_inspector(&mut inspector_mat.camera, displacement);
        }
    }
}

pub fn move_camera(camera: &mut ShaderCamera, move_amount: Vec3) {
    camera.pos += move_amount;
    camera.look_at += move_amount;
}

pub fn move_camera_inspector(camera: &mut ShaderCameraInspector, move_amount: Vec3) {
    camera.pos += move_amount;
    camera.look_at += move_amount;
}

pub fn rotate_camera(camera: &mut ShaderCamera, rotation: Quat) {
    let forward_dir = Vec3::Z;
    let forward_dir_quat =
        Quat::from_vec4(Vec4::new(forward_dir.x, forward_dir.y, forward_dir.z, 0.));
    let rotated_dir = rotation.inverse() * forward_dir_quat * rotation;

    camera.look_at = Vec3::new(rotated_dir.x, rotated_dir.y, rotated_dir.z) + camera.pos;

    // let rot = Vec4::from(rotation);

    // let rotated = forward_dir
    //     + 2. * rot
    //         .xyz()
    //         .cross(rot.xyz().cross(forward_dir) + rot.w * forward_dir);

    // camera.look_at = Vec3::new(rotated.x, rotated.y, rotated.z) * camera.zoom + camera.pos;

    let (forward, right, up) = get_camera_axes(camera.pos, camera.look_at);

    camera.forward = forward;
    camera.right = right;
    camera.up = up;

    camera.rotation = rotation.into();
}

pub fn get_camera_axes(pos: Vec3, look_at: Vec3) -> (Vec3, Vec3, Vec3) {
    let forward = (look_at - pos).normalize();
    let right = Vec3::Y.cross(forward); // Cross between world up-vector and forward
    let up = forward.cross(right);

    (forward, right, up)
}

pub struct ShaderCameraControllerPlugin;

impl Plugin for ShaderCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShaderCameraControllerSettings {
            speed: CAMERA_MOVEMENT_SPEED,
            mouse_sensitivity: MOUSE_SENSITIVITY,
        })
        .init_resource::<MouseMotionReader>()
        .add_plugins(ResourceInspectorPlugin::<ShaderCameraControllerSettings>::default())
        .add_systems(Startup, camera_setup)
        .add_systems(
            Update,
            (
                update_mouse,
                update_camera,
                handle_mouse_grab_events,
                handle_mouse_button_events,
            ),
        )
        .add_event::<MouseGrabEvent>();
    }
}

fn camera_setup(mut mouse_grab_event_writer: EventWriter<MouseGrabEvent>) {
    mouse_grab_event_writer.send(MouseGrabEvent { is_grab: true });
}

impl From<ShaderCameraInspector> for ShaderCamera {
    fn from(shader_camera: ShaderCameraInspector) -> Self {
        let (forward, right, up) = get_camera_axes(shader_camera.pos, shader_camera.look_at);

        Self {
            pos: shader_camera.pos,
            zoom: shader_camera.zoom * (CAMERA_MAX_ZOOM_LEVEL / CAMERA_MAX_FOV),
            look_at: shader_camera.look_at,
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
            look_at: shader_camera.look_at,
            rotation: Quat::from_vec4(shader_camera.rotation),
            zoom: shader_camera.zoom,
        }
    }
}
