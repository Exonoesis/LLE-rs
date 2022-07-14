mod mechanics;
mod audio;
mod entities;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::{AudioPlugin, AudioApp};
use mechanics::{camera::move_camera, input::*};
use audio::{music::{play_level_music, MusicChannel}, sfx::*};
use entities::player::{PlayerBundle, PlayerMovementActions};

/// Loads the LDtk test map with a Camera into the game at the origin (0,0,0).
fn spawn_map(mut commands: Commands, asset_spawner: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    asset_spawner
        .watch_for_changes()
        .expect("Hot Reloading is not working."); //For dev purposes only. REMOVE WHEN GIVING TO PLAYERS!
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_spawner.load("maps/hh_test.ldtk"),
        //transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(spawn_map)
        .add_startup_system(load_player_movement_sound)
        .insert_resource(LevelSelection::Identifier("Test_level".to_string()))
        .add_audio_channel::<MusicChannel>()
        .add_audio_channel::<SFXChannel>()
        .add_event::<Movement>()
        .add_event::<PlayerMovementActions>()
        .add_system(player_input)
        .add_system(move_player)
        .add_system(move_camera)
        .add_system(play_level_music)
        .add_system(play_player_movement_sound)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}
