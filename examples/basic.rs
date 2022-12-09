use std::marker::PhantomData;

use bevy::{prelude::App, DefaultPlugins};
use decora::DecoraPlugin;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(DecoraPlugin);
    app.add_startup_system(start_dialog);
    app.run();
}

pub fn start_dialog() {
    
}
