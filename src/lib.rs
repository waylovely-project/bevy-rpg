pub mod dialog;
pub mod ui;

use bevy::prelude::*;

pub use characters::Character;
pub use dialog::Dialog;
use dialog::DialogIncomingEvent;

pub mod characters;

pub struct RPGPlugin;

impl Plugin for RPGPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(ui::ui);
        app.add_system(ui::update_dialog);
        app.add_event::<DialogIncomingEvent>();
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
