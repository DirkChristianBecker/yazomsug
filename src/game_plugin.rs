use bevy::prelude::*;

use bevy_inventory_system::prelude::UiCameraComponent;

pub struct GamePlugin;

fn setup(mut commands: Commands, _server: Res<AssetServer>) {
    commands.spawn((Camera3dBundle::default(), UiCameraComponent {}));
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
