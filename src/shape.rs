use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderType},
};
use bevy_inspector_egui::InspectorOptions;

#[derive(Debug, Copy, Clone, Default, Reflect)]
pub enum ShapeType {
    #[default]
    None,
    Sphere,
    Cube,
    Plane,
}

#[derive(AsBindGroup, Debug, Clone, TypePath, ShaderType, Default)]
pub struct Shapes {
    pub shape1: Shape,
    pub shape2: Shape,
    pub shape3: Shape,
    pub shape4: Shape,
}

#[derive(Debug, Copy, Clone, Asset, Reflect, Resource, InspectorOptions, Component, Default)]
pub struct ShapesInspector {
    pub shape1: ShapeInspector,
    pub shape2: ShapeInspector,
    pub shape3: ShapeInspector,
    pub shape4: ShapeInspector,
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
            ShapeType::Plane => 3,
        }
    }
}

impl From<u32> for ShapeType {
    fn from(shape_type: u32) -> Self {
        match shape_type {
            1 => Self::Sphere,
            2 => Self::Cube,
            3 => Self::Plane,
            _ => Self::None,
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
