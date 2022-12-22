use crate::characters::{text_style, CharacterName};

use crate::{ActiveState, Dialog};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DialogIter {
    pub dialogs: Vec<Dialog>,
    pub current: usize,
    pub current_char_step: usize,
    /// The current dialog has finished and we can wait for the user to click to continue the next one
    pub finished: bool,
    pub timer: Timer,
}

impl Iterator for DialogIter {
    type Item = Dialog;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = current + 1;
        Some(self.dialogs[current].clone())
    }
}

pub fn ui(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            ButtonBundle {
                background_color: BackgroundColor(Color::PINK),
                style: Style {
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Percent(80.0), Val::Percent(20.0)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect {
                        left: Val::Percent(1.5),
                        right: Val::Percent(1.0),
                        top: Val::Percent(1.5),
                        bottom: Val::Percent(1.0),
                    },
                    position: UiRect {
                        right: Val::Percent(10.0),
                        bottom: Val::Percent(10.0),
                        top: Val::Percent(70.0),
                        left: Val::Percent(10.0),
                    },
                    ..default()
                },
                ..default()
            },
            Name::new("dialog-box"),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle { ..default() }, Name::new("character-box")));

            parent.spawn((TextBundle { ..default() }, Name::new("text-box")));
        });
    commands
        .spawn((NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                position: UiRect {
                    right: Val::Percent(0.0),
                    top: Val::Px(15.0),
                    ..default()
                },
        
                position_type: PositionType::Absolute,
                size: Size::new(Val::Auto, Val::Auto),
                justify_content: JustifyContent::FlexEnd,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },  Name::new("dialog-buttons")))
        .with_children(|parent| {
            //
            let mut button = |image: Handle<Image>| {
                parent
                    .spawn(ButtonBundle {
                        background_color: BackgroundColor(Color::PINK),
                   
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                            margin: UiRect::right(Val::Px(15.0)),
                            padding: UiRect::all(Val::Px(10.0)),
                            ..default()
                        },
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn(ImageBundle {
                            style: Style { size: Size::new(Val::Px(40.0), Val::Auto), ..default()}, 
                             image: UiImage(image),
                             ..default()
                    });
                } 
            );
            };

            
            button(server.load(
                "plugins/com.github.project-flaura.bevy-rpg/icons/scalable/media-skip-forward-symbolic.png",
            ));
            button(server.load(
                "plugins/com.github.project-flaura.bevy-rpg/icons/scalable/playback-speed-symbolic.png",
            ));
              button(server.load(
                "plugins/com.github.project-flaura.bevy-rpg/icons/scalable/eye-open-negative-filled-symbolic.png",
            ));
            button(server.load(
                "plugins/com.github.project-flaura.bevy-rpg/icons/scalable/view-more-symbolic.png",
            ));

            //
        });
}

pub fn update_dialog(
    mut query: Query<(&Name, &mut Text)>,
    button: Query<(&Name, &Interaction)>,
    mut dialog_iter: ResMut<DialogIter>,
    time: Res<Time>,
    mut state: ResMut<State<ActiveState>>,
) {
    if dialog_iter.dialogs.is_empty() {
        return;
    }

    dialog_iter.timer.tick(time.delta());

    if dialog_iter.timer.finished() {
        println!("finished timer");
        dialog_iter.current_char_step += 1;
        dialog_iter.timer.reset();
    }

    let (_, interaction) = button
        .iter()
        .find(|(name, _)| name.as_str() == "dialog-box")
        .unwrap();
    let mut text = None;
    if dialog_iter.finished {
        println!("dialog_iter finished");
        if *interaction == Interaction::Clicked {
            dialog_iter.current += 1;

            dialog_iter.current_char_step = 0;
            dialog_iter.timer.reset();
        } else {
            return;
        }
    } else if *interaction == Interaction::Clicked {
        let dialog = &dialog_iter.dialogs[dialog_iter.current];
        if let Dialog::Text(dialog) = dialog {
            text = Some(dialog.text.clone());
            dialog_iter.finished = true;
            dialog_iter.timer.reset();
        }
    }
    let dialog = &dialog_iter.dialogs[dialog_iter.current];
    if text.is_none() {
        if let Dialog::Text(dialog) = dialog {
            let (indexed_text, finished) = index_text(&dialog.text, dialog_iter.current_char_step);
            text = Some(indexed_text);
            dialog_iter.finished = finished;
        }
    }
    let (_, mut char_text) = query
        .iter_mut()
        .find(|(name, _)| name.as_str() == "character-box")
        .unwrap();
    let dialog = &dialog_iter.dialogs[dialog_iter.current];
    match dialog {
        Dialog::Text(dialog) => {
            *char_text = dialog
                .charname()
                .unwrap_or_else(|| Text::from_section("Unknown", Default::default()));
        }
        crate::Dialog::Choose(choose) => {
            warn!("ChooseDialog support is not implemented yet: {choose:#?}");
        }
    };

    let (_, mut text_text) = query
        .iter_mut()
        .find(|(name, _)| name.as_str() == "text-box")
        .unwrap();
    if dialog_iter.dialogs.len() < dialog_iter.current {
        state.set(ActiveState::Inactive).unwrap();
        return;
    } else if text.is_none() {
        state.set(ActiveState::Inactive).unwrap();
        warn!("Text is empty!");
        return;
    }
    *text_text = text.unwrap();
}

pub fn on_exit(query: Query<(Entity, &Name)>, mut commands: Commands) {
    for (entity, _) in query
        .iter()
        .filter(|(_, name)| name.as_str() == "dialog-box" || name.as_str() == "dialog-buttons")
    {
        commands.entity(entity).despawn_recursive();
    }
}

pub(crate) fn index_text(text: &Text, mut max: usize) -> (Text, bool) {
    let mut vec = vec![];
    let mut finished = false;
    for section in &text.sections {
        if section.value.len() < max {
            max -= section.value.len();
            vec.push(TextSection {
                value: section.value.clone(),
                style: text_style(&section.style),
            });
            finished = true;
        } else {
            vec.push(TextSection {
                value: section.value[0..max].to_string(),
                style: text_style(&section.style),
            });

            finished = false;
        };
    }

    (Text::from_sections(vec), finished)
}

#[cfg(test)]
mod test {

    use bevy::text::{Text, TextSection};

    use super::index_text;

    #[test]
    fn test_index() {
        let text = Text::from_sections([
            TextSection {
                value: "Hiiiiii".to_string(),
                ..Default::default()
            },
            TextSection {
                value: "Fianaaaaa hiiiii".to_string(),
                ..Default::default()
            },
        ]);
        let index_text = index_text(&text, 10).0;
        assert_eq!(index_text.sections[0].value, "Hiiiiii".to_string(),);
        assert_eq!(index_text.sections[1].value, "Fia".to_string());
    }
}
