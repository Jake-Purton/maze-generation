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
                SystemSet::on_update(AppState::Setup)
                    .with_system(click_system)
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Setup), // .with_system(despawn_everything)
            );
    }
}

#[derive(Resource)]
pub struct StartEnd {
    pub start: Option<(usize, usize)>,
    pub end: Option<(usize, usize)>,
}

fn setup (
    mut commands: Commands, 
    data: Res<MazeMapData>, 
    mut images: ResMut<Assets<Image>>,
) {
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

    commands.insert_resource(MazeMapId(id));

}

fn click_system (
    buttons: Res<Input<MouseButton>>,
    id: Res<MazeMapId>,
    mut images: ResMut<Assets<Image>>,
    windows: Res<Windows>,
    mut start_end: ResMut<StartEnd>,
    mut app_state: ResMut<State<AppState>>
) {
    if buttons.just_pressed(MouseButton::Left) {

        let handle = Handle::weak(id.0);

        if let Some(image) = images.get_mut(&handle) {

            let window = windows.get_primary().unwrap();

            if let Some(position) = window.cursor_position() {

                let position = (position.x as usize * MAP_SIZE / 100, (window.height() - position.y) as usize * MAP_SIZE / 100);
                let index = position.0 + (position.1 * 8 * MAP_SIZE);
                if image.data[(index * 4) + 3] == 0 {

                    image.data[(index * 4)] = 50;
                    image.data[(index * 4) + 1] = 100;
                    image.data[(index * 4) + 2] = 255;
                    image.data[(index * 4) + 3] = 255;

                    if start_end.start.is_some() {
                        start_end.end = Some(position);
                        app_state.set(AppState::AStar).unwrap();
                    } else {
                        start_end.start = Some(position);
                    }
                }           
            }
        }
    }
}