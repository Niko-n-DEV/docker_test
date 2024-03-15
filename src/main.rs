use bevy::prelude::*;
use bevy::render::{Render, RenderApp, RenderSet};
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, TilemapPlugin))
    .add_systems(Startup, (spawn_camera, spawn_tilemap).chain())
    .add_plugins(ExamplePlugin)
    .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_tilemap(mut commands: Commands) {
    let tilemap_entity = commands.spawn_empty().id();
    let map_size =  TilemapSize { x: 144, y: 144 };
    let mut tile_storage = TileStorage::empty(map_size);

    let tile_pos = TilePos { x: 0, y: 0 };

    let tile_entity = commands.spawn(TileBundle {
        position: tile_pos,
        tilemap_id: TilemapId(tilemap_entity),
        ..default()
    }).id();

    tile_storage.set(&tile_pos, tile_entity);

    commands.entity(tilemap_entity).insert(
        TilemapBundle {
            size: map_size,
            storage: tile_storage,
            tile_size: TilemapTileSize { x: 128.0, y: 128.0 },
            ..default()
        },
    );
}

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
    }

    fn finish(&self, app: &mut App) {
        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(Render, test_system.in_set(RenderSet::Queue));
        }
    }
}