use bevy::{input::keyboard::KeyCode, prelude::*};

use crate::PauseState;

pub struct PlayerPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (player_setup))
            .add_systems(Update, (player_input, pause));
    }
}

#[derive(Component)]
struct PlayerTag;

#[derive(Component)]
pub struct Pause {
    pub active: bool,
}

#[derive(Bundle)]
struct PlayerBundle {
    tag: PlayerTag,
    sprite: SpriteBundle,
    active: Pause,
}

// player specific systems

fn player_setup(mut commands: Commands, server: Res<AssetServer>) {
    let player_sprite: Handle<Image> = server.load("change.png");
    let player = PlayerBundle {
        tag: PlayerTag,
        sprite: SpriteBundle {
            texture: player_sprite,
            transform: Transform::from_xyz(0., 0., 100.),
            ..default()
        },
        active: Pause { active: false },
    };
    commands.spawn(player);
}

fn player_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut ev_pause: EventWriter<PauseEvent>,
    mut query: Query<(&mut Transform, Entity, &Pause), With<PlayerTag>>,
) {
    for (mut position, _entity, pause) in query.iter_mut() {
        let translate = 250. * time.delta_seconds();

        for key in keys.get_pressed() {
            match key {
                KeyCode::ArrowDown => position.translation.y -= translate,
                KeyCode::ArrowLeft => position.translation.x -= translate,
                KeyCode::ArrowRight => position.translation.x += translate,
                KeyCode::ArrowUp => position.translation.y += translate,
                _ => {}
            }
        }

        for key in keys.get_just_pressed() {
            if key == &KeyCode::KeyW {}
            if key == &KeyCode::KeyA {}
            if key == &KeyCode::KeyS {}
            if key == &KeyCode::KeyD {}
            if key == &KeyCode::Space {}
            if key == &KeyCode::Enter {}
            if key == &KeyCode::Backquote {
                ev_pause.send(PauseEvent);
            }
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
