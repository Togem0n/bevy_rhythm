use bevy::prelude::*;
mod arrow;
use arrow::ArrowsPlugin;
mod consts;
mod types;
mod ui;
use ui::UiPlugin;
mod score;
use score::ScoreResource;
mod audio;
use audio::AudioPlugin;

// check ecs basic here
// https://bevy-cheatbook.github.io/programming/ecs-intro.html
fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        .init_resource::<ScoreResource>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 900.0,
                height: 900.0,
                ..default()
              },
              ..default()
        }))
        .add_plugin(ArrowsPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(AudioPlugin)
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

// start up system that spawn camera
// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
// }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = types::load_config("test.toml", &asset_server);
    commands
        .spawn(Camera2dBundle::default());

    commands.insert_resource(config);
}