pub mod dialog;
pub mod ui;
use std::marker::PhantomData;

use bevy::prelude::*;

use characters::Character;

pub mod characters;

pub struct DecoraPlugin;

impl Plugin for DecoraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(ui::ui);
    }
}
