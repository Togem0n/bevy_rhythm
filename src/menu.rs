use crate::consts::*;
use bevy::prelude::*;

fn setup_main_menu(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
) {
    println!("enter main menu");
}

fn in_main_menu(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::M) {
        game_state.set(AppState::MakeMap).unwrap();
    }
    if keys.just_pressed(KeyCode::Space) {
        game_state.set(AppState::Game).unwrap();
    }
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(
            SystemSet::on_enter(AppState::Menu)
            .with_system(setup_main_menu)
        )
        .add_system_set(
            SystemSet::on_update(AppState::Menu)
            .with_system(in_main_menu)
        );
    }
}