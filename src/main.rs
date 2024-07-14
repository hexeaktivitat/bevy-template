use bevy::prelude::*;

use bevy_template::ProjectnamePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ViewportPlugin, ProjectnamePlugin))
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

fn field_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {}

// for rapid prototyping before esc menu added
fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
