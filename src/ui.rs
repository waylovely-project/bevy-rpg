use crate::characters::CharacterName;
use crate::dialog::DialogIncomingEvent;
use crate::Dialog;
use bevy::{prelude::*, text};

#[derive(Resource)]
pub struct DialogIter {
    pub dialogs: Vec<Dialog>,
    pub current: usize,
}

impl Iterator for DialogIter {
    type Item = Dialog;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = current + 1;
        Some(self.dialogs[current].clone())
    }
}
#[derive(Resource)]
pub struct UITree {
    pub character_box: Entity,
    pub text_box: Entity,
}
pub fn ui(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let mut char_box = None;
    let mut text_box = None;
    commands
        .spawn(NodeBundle {
            background_color: BackgroundColor(Color::PINK),
            style: Style {
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(80.0), Val::Percent(20.0)),
                justify_content: JustifyContent::Center,
                position: UiRect {
                    right: Val::Percent(20.0),
                    bottom: Val::Percent(10.0),
                    top: Val::Percent(70.0),
                    left: Val::Percent(20.0),
                },
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
    dialog: Res<DialogIter>,
) {
    let dialog = &dialog.dialogs[dialog.current];
    for (id, mut text) in query.iter_mut() {
        if id == tree.character_box {
            match dialog {
                crate::Dialog::Text(dialog) => {
                    *text = dialog
                        .charname()
                        .unwrap_or_else(|| Text::from_section("Unknown", Default::default()))
                }
                crate::Dialog::Choose(choose) => {}
            };
        } else if id == tree.text_box {
            match dialog {
                crate::Dialog::Text(dialog) => *text = dialog.text.clone(),
                crate::Dialog::Choose(choose) => {}
            };
        }
    }
}
