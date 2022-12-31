use std::time::{Duration, Instant};

use crate::characters::{text_style, CharacterName};

use crate::{ActiveState, Dialog};
use bevy::prelude::*;

pub enum DialogBoxButtonBehavior {
    ///
    FinishWriting,
    ///
    SkipNextDialog,
}

impl Default for DialogBoxButtonBehavior {
    fn default() -> Self {
        Self::FinishWriting
    }
}
#[derive(Deref, DerefMut)]
pub struct DialogTimer(Timer);

impl Default for DialogTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(6000), TimerMode::Once))
    }
}
#[derive(Resource, Default)]
pub struct DialogIter {
    pub dialogs: Vec<Dialog>,
    pub current: usize,
    pub current_char_step: usize,
    /// The current dialog has finished and we can wait for the user to click to continue the next one
    pub finished: bool,
    pub timer: Timer,
    pub dialog_box_button_behavior: DialogBoxButtonBehavior,
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
            DialogBox,
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle { ..default() }, CharText));

            parent.spawn((TextBundle { ..default() }, DialogText));
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
        },  DialogMenu))
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
    mut char_text: Query<&mut Text, With<CharText>>,
    mut dialog_text: Query<&mut Text, (Without<CharText>, With<DialogText>)>,

    mut dialog_iter: ResMut<DialogIter>,
    time: Res<Time>,
    mut state: ResMut<State<ActiveState>>,
    interaction: Query<&Interaction, With<DialogBox>>,
) {
    if !dialog_iter.timer.finished() && !dialog_iter.timer.paused() && dialog_iter.current_char_step > 0 {
    dialog_iter
        .timer
        .tick(time.delta() /*+ Instant::now().duration_since(time.last_update().unwrap())*/);
    }
    if dialog_iter.dialogs.is_empty() {
        return;
    }
    if dialog_iter.dialogs.len() <= dialog_iter.current {
        state.set(ActiveState::Inactive).unwrap();
        return;
    }

    if dialog_iter.timer.finished() {
        println!("finished timer");
        dialog_iter.current_char_step += 1;
        dialog_iter.timer.reset();
    }

    let interaction = interaction.single();
    let mut text = None;

    if *interaction == Interaction::Clicked {
          dialog_iter.dialog_box_button_behavior = match dialog_iter.dialog_box_button_behavior {
            DialogBoxButtonBehavior::FinishWriting => {
                if let Dialog::Text(dialog) = &dialog_iter.dialogs[dialog_iter.current] {
                    text = Some(dialog.text.clone());
                    dialog_iter.finished = true;
                    dialog_iter.timer.reset();
                    dialog_iter.timer.pause();
                }

              DialogBoxButtonBehavior::SkipNextDialog
            },
            DialogBoxButtonBehavior::SkipNextDialog => {
                dialog_iter.current += 1;

                dialog_iter.current_char_step = 0;
                dialog_iter.timer.reset();
                if dialog_iter.timer.paused() {
                    dialog_iter.timer.unpause();
                }
               DialogBoxButtonBehavior::FinishWriting
            }
        };
    }

    let dialog = &dialog_iter.dialogs[dialog_iter.current];

    match &dialog {
        Dialog::Text(dialog) => {
            *char_text.single_mut() = dialog
                .charname()
                .unwrap_or_else(|| Text::from_section("Unknown", Default::default()));

            if text.is_none() {
                if let Dialog::Text(dialog) = &dialog_iter.dialogs[dialog_iter.current] {
                    let (indexed_text, finished) =
                        index_text(&dialog.text, dialog_iter.current_char_step + 1);

                    println!("{} {}", dialog_iter.current_char_step, dialog_iter.current);
                    text = Some(indexed_text);
                    dialog_iter.finished = finished;
                    if finished {
                        dialog_iter.dialog_box_button_behavior = DialogBoxButtonBehavior::SkipNextDialog;
                    }
                }
            }

            *dialog_text.single_mut() = text.clone().unwrap();
        }
        crate::Dialog::Choose(choose) => {
            warn!("ChooseDialog support fis not implemented yet: {choose:#?}");
        }
    };

    if dialog_iter.dialogs.len() < dialog_iter.current {
        state.set(ActiveState::Inactive).unwrap();
        return;
    } else if text.is_none() {
        state.set(ActiveState::Inactive).unwrap();
        warn!("Text is empty!");
        return;
    }
}

pub fn on_exit(
    dialog_box: Query<Entity, With<DialogBox>>,
    dialog_menu: Query<Entity, With<DialogMenu>>,
    mut commands: Commands,
) {
    commands.entity(dialog_box.single()).despawn_recursive();
    commands.entity(dialog_menu.single()).despawn_recursive();
}

pub(crate) fn index_text(text: &Text, mut max: usize) -> (Text, bool) {
   /*let mut vec = vec![];
    let mut len = 0;
    for (nth, section) in text.sections.iter().enumerate() {

   
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

    (Text::from_sections(vec), finished) */ 

    todo!()
}

#[derive(Component)]
///
pub struct CharText;
#[derive(Component)]
/// Identifier for the
pub struct DialogText;

#[derive(Component)]
pub struct DialogBox;
#[derive(Component)]
pub struct DialogMenu;
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
        let (text, finished) = index_text(&text, 10);
        assert_eq!(text.sections[0].value, "Hiiiiii".to_string(),);
        assert_eq!(text.sections[1].value, "Fia".to_string());
        assert_eq!(finished, false);
        let text = index_text(&text, 1).0;
        assert_eq!(text.sections[0].value, "H".to_string(),);
        assert_eq!(finished, false);
   

    }
}
