use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use core::hash::Hash;

pub struct FullscreenShaderPlugin<S: Material2d> {
    pub shader: S,
}

impl<S: Material2d> Plugin for FullscreenShaderPlugin<S>
where
    <S as AsBindGroup>::Data: PartialEq<<S as AsBindGroup>::Data> + Eq + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(FullscreenShader(self.shader.clone()))
            .add_plugins(
                DefaultPlugins
                    .set(AssetPlugin {
                        watch_for_changes_override: Some(true),
                        ..default()
                    })
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resolution: WindowResolution::new(300., 168.75),
                            resize_constraints: WindowResizeConstraints {
                                min_width: 300.,
                                min_height: 300.,
                                max_width: f32::INFINITY,
                                max_height: f32::INFINITY,
                            },
                            resizable: true,
                            fit_canvas_to_parent: true,
                            ..default()
                        }),
                        ..default()
                    }),
            )
            .add_plugins(Material2dPlugin::<S>::default())
            .add_systems(Startup, FullscreenShader::<S>::setup)
            .add_systems(Update, FullscreenShader::<S>::update_window);
    }
}

#[derive(Resource)]
pub struct FullscreenShader<S: Material2d>(S);

#[derive(Component)]
struct FullscreenCover;

impl<S: Material2d> FullscreenShader<S> {
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<S>>,
        shader: Res<FullscreenShader<S>>,
    ) {
        // Spawn quad which takes up the entire screen
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle::from(meshes.add(Mesh::from(Rectangle::new(4000., 4000.)))),
                transform: Transform::from_xyz(0., 0., 0.),
                material: materials.add(shader.0.clone()),
                ..default()
            },
            FullscreenCover,
        ));

        // Spawn a camera
        commands.spawn(Camera2dBundle::default());
    }

    fn update_window(
        window: Query<&Window, Changed<Window>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut entities: Query<&mut Mesh2dHandle, With<FullscreenCover>>,
    ) {
        if window.is_empty() {
            return;
        }

        // Create a new Quad which covers the new screen size
        let res = &window.single().resolution;
        let new_mesh =
            Mesh2dHandle::from(meshes.add(Mesh::from(Rectangle::new(res.width(), res.height()))));

        // Replace the old meshes with the rescaled one
        for mut handle in entities.iter_mut() {
            *handle = new_mesh.clone();
        }
    }
}
