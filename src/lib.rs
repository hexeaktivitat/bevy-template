use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsolePlugin};
use clap::Parser;
use input::{InputPlugin, InputSet};
use player::{PlayerPlugin, PlayerSet};
use ui::{UiPauseSet, UiPlugin, UiSet};

mod input;
mod player;
mod ui;

pub struct ProjectnamePlugin;

impl Plugin for ProjectnamePlugin {
    fn build(&self, app: &mut App) {
        // state setup
        app.insert_state(ApplicationState::Menu)
            .init_state::<ModeState>()
            .init_state::<PauseState>();

        app.configure_sets(
            OnEnter(ApplicationState::InGame),
            PlayerSet
                // .run_if(in_state(ApplicationState::Loading))
                // .run_if(in_state(ApplicationState::Menu))
                .run_if(in_state(ApplicationState::InGame)),
        );
        app.configure_sets(
            OnEnter(ApplicationState::Menu),
            UiSet.run_if(in_state(ApplicationState::Menu)),
        );
        app.configure_sets(
            Update,
            (
                PlayerSet
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(PauseState::Unpaused)),
                InputSet,
                UiPauseSet.run_if(in_state(PauseState::Paused)),
            ),
        );
        app.configure_sets(
            FixedUpdate,
            (UiSet.run_if(in_state(ApplicationState::Menu)),),
        );

        // resources
        // app.insert_resource(ResourceStruct {})

        // plugins
        app.add_plugins((PlayerPlugin, UiPlugin, InputPlugin));

        // systems

        // console comands
    }
}

// console commands

#[derive(Parser, ConsoleCommand)]
#[command(name = "echo")]
struct EchoCommand {
    msg: String,
}

fn echo_command(mut log: ConsoleCommand<EchoCommand>) {
    if let Some(Ok(EchoCommand { msg })) = log.take() {
        log.reply(msg);
    }
}

// game states

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ApplicationState {
    Loading,
    Menu,
    InGame,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum ModeState {
    NotInGame,
    #[default]
    Singleplayer,
}

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    Unpaused,
    Paused,
}
