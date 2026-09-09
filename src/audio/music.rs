use bevy::prelude::*;

use crate::audio::assets::AudioAssets;

#[derive(Component)]
pub struct MusicTrack;

pub fn start_menu_music(
    mut commands: Commands,
    audio_assets: Option<Res<AudioAssets>>,
    existing: Query<Entity, With<MusicTrack>>,
) {
    for entity in existing {
        commands.entity(entity).despawn();
    }

    let Some(audio) = audio_assets else {
        return;
    };

    commands.spawn((
        MusicTrack,
        AudioPlayer::new(audio.menu_music.clone()),
        PlaybackSettings::LOOP,
    ));
}

pub fn start_battle_music(
    mut commands: Commands,
    audio_assets: Option<Res<AudioAssets>>,
    existing: Query<Entity, With<MusicTrack>>,
) {
    for entity in existing {
        commands.entity(entity).despawn();
    }

    let Some(audio) = audio_assets else {
        return;
    };

    commands.spawn((
        MusicTrack,
        AudioPlayer::new(audio.battle_music.clone()),
        PlaybackSettings::LOOP,
    ));
}

pub fn stop_all_music(mut commands: Commands, existing: Query<Entity, With<MusicTrack>>) {
    for entity in existing {
        commands.entity(entity).despawn();
    }
}
