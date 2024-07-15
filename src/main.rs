#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

use bevy_template::ProjectnamePlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            ViewportPlugin,
            ProjectnamePlugin,
        ))
        // .add_systems(Update, (close_on_esc))
        .run();
}

struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup)
            .add_systems(Update, field_setup);
    }
}

fn camera_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}

fn field_setup(commands: Commands, _asset_server: Res<AssetServer>) {}
