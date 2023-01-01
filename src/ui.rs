use std::time::{Duration, Instant};

use crate::characters::{text_style, CharacterName};

use crate::text::DrainedText;
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
        Self(Timer::new(Duration::from_millis(60), TimerMode::Once))
    }
}
#[derive(Resource, Default)]
pub struct DialogIter {
    pub dialogs: Vec<Dialog>,
    pub current: usize,
    pub current_char_step: usize,
    /// The current dialog has finished and we can wait for the user to click to continue the next one
    pub finished: bool,
    pub timer: DialogTimer,
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
} /*
  pub fn update_dialog(
      mut char_text: Query<&mut Text, With<CharText>>,
      mut dialog_text: Query<&mut Text, (Without<CharText>, With<DialogText>)>,

      mut dialog_iter: ResMut<DialogIter>,
      mut state: ResMut<State<ActiveState>>,
      interaction: Query<&Interaction, (With<DialogBox>, Changed<Interaction>)>,
  ) {
      if dialog_iter.dialogs.is_empty() {
          return;
      }

      if let Ok(interaction) = interaction.get_single() {
          if *interaction == Interaction::Clicked {
              dialog_iter.current += 1;

              dialog_iter.current_char_step = 0;
          }
      }
      if dialog_iter.dialogs.len() <= dialog_iter.current {
          state.set(ActiveState::Inactive).unwrap();
          return;
      }
      match &dialog_iter.dialogs[dialog_iter.current] {
          Dialog::Text(dialog) => {
              *char_text.single_mut() = dialog
                  .charname()
                  .unwrap_or_else(|| Text::from_section("Unknown", Default::default()));

              let dialog = dialog.clone();
              let text = Some(dialog.text.clone());
              dialog_iter.finished = true;

              dialog_iter.current_char_step = DrainedText::i_just_want_the_length(&dialog.text) - 1;
              println!("{} {}", dialog_iter.current_char_step, dialog_iter.current);

              dialog_iter.finished = true;

              *dialog_text.single_mut() = text.clone().unwrap();
          }
          crate::Dialog::Choose(choose) => {
              warn!("ChooseDialog support fis not implemented yet: {choose:#?}");
          }
      };
  }*/

pub fn update_dialog(
    mut char_text: Query<&mut Text, With<CharText>>,
    mut dialog_text: Query<&mut Text, (Without<CharText>, With<DialogText>)>,

    mut dialog_iter: ResMut<DialogIter>,
    time: Res<Time>,
    mut state: ResMut<State<ActiveState>>,
    interaction: Query<&Interaction, (Changed<Interaction>, With<DialogBox>)>,
) {
 
    dialog_iter.timer.tick(
            time.delta(), /*+ Instant::now().duration_since(time.last_update().unwrap())*/
    );
    
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

    let mut text = None;

    if let Ok(interaction) = interaction.get_single() {
        if *interaction == Interaction::Clicked {
            dialog_iter.dialog_box_button_behavior = match dialog_iter.dialog_box_button_behavior {
                DialogBoxButtonBehavior::FinishWriting => {
                    if let Dialog::Text(dialog) = &dialog_iter.dialogs[dialog_iter.current] {
                        let dialog = dialog.clone();
                        text = Some(dialog.text.clone());
                        dialog_iter.finished = true;
                        dialog_iter.timer.reset();
                        dialog_iter.timer.pause();
                        dialog_iter.current_char_step =
                            DrainedText::i_just_want_the_length(&dialog.text) - 1;
                    }

                    DialogBoxButtonBehavior::SkipNextDialog
                }
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
    }
    let dialog = &dialog_iter.dialogs[dialog_iter.current];

    match &dialog {
        Dialog::Text(dialog) => {
            *char_text.single_mut() = dialog
                .charname()
                .unwrap_or_else(|| Text::from_section("Unknown", Default::default()));

            if text.is_none() {
                if let Dialog::Text(dialog) = &dialog_iter.dialogs[dialog_iter.current] {
                    let drained =
                        DrainedText::drain_from(&dialog.text, dialog_iter.current_char_step + 1);

                    println!("{} {}", dialog_iter.current_char_step, dialog_iter.current);
                    text = Some(drained.text);
                    dialog_iter.finished = drained.len <= dialog_iter.current_char_step + 1;
                    if dialog_iter.finished {
                        dialog_iter.dialog_box_button_behavior =
                            DialogBoxButtonBehavior::SkipNextDialog;
                    }
                }
            } else {
                println!(
                    "OUTSIDE: {} {}",
                    dialog_iter.current_char_step, dialog_iter.current
                );
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
