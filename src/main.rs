use bevy::prelude::*;
use plugins::HelloPlugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

mod entities;
mod plugins;
mod systems;
