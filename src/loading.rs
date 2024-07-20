use bevy::prelude::*;

use crate::ApplicationState;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct LoadingSet;

pub(super) struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(ApplicationState::Loading),
            loading_display.in_set(LoadingSet),
        );

        app.add_systems(
            OnExit(ApplicationState::Loading),
            loading_clear.in_set(LoadingSet),
        );
    }
}

fn loading_display(mut commands: Commands) {
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "LOADING...",
            TextStyle {
                font_size: 32.,
                ..default()
            },
        ),
        transform: Transform::from_translation(Vec3::new(4., 0., 104.)),
        ..default()
    });
}

fn loading_clear(mut commands: Commands, mut query: Query<Entity, With<Text>>) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
