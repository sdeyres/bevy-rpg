pub mod game_over;
pub mod game_state;
pub mod loading;
pub mod pause;

use bevy::prelude::*;

pub use game_state::GameState;

use crate::{characters::{config::CharactersList, spawn::CharactersListResource}, map::generate::MapReady};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Loading), loading::spawn_loading_screen)
            .add_systems(
                Update,
                (check_assets_loaded, loading::animate_loading)
                    .run_if(in_state(GameState::Loading)),
            )
            .add_systems(OnExit(GameState::Loading), loading::despawn_loading_screen)
            .add_systems(OnEnter(GameState::Paused), pause::spawn_pause_menu)
            .add_systems(OnExit(GameState::Paused), pause::despawn_pause_menu)
            .add_systems(
                Update,
                toggle_pause
                    .run_if(in_state(GameState::Playing).or_else(in_state(GameState::Paused))),
            )
            .add_systems(
                OnEnter(GameState::GameOver),
                game_over::spawn_game_over_screen,
            )
            .add_systems(
                OnExit(GameState::GameOver),
                (
                    game_over::despawn_game_over_screen,
                    game_over::cleanup_game_world,
                ),
            )
            .add_systems(
                Update,
                game_over::handle_restart_input.run_if(in_state(GameState::GameOver)),
            );
    }
}

fn check_assets_loaded(
    characters_list_res: Option<Res<CharactersListResource>>,
    characters_list: Res<Assets<CharactersList>>,
    map_ready: Option<Res<MapReady>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(res) = characters_list_res else {
        return;
    };

    if characters_list.get(&res.handle).is_some() && map_ready.is_some() {
        info!("Assets loaded, transitioning to Playing!");
        next_state.set(GameState::Playing);
    }
}

fn toggle_pause(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match current_state.get() {
            GameState::Playing => {
                info!("Game paused!");
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                info!("Game resumed!");
                next_state.set(GameState::Playing);
            }
            GameState::Loading => {}
            GameState::GameOver => {}
        }
    }
}
