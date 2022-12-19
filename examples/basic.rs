use bevy_inspector_egui::widgets::InspectorQuery;
use bevy_rpg::{
    characters::prelude::*,
    d,
    dialog::{ChooseDialogSettings, Dialogs, StyleDefaults, UseDialog},
    ActiveState,
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
        // We can't directly run a dialog on the first frame, since that'll emit StateAlreadyQueued error with Bevy.
        // So we would like to go to the main menu first
        // But actually, we can directly hook the start_dialog to the SystemSet::on_enter(ActiveState::Active)
        // But at least the main menu is there for people to replay the dialog.
        .add_system_set(SystemSet::on_enter(ActiveState::Inactive).with_system(on_enter))
        .add_system_set(SystemSet::on_update(ActiveState::Inactive).with_system(on_update))
        .add_system_set(SystemSet::on_exit(ActiveState::Inactive).with_system(on_exit))
        .add_system_set(SystemSet::on_enter(ActiveState::Active).with_system(start_dialog))
        .add_plugin(bevy_inspector_egui::InspectorPlugin::<RootUINode>::new())
        .run();
}

pub fn on_enter(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
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

pub fn start_dialog(mut commands: Commands, font: Res<AssetServer>) {
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
