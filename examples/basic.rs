use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_rpg::{
    characters::prelude::*,
    d,
    dialog::{ChooseDialogSettings, Dialogs, StyleDefaults, UseDialog},
    ActiveState,
};

use bevy::{prelude::*, DefaultPlugins};
use bevy_rpg::RPGPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::rgb(53.0, 56.0, 57.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(camera)
        .add_plugin(RPGPlugin)
        // We can't directly run a dialog on the first frame, since that'll emit StateAlreadyQueued error with Bevy.
        // So we would like to go to the main menu first
        // But actually, we can directly hook the start_dialog to the SystemSet::on_enter(ActiveState::Active)
        // But at least the main menu is there for people to replay the dialog.
        .add_system_set(SystemSet::on_enter(ActiveState::Inactive).with_system(on_enter))
        .add_system_set(SystemSet::on_update(ActiveState::Inactive).with_system(on_update))
        .add_system_set(SystemSet::on_exit(ActiveState::Inactive).with_system(on_exit))
        .add_system_set(SystemSet::on_enter(ActiveState::Active).with_system(start_dialog))
        .add_plugin(WorldInspectorPlugin)
        .run();
}

pub fn on_enter(mut commands: Commands, server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("main-menu"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::PINK),
                        style: Style {
                            padding: UiRect::all(Val::Px(15.0)),
                            ..default()
                        },

                        ..default()
                    },
                    Name::new("replay-button"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Replay",
                            TextStyle {
                                font_size: 56.0,
                                font: server.load("NotoSans-Regular.ttf"),
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
}

fn on_update(query: Query<(Entity, &Name, &Interaction)>, mut state: ResMut<State<ActiveState>>) {
    let interaction = query
        .iter()
        .find(|(_, name, _)| name.as_str() == "replay-button")
        .unwrap()
        .2;

    if *interaction == Interaction::Clicked {
        state.set(ActiveState::Active).unwrap();
    }
}

fn on_exit(query: Query<(Entity, &Name)>, mut commands: Commands) {
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
        d((&ayame, "It was awesome! How about yours?")),
        d((&yuki, "Me too!")),
        d((&everyone, "Yahuuu!!")),
        d((
            [
                ("I like this example", "LIKED_THIS"),
                ("Great enough", "GREAT_ENOUGH"),
                ("Not so much", "NOT_SO_MUCH"),
            ],
            ChooseDialogSettings {
                question: Some(Text::from_section(
                    "Do you like this demo?",
                    text_style.clone(),
                )),
                use_dialog: UseDialog::None,
            },
            text_style.clone(),
        )),
        d((
            ["I like this example", "Great enough", "Not so much"],
            ChooseDialogSettings {
                question: Some(Text::from_section(
                    "Do you like this demo?",
                    text_style.clone(),
                )),
                use_dialog: UseDialog::None,
            },
            text_style,
            // need this empty bracket () to prevent confusing trait conflicts
            ()
        )),
    ]);

    dialogs.start(commands);
}
pub fn camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
