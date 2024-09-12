use std::f32::consts::PI;

use bevy::{
    ecs::event::ManualEventReader,
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ButtonState,
    },
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{
    camera::{rotate_camera, ShaderCameraControllerSettings},
    shader_material::ShaderMat,
};

pub const MOUSE_SENSITIVITY: f32 = 0.00012;

// TODO weird bug when looking around, sometimes the screen goes black
pub fn update_mouse(
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
            // let window_scale = 1.;
            pitch -=
                (controller_settings.mouse_sensitivity * event.delta.y * window_scale).to_radians();
            yaw -=
                (controller_settings.mouse_sensitivity * event.delta.x * window_scale).to_radians();

            // Clamp pitch to prevent gimbal lock
            pitch = pitch.clamp(-PI / 2.03, PI / 2.03);
            // pitch = pitch.clamp(-1.57, 1.57);

            // The order matters, otherwise unintended roll will occur
            let rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);

            rotate_camera(&mut mat.camera, rotation);
        }
    }
}

#[derive(Event)]
pub struct MouseGrabEvent {
    pub is_grab: bool,
}

#[derive(Resource, Default)]
pub struct MouseMotionReader {
    motion: ManualEventReader<MouseMotion>,
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
        match button_event.state {
            ButtonState::Pressed => {
                if button_event.button == MouseButton::Left {
                    mouse_grab_event_writer.send(MouseGrabEvent { is_grab: true });
                }
            }
            ButtonState::Released => {}
        }
    }
}
