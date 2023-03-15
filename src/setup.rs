use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use crate::{AppState, MazeMapData, MazeMapId, MAP_SIZE, SCREEN_SIZE};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(AppState::Setup).with_system(setup))
            .add_system_set(
                SystemSet::on_update(AppState::Setup), // .with_system(menu_click_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Setup), // .with_system(despawn_everything)
            );
    }
}

fn setup(mut commands: Commands, data: Res<MazeMapData>, mut images: ResMut<Assets<Image>>) {
    commands.spawn(Camera2dBundle::default());

    let mut image = Image::new(
        Extent3d {
            width: (MAP_SIZE * 8) as u32,
            height: (MAP_SIZE * 8) as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data.0.clone(),
        TextureFormat::Rgba8Unorm,
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image = images.add(image);
    let id = image.id();

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(SCREEN_SIZE),
            ..default()
        },
        texture: image,
        ..default()
    });

    commands.insert_resource(MazeMapId(id))
}
