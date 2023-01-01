pub mod text;
pub mod dialog;
pub mod ui;

use bevy::prelude::*;

pub use characters::Character;
pub use dialog::{Dialog, Dialogs};
use ui::{on_exit, ui, update_dialog, DialogIter};

pub mod characters;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ActiveState {
    Inactive,
    Active,
}

pub struct RPGPlugin;

impl Plugin for RPGPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(ActiveState::Inactive);
        app.init_resource::<DialogIter>();
        app.add_system_set(SystemSet::on_enter(ActiveState::Active).with_system(ui));

        app.add_system_set(SystemSet::on_update(ActiveState::Active).with_system(update_dialog));

        app.add_system_set(SystemSet::on_exit(ActiveState::Active).with_system(on_exit));
    }
}
/// With [d()]. You can easily create a dialog from many syntaxes!
///
/// Make a character say something:
/// ```rs
/// d((&ayame, "Mine is pretty great! How about you?"))
/// ```
/// Make a chooser dialog that will prompt the player to choose an option:
/// ```rs
/// d((
///     ["I like this example", "Great enough", "Not so much"],
///     ChooseDialogSettings {
///              question: Some("Do you like this demo?"),
///              use_dialog: UseDialog::None,
///     },
///  ))
/// ```
pub fn d<A: Into<Dialog>>(dialog: A) -> Dialog {
    dialog.into()
}
