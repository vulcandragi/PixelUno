use bevy::{
    asset::{Asset, Assets},
    ecs::{
        component::Component,
        name::Name,
        observer::On,
        system::{Commands, ResMut, Single},
    },
    math::{Vec2, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, MeshMaterial2d},
    transform::components::Transform,
    window::Window,
};

use crate::events::Spawn;

#[derive(Component)]
pub struct Table;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TableMaterial {
    #[uniform(0)]
    size: Vec2,
}

impl Table {
    pub fn on_spawn(
        _: On<Spawn<Table>>,
        mut commands: Commands,
        mut meshs: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<TableMaterial>>,
        window: Single<&Window>,
    ) {
        let size = window.size();
        commands
            .spawn(Table)
            .insert(Name("Table".into()))
            .insert(Transform::from_xyz(0., 0., -100.))
            .insert(Mesh2d(meshs.add(Rectangle::from_size(size))))
            .insert(MeshMaterial2d(materials.add(TableMaterial { size })));
    }
}

impl Material2d for TableMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/table.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite_render::AlphaMode2d {
        AlphaMode2d::Mask(1.)
    }
}
