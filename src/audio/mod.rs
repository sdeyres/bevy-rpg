mod assets;
mod music;
mod sfx;

use bevy::prelude::*;

pub use sfx::SfxKind;

use crate::state::GameState;

pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(sfx::on_sfx)
            .add_systems(
                Startup,
                (assets::load_audio_assets, music::start_menu_music).chain(),
            )
            .add_systems(OnEnter(GameState::MainMenu), music::start_menu_music)
            .add_systems(OnEnter(GameState::Playing), music::start_battle_music)
            .add_systems(OnEnter(GameState::GameOver), music::stop_all_music);
    }
}
