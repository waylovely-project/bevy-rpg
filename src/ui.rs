use std::ops::Range;
use std::thread::sleep;
use std::time::Duration;

use crate::characters::{text_style, CharacterName};
use crate::Dialog;
use bevy::{prelude::*};

#[derive(Resource, Default)]
pub struct DialogIter {
    pub dialogs: Vec<Dialog>,
    pub current: usize,
    pub current_char_step: usize,
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
        .spawn((ButtonBundle {
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
        }, Name::new("dialog-box")))
        .with_children(|parent| {
            parent.spawn((TextBundle { ..default() }, Name::new("character-box")));

            parent.spawn((TextBundle { ..default() }, Name::new("text-box") ));
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
  
    mut dialog_iter: ResMut<DialogIter>,
) {
    if dialog_iter.dialogs.len() == 0 { return; }
    let dialog = &dialog_iter.dialogs[dialog_iter.current];
    {
    let (_, mut char_text) = query.iter_mut().find(|( name, _)| name.as_str() == "character-box").unwrap();
   
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
    }
     let (_, mut text_text) = query.iter_mut().find(|( name, _)| name.as_str() == "text-box").unwrap();
    
    match dialog {
        Dialog::Text(dialog) => {
        *text_text = index_text(&dialog.text, dialog_iter.current_char_step);
        },
        _ => {}
    }
   
        
    

    dialog_iter.current_char_step += 1;

    sleep(Duration::from_millis(60));
}


pub fn on_exit(mut query: Query<(Entity, &Name)>, mut commands: Commands) {
    commands
        .entity(
            query
                .iter()
                .find(|(_, name)| name.as_str() == "main-menu")
                .unwrap()
                .0,
        )
        .despawn_recursive();
}

pub(crate) fn index_text(text: &Text, mut max: usize) -> Text {
    let mut vec = vec![];
    for section in &text.sections {
        if section.value.len() < max {
            max -= section.value.len();
            vec.push(TextSection {
                value: section.value.clone(),
                style: text_style(&section.style),
            });
        } else {
            vec.push(TextSection {
                value: section.value[0..max].to_string(),
                style: text_style(&section.style),
            });
        };
    }

    Text::from_sections(vec)
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
        let index_text = index_text(&text, 10);
        assert_eq!(index_text.sections[0].value, "Hiiiiii".to_string(),);
        assert_eq!(index_text.sections[1].value, "Fia".to_string());
    }
}


