use bevy::prelude::*;
mod arrow;
use arrow::ArrowsPlugin;
mod consts;
use consts::*;
mod types;
mod ui;
use ui::UiPlugin;
mod score;
use score::ScoreResource;
mod audio;
use audio::AudioPlugin;
mod menu;
use menu::MenuPlugin;
mod time;
use time::TimePlugin;
// mod shaders;
// use shaders::ShadersPlugin;

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
        .add_plugin(MenuPlugin)
        .add_state(AppState::Menu)
        .add_plugin(TimePlugin) 
        // .add_stage_after(CoreStage::Update, APP_STATE_STAGE, SystemStage::add_system_set(&mut self, system_set))
        // .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup))
        .add_system(bevy::window::close_on_esc)
        // .add_plugin(ShadersPlugin)
        .add_startup_system(setup)
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