use bevy::prelude::*;

use crate::{ApplicationState, PauseState};

pub struct InputPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (game_input, pause, menu).in_set(InputSet));
        app.add_event::<PauseEvent>().add_event::<MenuEvent>();
    }
}

fn game_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_pause: EventWriter<PauseEvent>,
    mut ev_menu: EventWriter<MenuEvent>,
) {
    for key in keys.get_just_pressed() {
        if key == &KeyCode::Space {}
        if key == &KeyCode::Enter {}
        if key == &KeyCode::Backquote {
            ev_pause.send(PauseEvent);
        }
        if key == &KeyCode::Escape {
            ev_menu.send(MenuEvent);
        }
    }
}

// player specific events
#[derive(Event)]
struct PauseEvent;

fn pause(
    mut ev_pause: EventReader<PauseEvent>,
    state: Res<State<PauseState>>,
    mut next_state: ResMut<NextState<PauseState>>,
) {
    for _ev in ev_pause.read() {
        match state.get() {
            PauseState::Unpaused => next_state.set(PauseState::Paused),
            PauseState::Paused => next_state.set(PauseState::Unpaused),
        }
    }
}

#[derive(Event)]
struct MenuEvent;

fn menu(
    mut ev_menu: EventReader<MenuEvent>,
    state: Res<State<ApplicationState>>,
    mut next_state: ResMut<NextState<ApplicationState>>,
) {
    for _ev in ev_menu.read() {
        match state.get() {
            ApplicationState::Menu => next_state.set(ApplicationState::InGame),
            _ => next_state.set(ApplicationState::Menu),
        }
    }
}
