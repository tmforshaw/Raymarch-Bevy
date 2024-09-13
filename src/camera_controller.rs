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

pub const CAMERA_MOVEMENT_SPEED: f32 = 15.;
pub const CAMERA_SPRINTING_SPEED: f32 = CAMERA_MOVEMENT_SPEED * 2.;
pub const MOUSE_SENSITIVITY: f32 = 0.00012;

pub fn camera_move_using_keyboard(
    mut shader_mats: ResMut<Assets<ShaderMat>>,
    // mut inspector_mat: ResMut<ShaderMatInspector>,
    keys: Res<ButtonInput<KeyCode>>,
    mut mouse_grab_event_writer: EventWriter<MouseGrabEvent>,
    time: Res<Time>,
    mut controller_settings: ResMut<ShaderCameraControllerSettings>,
) {
    for (_handle, mat) in shader_mats.iter_mut() {
        // Calculate the directions of motion, this allows for movement independent of where the camera is looking
        let forward = Vec3::new(mat.camera.forward.x, 0., mat.camera.forward.z);
        let right = Vec3::new(mat.camera.forward.z, 0., -mat.camera.forward.x);
        let up = Vec3::Y;

        let mut velocity = Vec3::ZERO;

        // Test just pressed keys
        for key in keys.get_just_pressed() {
            // Begin sprinting
            if key == &KeyCode::ShiftLeft {
                controller_settings.is_sprinting = true;
            }
        }

        // Test pressed keys
        for key in keys.get_pressed() {
            match key {
                // Movement (Modify the velocity in the given camera direction)
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
        }

        // Test just released keys
        for key in keys.get_just_released() {
            // Stop sprinting
            if key == &KeyCode::ShiftLeft {
                controller_settings.is_sprinting = false;
            }
        }

        // Get the speed depending on if the camera is in sprinting mode
        let speed = if controller_settings.is_sprinting {
            controller_settings.sprinting_speed
        } else {
            controller_settings.speed
        };

        // Normalise the velocity and get the displacement, given the time since the last frame, then update the position
        mat.camera.pos += velocity.normalize_or_zero() * time.delta_seconds() * speed;

        // TODO fix this so the inspector can see these values
        // Pointless because this alters the ShaderMat camera as well
        // move_camera_inspector(&mut inspector_mat.camera, displacement);
    }
}

pub fn camera_rotate_using_mouse(
    window: Query<&Window, Changed<Window>>,
    mut motion_reader: ResMut<MouseMotionReader>,
    mouse_motion: Res<Events<MouseMotion>>,
    mut shader_mats: ResMut<Assets<ShaderMat>>,
    controller_settings: Res<ShaderCameraControllerSettings>,
) {
    // Exit the function if the window doesn't exist, or is not grabbing the cursor
    if window.is_empty() {
        return;
    }

    let window = window.single();
    if window.cursor.grab_mode == CursorGrabMode::None {
        return;
    }

    for event in motion_reader.motion.read(&mouse_motion) {
        for (_handle, mat) in shader_mats.iter_mut() {
            // Get the current rotation angles, to be updated
            let (mut yaw, mut pitch, _) =
                Quat::from_vec4(mat.camera.rotation).to_euler(EulerRot::YXZ);

            // Using smallest of width or height ensures equal vertical and horizontal sensitivity
            let window_scale = window.height().min(window.width());
            pitch += (controller_settings.sensitivity * event.delta.y * window_scale).to_radians();
            yaw += (controller_settings.sensitivity * event.delta.x * window_scale).to_radians();

            // Clamp pitch to prevent gimbal lock
            pitch = pitch.clamp(-PI / 2.01, PI / 2.01);

            // Creating a rotation quaternion from the new euler angles
            let rotation = (Quat::from_axis_angle(Vec3::Y, yaw)
                * Quat::from_axis_angle(Vec3::X, pitch))
            .normalize();

            rotate_camera(&mut mat.camera, rotation);
        }
    }
}

pub fn rotate_camera(camera: &mut ShaderCamera, rotation: Quat) {
    // Create a transform to apply the rotation to, the transform starts with no rotation because the new rotation includes the previous rotation as well
    let transform = Transform::from_translation(camera.pos).with_rotation(rotation);

    // Get the camera axes, specifying the backwards direction since our shader uses +Z for the forward direction
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

pub fn get_camera_axes(pos: Vec3, rotation: Quat) -> (Vec3, Vec3, Vec3) {
    // Create a translation and rotation transform, and get its axes
    let transform = Transform::from_translation(pos).with_rotation(rotation);

    // Specifying back instead of forward because of the shader camera using +Z for forward
    (
        transform.back().into(),
        transform.right().into(),
        transform.up().into(),
    )
}

pub struct ShaderCameraControllerPlugin;

impl Plugin for ShaderCameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShaderCameraControllerSettings::default())
            .init_resource::<MouseMotionReader>()
            .add_plugins(ResourceInspectorPlugin::<ShaderCameraControllerSettings>::default())
            .add_systems(Startup, camera_setup)
            .add_systems(
                Update,
                (
                    camera_rotate_using_mouse,
                    camera_move_using_keyboard,
                    handle_mouse_grab_events,
                    handle_mouse_button_events,
                ),
            )
            .add_event::<MouseGrabEvent>();
    }
}

// Initial cursor grab on Startup
fn camera_setup(mut mouse_grab_event_writer: EventWriter<MouseGrabEvent>) {
    mouse_grab_event_writer.send(MouseGrabEvent { is_grab: true });
}

pub fn handle_mouse_grab_events(
    mut mouse_grabs: EventReader<MouseGrabEvent>,
    mut window: Query<&mut Window, With<PrimaryWindow>>,
) {
    for mouse_grab in mouse_grabs.read() {
        let mut primary_window = window.single_mut();

        // Change the cursor grab mode, depending on the event
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
        // Ungrab the cursor when the correct mouse button is clicked
        if button_event.button == MouseButton::Right && button_event.state == ButtonState::Pressed {
            mouse_grab_event_writer.send(MouseGrabEvent { is_grab: true });
        }
    }
}

#[derive(Resource, Reflect)]
pub struct ShaderCameraControllerSettings {
    pub speed: f32,
    pub sprinting_speed: f32,
    pub sensitivity: f32,
    pub is_sprinting: bool,
}

impl Default for ShaderCameraControllerSettings {
    fn default() -> Self {
        Self {
            speed: CAMERA_MOVEMENT_SPEED,
            sprinting_speed: CAMERA_SPRINTING_SPEED,
            sensitivity: MOUSE_SENSITIVITY,
            is_sprinting: false,
        }
    }
}

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
    // Update the ShaderCamera using the inspector camera
    pub fn modify(&mut self, inspector_cam: ShaderCameraInspector) {
        let (forward, right, up) = get_camera_axes(self.pos, inspector_cam.rotation);

        self.pos = inspector_cam.pos;
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
