use bevy:: prelude::*;

mod arrow;
mod types;
mod consts;

fn main() {
    App::new()
        // Set antialiasing to use 4 samples
        .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                window: WindowDescriptor {
                    title: "kaiosu".to_string(),
                    height: 900.0,
                    width: 900.0,
                    ..default()
                },
                ..default()
            }
        ))
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(
        Camera2dBundle::default()
    );
}