use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::Material2d,
};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions,
    quick::{ResourceInspectorPlugin, WorldInspectorPlugin},
    InspectorOptions,
};
use bevy_screen_diagnostics::{
    ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin,
};
use fullscreen_shader::FullscreenShaderPlugin;

pub mod fullscreen_shader;

// TODO add time to shader so it can automatically move stuff around

pub fn main() {
    let shader_mat = ShaderMat {
        shapes: Shapes {
            shape1: Shape {
                shape_type: 0,
                pos: Vec3::new(0., -1., 1.5),
                size: Vec3::new(1., 0., 0.),
            },
            shape2: Shape {
                shape_type: 1,
                pos: Vec3::new(0., 0., 2.0),
                size: Vec3::new(1.0, 1.0, 1.0),
            },
            ..default()
        },
        union_type: 0,
        smoothness_val: 0.5,
        ..default()
    };

    App::new()
        .add_plugins((FullscreenShaderPlugin {
            shader: shader_mat.clone(),
        },))
        .add_plugins((
            ScreenDiagnosticsPlugin::default(),
            ScreenFrameDiagnosticsPlugin,
            ScreenEntityDiagnosticsPlugin,
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ShaderMatInspector::from(shader_mat))
        .register_type::<ShaderMatInspector>()
        .add_plugins(ResourceInspectorPlugin::<ShaderMatInspector>::default())
        .add_systems(Update, update_mouse)
        .add_systems(
            Update,
            update_shapes.run_if(resource_changed::<ShaderMatInspector>),
        )
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

fn update_shapes(mut materials: ResMut<Assets<ShaderMat>>, inspector_mat: Res<ShaderMatInspector>) {
    for (_handle, mat) in materials.iter_mut() {
        mat.shapes = inspector_mat.shapes.into();
        mat.union_type = inspector_mat.union_type;
        mat.smoothness_val = inspector_mat.smoothness_val;
    }
}

impl Material2d for ShaderMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/fullscreen_shader.wgsl".into()
    }
}

#[derive(Debug, AsBindGroup, Clone, Asset, TypePath, ShaderType, Default)]
#[repr(C)]
pub struct Shape {
    #[uniform[0]]
    pub shape_type: u32,
    #[uniform[0]]
    pub pos: Vec3,
    #[uniform[0]]
    pub size: Vec3,
}

#[derive(AsBindGroup, Debug, Clone, TypePath, ShaderType, Default)]
pub struct Shapes {
    #[uniform(0)]
    shape1: Shape,
    #[uniform(0)]
    shape2: Shape,
    #[uniform(0)]
    shape3: Shape,
    #[uniform(0)]
    shape4: Shape,
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
#[repr(C)]
pub struct ShaderMat {
    #[uniform(0)]
    mouse: Vec2,
    #[uniform(0)]
    shapes: Shapes,
    #[uniform(0)]
    union_type: u32,
    #[uniform(0)]
    smoothness_val: f32,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
pub struct ShapeInspector {
    pub shape_type: u32,
    pub pos: Vec3,
    pub size: Vec3,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
pub struct ShapesInspector {
    shape1: ShapeInspector,
    shape2: ShapeInspector,
    shape3: ShapeInspector,
    shape4: ShapeInspector,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
#[reflect(Resource, InspectorOptions)]
pub struct ShaderMatInspector {
    mouse: Vec2,
    shapes: ShapesInspector,
    union_type: u32,
    smoothness_val: f32,
}

impl From<ShapeInspector> for Shape {
    fn from(inspector: ShapeInspector) -> Self {
        Self {
            shape_type: inspector.shape_type,
            pos: inspector.pos,
            size: inspector.size,
        }
    }
}

impl From<ShapesInspector> for Shapes {
    fn from(inspector: ShapesInspector) -> Self {
        Self {
            shape1: inspector.shape1.into(),
            shape2: inspector.shape2.into(),
            shape3: inspector.shape3.into(),
            shape4: inspector.shape4.into(),
        }
    }
}

impl From<Shape> for ShapeInspector {
    fn from(shape: Shape) -> Self {
        Self {
            shape_type: shape.shape_type,
            pos: shape.pos,
            size: shape.size,
        }
    }
}

impl From<Shapes> for ShapesInspector {
    fn from(shapes: Shapes) -> Self {
        Self {
            shape1: shapes.shape1.into(),
            shape2: shapes.shape2.into(),
            shape3: shapes.shape3.into(),
            shape4: shapes.shape4.into(),
        }
    }
}

impl From<ShaderMat> for ShaderMatInspector {
    fn from(shader_mat: ShaderMat) -> Self {
        Self {
            mouse: shader_mat.mouse,
            shapes: shader_mat.shapes.into(),
            union_type: shader_mat.union_type,
            smoothness_val: shader_mat.smoothness_val,
        }
    }
}
