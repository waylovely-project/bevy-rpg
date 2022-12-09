use bevy::prelude::*;


pub fn ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle { ..default() });

            parent.spawn(TextBundle { ..default() });
        });
}
