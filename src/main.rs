use bevy::prelude::*;
mod arrow;
use arrow::ArrowsPlugin;
mod consts;
mod types;

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 800.0,
                height: 800.0,
                ..default()
              },
              ..default()
        }))
        .add_plugin(ArrowsPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// start up system that spawn camera
// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
// }

fn setup(mut commands: Commands) {
    // let config = types::load_config();

    commands
        .spawn(Camera2dBundle::default())
        .commands();
}
