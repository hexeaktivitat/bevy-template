use bevy::{
    color::palettes::css::{BLACK, DARK_SEA_GREEN, LAVENDER},
    prelude::*,
    sprite::Anchor,
};

use crate::{ApplicationState, PauseState};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UiPauseSet;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ApplicationState::Menu), menu_setup.in_set(UiSet))
            .add_systems(Update, (clear_menu, main_menu, interact_menu).in_set(UiSet));
        app.add_systems(OnEnter(PauseState::Paused), pause_screen.in_set(UiPauseSet))
            .add_systems(OnExit(PauseState::Paused), clear_pause.in_set(UiPauseSet));
    }
}

#[derive(Component)]
enum MenuLayer {
    Main,
}

#[derive(Component)]
enum MenuOptions {
    Start,
    Resume,
    Exit,
}

fn menu_setup(mut commands: Commands, server: Res<AssetServer>) {
    let font_size = 24.0;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    ..default()
                },
                ..default()
            },
            MenuLayer::Main,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::Srgba(DARK_SEA_GREEN)),
                        background_color: BackgroundColor(Color::Srgba(LAVENDER)),
                        ..default()
                    },
                    MenuOptions::Start,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            // font: server.load("fonts/TitilliumWeb-SemiBold.ttf"),
                            font_size,
                            color: Srgba::rgb(0.1, 0.1, 0.1).into(),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::Srgba(DARK_SEA_GREEN)),
                        background_color: BackgroundColor(Color::Srgba(LAVENDER)),
                        ..default()
                    },
                    MenuOptions::Resume,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            // font: server.load("fonts/TitilliumWeb-SemiBold.ttf"),
                            font_size,
                            color: Srgba::rgb(0.1, 0.1, 0.1).into(),
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::Srgba(DARK_SEA_GREEN)),
                        background_color: BackgroundColor(Color::Srgba(LAVENDER)),
                        ..default()
                    },
                    MenuOptions::Exit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            // font: server.load("fonts/TitilliumWeb-SemiBold.ttf"),
                            font_size,
                            color: Srgba::rgb(0.1, 0.1, 0.1).into(),
                            ..default()
                        },
                    ));
                });
        });
}

fn main_menu(
    mut query: Query<(&MenuOptions, &Children), With<Button>>,
    mut text_query: Query<&mut Text>,
) {
    for (menu_options, children) in query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match menu_options {
            MenuOptions::Start => text.sections[0].value = "Start Game".into(),
            MenuOptions::Resume => text.sections[0].value = "Resume Game".into(),
            MenuOptions::Exit => text.sections[0].value = "Quit Game".into(),
        }
    }
}

fn interact_menu(
    mut interaction_query: Query<
        (
            &Interaction,
            &MenuOptions,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    state: Res<State<ApplicationState>>,
    mut next_state: ResMut<NextState<ApplicationState>>,
) {
    for (interaction, menu_options, mut color, mut border, children) in interaction_query.iter_mut()
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => match menu_options {
                MenuOptions::Start => next_state.set(ApplicationState::InGame),
                MenuOptions::Resume => next_state.set(ApplicationState::InGame),
                MenuOptions::Exit => next_state.set(ApplicationState::Exit),
            },
            Interaction::Hovered => {
                *color = BackgroundColor(Color::Srgba(DARK_SEA_GREEN));
                *border = BorderColor(Color::Srgba(BLACK));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::Srgba(LAVENDER));
                *border = BorderColor(Color::Srgba(DARK_SEA_GREEN));
            }
        }
    }
}

fn clear_menu(
    mut commands: Commands,
    mut query: Query<Entity, With<Node>>,
    state: Res<State<ApplicationState>>,
) {
    if state.get() != &ApplicationState::Menu {
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
        }
    }
}

fn pause_screen(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "PAUSED",
            TextStyle {
                font_size: 32.,
                ..default()
            },
        ),
        transform: Transform::from_translation(Vec3::new(4., 0., 104.)),
        ..default()
    });
}

fn clear_pause(mut commands: Commands, mut query: Query<Entity, With<Text>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
