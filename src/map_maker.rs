use crate::time::ControlledTime;
use crate::consts::*;
use crate::types::{
    ArrowTimeToml,
    Directions::{self, *},
    Speed,
};
use bevy::core_pipeline::core_2d::graph::input;
use bevy::{
    app::AppExit,
    input::{keyboard::KeyCode, Input},
    prelude::*,
};
use serde_derive::Serialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Resource, Serialize, Debug, Default)]
struct Presses {
    arrows: Vec<ArrowTimeToml>,
}

/// Saves key presses to Presses
fn save_key_presses(
    time: Res<ControlledTime>,
    keyboard_input: Res<Input<KeyCode>>,
    mut presses: ResMut<Presses>,
) {
    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        if direction.key_just_pressed(&keyboard_input) {
            presses.arrows.push(ArrowTimeToml {
                click_time: time.seconds_since_startup(),
                speed: Speed::Medium,
                direction: *direction,
            });
        }
    }
}

fn save_to_file_on_exit(
    keys: Res<Input<KeyCode>>,
    events: Res<Events<AppExit>>,
    presses: Res<Presses>,
) {
    if keys.just_pressed(KeyCode::S) {
        let text = toml::to_string(&*presses).expect("Couldn't convert to toml text");
        println!("save");
        let mut file = File::create("map.toml").expect("Couldn't open map.toml");
        file.write_all(text.as_bytes())
            .expect("Couldn't write to map.toml");
    }
}

struct MapMakerArrow(Directions);

/// Creates map maker arrows
fn setup_map_maker_arrows(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
){
    let border_handle = asset_server.load("images/arrow_border.png");

    let directions = [Up, Down, Left, Right];
    for direction in directions.iter() {
        let y = match direction {
            Up => 150.,
            Down => 50.,
            Left => -50.,
            Right => -150.,
        };

        let mut transform = Transform::from_translation(Vec3::new(0., y, 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn(SpriteBundle {
                texture: border_handle.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2{
                        x: 100.0,
                        y: 100.0,
                    }),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            })
            ;
    }
}

/// Toggles visibility according to if corresponding key is being pressed
// fn toggle_map_maker_arrows(
//     mut query: Query<(&mut Visible, &MapMakerArrow)>,
//     keyboard_input: Res<Input<KeyCode>>,
// ) {
//     for (mut visible, arrow) in query.iter_mut() {
//         visible.is_visible = arrow.0.key_pressed(&keyboard_input);
//     }
// }

pub struct MapMakerPlugin;
impl Plugin for MapMakerPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Presses>()
        .add_system_set(
            SystemSet::on_enter(AppState::MakeMap)
            .with_system(setup_map_maker_arrows)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MakeMap)
            .with_system(save_key_presses)
        )
        .add_system_set(
            SystemSet::on_update(AppState::MakeMap)
            .with_system(save_to_file_on_exit)
        )
        ;

            // .on_state_enter(
            //     APP_STATE_STAGE,
            //     AppState::MakeMap,
            //     setup_map_maker_arrows.system(),
            // )
            // .on_state_update(
            //     APP_STATE_STAGE,
            //     AppState::MakeMap,
            //     toggle_map_maker_arrows.system(),
            // )
            // .on_state_update(
            //     APP_STATE_STAGE,
            //     AppState::MakeMap,
            //     save_key_presses.system(),
            // )
            // .on_state_update(
            //     APP_STATE_STAGE,
            //     AppState::MakeMap,
            //     save_to_file_on_exit.system(),
            // );
    }
}