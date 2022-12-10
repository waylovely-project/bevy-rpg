use bevy_rpg::{
    characters::prelude::*,
    d,
    dialog::{ChooseDialogSettings, UseDialog},
};

use bevy::{
    prelude::{App, EventWriter},
    DefaultPlugins,
};
use bevy_rpg::{
    RPGPlugin,
    dialog::{Dialog, DialogIncomingEvent},
};
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(RPGPlugin);
    app.add_startup_system(start_dialog);
    app.run();
}

pub fn start_dialog(dialog_event: EventWriter<DialogIncomingEvent>) {
    let yuki: PC = Single::from("Fiana").into();
    let ayame: PC = Single::from("Ayame").into();
    let everyone: PC = Multi::from("Everyone").into();
    Dialog::start(
        [
            d((&yuki, "Hiii haii haiii!")),
            d((&ayame, "Hii Yuki!")),
            d((&yuki, "How was your day?")),
            d((&ayame, "Mine is pretty great! How about you?")),
            d((&yuki, "Me too!")),
            d((&everyone, "Yahuuu!!")),
            d((
                ["I like this example", "Great enough", "Not so much"],
                ChooseDialogSettings {
                    question: Some("Do you like this demo?"),
                    use_dialog: UseDialog::None,
                },
            )),
        ]
        .into(),
        dialog_event,
    )
}
