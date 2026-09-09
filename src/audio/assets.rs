use std::path::MAIN_SEPARATOR;

use bevy::prelude::*;

#[derive(Resource)]
pub struct AudioAssets {
    pub menu_music: Handle<AudioSource>,
    pub battle_music: Handle<AudioSource>,
    pub spell_generic: Handle<AudioSource>,
    pub spell_fire: Handle<AudioSource>,
    pub enemy_shoot: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub player_death: Handle<AudioSource>,
    pub enemy_death: Handle<AudioSource>,
    pub pickup: Handle<AudioSource>,
    pub button_click: Handle<AudioSource>,
    pub jump: Handle<AudioSource>,
}

pub fn load_audio_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        menu_music: asset_server.load(format!("audio{0}music{0}menu_ambient.mp3", MAIN_SEPARATOR)),
        battle_music: asset_server.load(format!("audio{0}music{0}battle_theme.ogg", MAIN_SEPARATOR)),
        spell_generic: asset_server.load(format!("audio{0}sfx{0}spell_01.ogg", MAIN_SEPARATOR)),
        spell_fire: asset_server.load(format!("audio{0}sfx{0}fire.wav", MAIN_SEPARATOR)),
        enemy_shoot: asset_server.load(format!("audio{0}sfx{0}spell_enemy.ogg", MAIN_SEPARATOR)),
        hit: asset_server.load(format!("audio{0}sfx{0}hit.wav", MAIN_SEPARATOR)),
        player_death: asset_server.load(format!("audio{0}sfx{0}death_player.mp3", MAIN_SEPARATOR)),
        enemy_death: asset_server.load(format!("audio{0}sfx{0}death_enemy.mp3", MAIN_SEPARATOR)),
        pickup: asset_server.load(format!("audio{0}sfx{0}pickup.wav", MAIN_SEPARATOR)),
        button_click: asset_server.load(format!("audio{0}sfx{0}button_click.wav", MAIN_SEPARATOR)),
        jump: asset_server.load(format!("audio{0}sfx{0}jump.wav", MAIN_SEPARATOR)),
    });
}
