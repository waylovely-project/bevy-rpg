use bevy_inspector_egui::widgets::InspectorQuery;
use bevy_rpg::{
    characters::prelude::*,
    d,
    dialog::{ChooseDialogSettings, Dialogs, StyleDefaults, UseDialog},
};

use bevy::{prelude::*, DefaultPlugins};
use bevy_rpg::{
    dialog::{Dialog, DialogIncomingEvent},
    RPGPlugin,
};

type RootUINode = InspectorQuery<Entity, (With<Node>, Without<Parent>)>;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(53.0, 56.0, 57.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RPGPlugin)
        .add_startup_system(start_dialog)
        .add_plugin(bevy_inspector_egui::InspectorPlugin::<RootUINode>::new())

        .run();
}

pub fn start_dialog(commands: Commands, font: Res<AssetServer>) {
    let text_style = TextStyle {
        font: font.load("NotoSans-Regular.ttf"),
        font_size: 24.0,
        color: Color::WHITE,
    };
    let mut dialogs = Dialogs::new(StyleDefaults {
        text: text_style.clone(),
    });
    let yuki: PC = dialogs.single("Yuki").into();
    let ayame: PC = dialogs.single("Ayame").into();
    let everyone: PC = dialogs.multi("Everyone").into();

    dialogs.add([
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
            text_style,
        )),
    ]);

    dialogs.start(commands);
}
