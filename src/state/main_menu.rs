use bevy::prelude::*;

use crate::{
    save::{SaveLoadMode, SaveLoadUIState},
    state::GameState,
};

#[derive(Component)]
pub struct MainMenuScreen;

#[derive(Component)]
pub enum MainMenuButton {
    NewGame,
    LoadGame,
    Quit,
}

pub fn spawn_main_menu(mut commands: Commands) {
    commands
        .spawn((
            MainMenuScreen,
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.1)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Main menu"),
                TextFont {
                    font_size: FontSize::Px(64.),
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.7, 1.0)),
                Node {
                    margin: UiRect::bottom(Val::Px(60.)),
                    ..default()
                },
            ));

            let buttons = [
                (MainMenuButton::NewGame, "New game"),
                (MainMenuButton::LoadGame, "Load game"),
                (MainMenuButton::Quit, "Quit"),
            ];

            for (button_type, label) in buttons {
                parent
                    .spawn((
                        button_type,
                        Button,
                        Node {
                            width: Val::Px(300.),
                            height: Val::Px(55.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::vertical(Val::Px(8.)),
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
                    ))
                    .with_children(|button_parent| {
                        button_parent.spawn((
                            Text::new(label),
                            TextFont {
                                font_size: FontSize::Px(28.),
                                ..default()
                            },
                            TextColor::WHITE,
                        ));
                    });
            }
        });
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

pub fn handle_main_menu_buttons(
    mut next_state: ResMut<NextState<GameState>>,
    mut ui_state: ResMut<SaveLoadUIState>,
    interaction_query: Query<(&Interaction, &MainMenuButton), Changed<Interaction>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button) in interaction_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        match button {
            MainMenuButton::NewGame => {
                next_state.set(GameState::Loading);
            }
            MainMenuButton::LoadGame => {
                ui_state.active = true;
                ui_state.mode = SaveLoadMode::Load;
            }
            MainMenuButton::Quit => {
                exit.write(AppExit::Success);
            }
        }
    }
}

pub fn handle_main_menu_hover(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MainMenuButton>),
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
