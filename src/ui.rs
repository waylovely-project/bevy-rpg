use crate::characters::CharacterName;
use crate::dialog::DialogIncomingEvent;
use bevy::prelude::*;
#[derive(Resource)]
pub struct UITree {
    pub character_box: Entity,
    pub text_box: Entity,
}
pub fn ui(mut commands: Commands) {
    let mut char_box = None;
    let mut text_box = None;
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::PINK),
            style: Style {
                flex_direction: FlexDirection::Column,

                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            char_box = Some(parent.spawn(TextBundle { ..default() }).id());

            text_box = Some(parent.spawn(TextBundle { ..default() }).id());
        });

    commands.insert_resource(UITree {
        character_box: char_box.unwrap(),
        text_box: text_box.unwrap(),
    })
}

pub fn update_dialog(
    mut query: Query<(Entity, &mut Text)>,
    tree: Res<UITree>,
    mut event: EventReader<DialogIncomingEvent>,
) {
    for event in event.iter() {
        for (id, mut text) in query.iter_mut() {
            if id == tree.character_box {
                match &event.0 {
                    crate::Dialog::Text(dialog) => {
                        *text = dialog
                            .charname()
                            .unwrap_or_else(|| Text::from_section("Unknown", Default::default()))
                    }
                    crate::Dialog::Choose(_) => todo!(),
                };
            } else if id == tree.text_box {
                match &event.0 {
                    crate::Dialog::Text(dialog) => *text = dialog.text.clone(),
                    crate::Dialog::Choose(_) => todo!(),
                };
            }
        }
    }
}
