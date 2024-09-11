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
                shape_type: ShapeType::Sphere.into(),
                pos: Vec3::new(0., -1., 0.),
                ..default()
            },
            shape2: Shape {
                shape_type: ShapeType::Cube.into(),
                pos: Vec3::new(0., 0., 0.),
                ..default()
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
        // .add_systems(Startup, change_resource_inspector_size)
        .add_systems(Update, update_mouse)
        .add_systems(
            Update,
            update_shapes.run_if(resource_changed::<ShaderMatInspector>),
        )
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
        mat.union_type = inspector_mat.union_type.into();
        mat.smoothness_val = inspector_mat.smoothness_val;
    }
}

impl Material2d for ShaderMat {
    fn fragment_shader() -> ShaderRef {
        "shaders/fullscreen_shader.wgsl".into()
    }
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
#[reflect(Resource, InspectorOptions)]
pub struct ShaderMatInspector {
    mouse: Vec2,
    shapes: ShapesInspector,
    union_type: UnionType,
    smoothness_val: f32,
}

#[derive(AsBindGroup, Debug, Clone, TypePath, ShaderType, Default)]
pub struct Shapes {
    shape1: Shape,
    shape2: Shape,
    shape3: Shape,
    shape4: Shape,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
pub struct ShapesInspector {
    shape1: ShapeInspector,
    shape2: ShapeInspector,
    shape3: ShapeInspector,
    shape4: ShapeInspector,
}

#[derive(Debug, AsBindGroup, Clone, Asset, TypePath, ShaderType)]
#[repr(C)]
pub struct Shape {
    pub shape_type: u32,
    pub pos: Vec3,
    pub size: Vec3,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
pub struct ShapeInspector {
    pub shape_type: ShapeType,
    pub pos: Vec3,
    pub size: Vec3,
}

// #[derive(Debug, AsBindGroup, Clone, Asset, TypePath, ShaderType)]
// pub struct ShaderLight {
//     #[uniform[0]]
// }

#[derive(Debug, Copy, Clone, Default, Reflect)]
pub enum ShapeType {
    #[default]
    None,
    Sphere,
    Cube,
}

#[derive(Debug, Copy, Clone, Default, Reflect)]
pub enum UnionType {
    #[default]
    MinAll,
    Min1,
    Min2,
    Min3,
    Min4,
    MaxAll,
    Max1,
    Max2,
    Max3,
    Max4,
}

impl Default for Shape {
    fn default() -> Self {
        Self {
            shape_type: u32::default(),
            pos: Vec3::default(),
            size: Vec3::splat(1.),
        }
    }
}

impl From<ShapeType> for u32 {
    fn from(shape_type: ShapeType) -> Self {
        match shape_type {
            ShapeType::None => 0,
            ShapeType::Sphere => 1,
            ShapeType::Cube => 2,
        }
    }
}

impl From<u32> for ShapeType {
    fn from(shape_type: u32) -> Self {
        match shape_type {
            1 => Self::Sphere,
            2 => Self::Cube,
            _ => Self::None,
        }
    }
}

impl From<UnionType> for u32 {
    fn from(union_type: UnionType) -> Self {
        match union_type {
            UnionType::MinAll => 0,
            UnionType::Min1 => 1,
            UnionType::Min2 => 2,
            UnionType::Min3 => 3,
            UnionType::Min4 => 4,
            UnionType::MaxAll => 5,
            UnionType::Max1 => 6,
            UnionType::Max2 => 7,
            UnionType::Max3 => 8,
            UnionType::Max4 => 9,
        }
    }
}

impl From<u32> for UnionType {
    fn from(union_type: u32) -> Self {
        match union_type {
            1 => UnionType::Min1,
            2 => UnionType::Min2,
            3 => UnionType::Min3,
            4 => UnionType::Min4,
            5 => UnionType::MaxAll,
            6 => UnionType::Max1,
            7 => UnionType::Max2,
            8 => UnionType::Max3,
            9 => UnionType::Max4,
            _ => UnionType::MinAll,
        }
    }
}

impl From<ShapeInspector> for Shape {
    fn from(inspector: ShapeInspector) -> Self {
        Self {
            shape_type: inspector.shape_type.into(),
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
            shape_type: shape.shape_type.into(),
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
            union_type: shader_mat.union_type.into(),
            smoothness_val: shader_mat.smoothness_val,
        }
    }
}
