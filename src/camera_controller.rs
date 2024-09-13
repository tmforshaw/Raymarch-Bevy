use bevy::{
    ecs::event::ManualEventReader,
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ButtonState,
    },
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::ResourceInspectorPlugin, InspectorOptions,
};

use std::f32::consts::PI;

use crate::shader_material::ShaderMat;

pub const CAMERA_MAX_FOV: f32 = 100.;
pub const CAMERA_MAX_ZOOM_LEVEL: f32 = 4.;
pub const CAMERA_DEFAULT_ZOOM: f32 = 25.;

pub const CAMERA_MOVEMENT_SPEED: f32 = 10.;
pub const MOUSE_SENSITIVITY: f32 = 0.00012;

pub fn update_camera(
    mut shader_mats: ResMut<Assets<ShaderMat>>,
    // mut inspector_mat: ResMut<ShaderMatInspector>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_grab_event_writer: EventWriter<MouseGrabEvent>,
    time: Res<Time>,
    controller_settings: Res<ShaderCameraControllerSettings>,
) {
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

pub fn get_look_at_from_rotation(camera: &mut ShaderCamera, rotation: Quat) -> Vec3 {
    // let forward_dir = Vec3::Z;
    // let forward_dir_quat =
    //     Quat::from_vec4(Vec4::new(forward_dir.x, forward_dir.y, forward_dir.z, 0.)).normalize();
    // let rotated_dir = (rotation.conjugate() * forward_dir_quat * rotation).normalize();

    let transform = Transform::from_translation(camera.pos).with_rotation(rotation);

    let rotated_dir = transform.back();

    Vec3::new(rotated_dir.x, rotated_dir.y, rotated_dir.z) + camera.pos
}

pub fn rotate_camera(camera: &mut ShaderCamera, rotation: Quat) {
    let transform = Transform::from_translation(camera.pos).with_rotation(rotation);

    let (forward, right, up) = (
        transform.back().into(),
        transform.right().into(),
        transform.up().into(),
    );

    camera.forward = forward;
    camera.right = right;
    camera.up = up;

    camera.rotation = rotation.into();
}

pub fn get_camera_axes(pos: Vec3, look_at: Vec3) -> (Vec3, Vec3, Vec3) {
    // let forward = (look_at - pos).normalize();
    // let right = Vec3::Y.cross(forward); // Cross between world up-vector and forward
    // let up = forward.cross(right);

    // (forward, right, up)

    let transform = Transform::from_translation(pos);

    (
        transform.back().into(),
        transform.right().into(),
        transform.up().into(),
    )
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
                camera_rotate_using_mouse,
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

pub fn camera_rotate_using_mouse(
    window: Query<&Window, Changed<Window>>,
    mut motion_reader: ResMut<MouseMotionReader>,
    mouse_motion: Res<Events<MouseMotion>>,
    mut shader_mats: ResMut<Assets<ShaderMat>>,
    controller_settings: Res<ShaderCameraControllerSettings>,
) {
    if window.is_empty() {
        return;
    }

    let window = window.single();
    if window.cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    for event in motion_reader.motion.read(&mouse_motion) {
        for (_handle, mat) in shader_mats.iter_mut() {
            let (mut yaw, mut pitch, _) =
                Quat::from_vec4(mat.camera.rotation).to_euler(EulerRot::YXZ);

            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
            let window_scale = window.height().min(window.width());
            pitch +=
                (controller_settings.mouse_sensitivity * event.delta.y * window_scale).to_radians();
            yaw +=
                (controller_settings.mouse_sensitivity * event.delta.x * window_scale).to_radians();

            // Clamp pitch to prevent gimbal lock
            pitch = pitch.clamp(-PI / 2.01, PI / 2.01);

            // The order matters, otherwise unintended roll will occur
            let rotation = (Quat::from_axis_angle(Vec3::Y, yaw)
                * Quat::from_axis_angle(Vec3::X, pitch))
            .normalize();

            rotate_camera(&mut mat.camera, rotation);
        }
    }
}

pub fn handle_mouse_grab_events(
    mut mouse_grabs: EventReader<MouseGrabEvent>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    for mouse_grab in mouse_grabs.read() {
        let mut primary_window = window.single_mut();

        if mouse_grab.is_grab {
            primary_window.cursor.grab_mode = CursorGrabMode::Confined;
            primary_window.cursor.visible = false;
        } else {
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
        }
    }
}

pub fn handle_mouse_button_events(
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut mouse_grab_event_writer: EventWriter<MouseGrabEvent>,
) {
    for button_event in mouse_button_events.read() {
        if button_event.button == MouseButton::Left && button_event.state == ButtonState::Pressed {
            mouse_grab_event_writer.send(MouseGrabEvent { is_grab: true });
        }
    }
}

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
    #[inspector(min=0., max=CAMERA_MAX_FOV)]
    pub zoom: f32,
    pub rotation: Quat,
}

#[derive(Event)]
pub struct MouseGrabEvent {
    pub is_grab: bool,
}

#[derive(Resource, Default)]
pub struct MouseMotionReader {
    motion: ManualEventReader<MouseMotion>,
}

impl ShaderCamera {
    pub fn modify(&mut self, inspector_cam: ShaderCameraInspector) {
        let look_at = get_look_at_from_rotation(self, inspector_cam.rotation);

        let (forward, right, up) = get_camera_axes(self.pos, look_at);

        self.pos = inspector_cam.pos;
        self.look_at = look_at;
        self.zoom = inspector_cam.zoom * CAMERA_MAX_ZOOM_LEVEL / CAMERA_MAX_FOV;
        self.rotation = inspector_cam.rotation.into();

        self.forward = forward;
        self.right = right;
        self.up = up;
    }
}

impl From<ShaderCamera> for ShaderCameraInspector {
    fn from(shader_camera: ShaderCamera) -> Self {
        Self {
            pos: shader_camera.pos,
            zoom: shader_camera.zoom, //  * CAMERA_MAX_FOV / CAMERA_MAX_ZOOM_LEVEL
            rotation: Quat::from_vec4(shader_camera.rotation),
        }
    }
}
