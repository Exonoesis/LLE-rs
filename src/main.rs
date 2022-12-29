mod audio;
mod diagnostics;
mod entities;
mod mechanics;
mod plugins;
mod visuals;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_kira_audio::AudioPlugin;
use plugins::developer_helpers::SmartAssetIoPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    //For dev purposes only. REMOVE WHEN GIVING TO PLAYERS!
                    watch_for_changes: true,
                    ..default()
                })
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(SmartAssetIoPlugin),
        )
        .add_plugin(LdtkPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(plugins::levels::LevelsPlugin)
        .add_plugin(plugins::playable_character::PlayableCharacterPlugin)
        .run();
}
