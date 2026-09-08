use bevy::prelude::*;

use crate::{
    save::{SaveLoadMode, SaveLoadUIState},
    state::GameState,
};

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseMenuButton {
    Resume,
    SaveGame,
    LoadGame,
    MainMenu,
    Quit,
}

pub fn spawn_pause_menu(mut commands: Commands) {
    commands
        .spawn((
            PauseMenu,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0., 0., 0., 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: FontSize::Px(42.),
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(30.)),
                    ..default()
                },
            ));

            let buttons = [
                (PauseMenuButton::Resume, "Resume"),
                (PauseMenuButton::SaveGame, "Save game"),
                (PauseMenuButton::LoadGame, "Load game"),
                (PauseMenuButton::MainMenu, "Main menu"),
                (PauseMenuButton::Quit, "Quit"),
            ];

            for (button_type, label) in buttons {
                parent
                    .spawn((
                        button_type,
                        Button,
                        Node {
                            width: Val::Px(250.),
                            height: Val::Px(50.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(5.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
                    ))
                    .with_children(|btn_parent| {
                        btn_parent.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: FontSize::Px(24.),
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });

    info!("Pause menu spawned!");
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("Pause menu despawned!");
}

pub fn handle_pause_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<SaveLoadUIState>,
    interaction_query: Query<(&Interaction, &PauseMenuButton), Changed<Interaction>>,
    mut exit: MessageWriter<AppExit>,
) {
    if ui_state.active {
        return;
    }

    for (interaction, button) in interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            PauseMenuButton::Resume => {
                next_state.set(GameState::Playing);
            }
            PauseMenuButton::SaveGame => {
                ui_state.active = true;
                ui_state.mode = SaveLoadMode::Save;
            }
            PauseMenuButton::LoadGame => {
                ui_state.active = true;
                ui_state.mode = SaveLoadMode::Load;
            }
            PauseMenuButton::MainMenu => {
                next_state.set(GameState::MainMenu);
            }
            PauseMenuButton::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub fn handle_pause_menu_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PauseMenuButton>),
    >,
) {
    for (interaction, mut bg) in &mut interaction_query {
        *bg = match interaction {
            Interaction::Hovered => BackgroundColor(Color::srgba(0.25, 0.25, 0.5, 0.9)),
            Interaction::Pressed => BackgroundColor(Color::srgba(0.35, 0.35, 0.6, 0.9)),
            Interaction::None => BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
        }
    }
}
