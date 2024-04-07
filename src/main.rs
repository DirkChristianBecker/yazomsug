mod game_plugin;

use bevy::prelude::*;
use game_plugin::GamePlugin;
use bevy_inventory_system::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, GamePlugin, InventoryPlugin))
        //.add_systems(Startup, load_ui)
        // .add_systems(Update, ((update_people, greet_people).chain()))
        .run();
}
