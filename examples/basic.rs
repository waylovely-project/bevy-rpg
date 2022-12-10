use decora::characters::prelude::*;

use bevy::{
    prelude::{App, EventWriter},
    DefaultPlugins,
};
use decora::{
    dialog::{Dialog, DialogIncomingEvent},
    DecoraPlugin,
};
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(DecoraPlugin);
    app.add_startup_system(start_dialog);
    app.run();
}

pub fn start_dialog(dialog_event: EventWriter<DialogIncomingEvent>) {
    let yuki: PC = Single::from("Fiana").into();
    let ayame: PC = Single::from("Ayame").into();
    let everyone: PC = Multi::from("Everyone").into();
    Dialog::start(
        [
            (&yuki, "Hiii haii haiii!"),
            (&ayame, "Hii Yuki!"),
            (&yuki, "How was your day?"),
            (&everyone, "Great news!"),
        ]
        .into(),
        dialog_event,
    )
}
