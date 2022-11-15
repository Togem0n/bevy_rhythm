use std::time::Duration;
use crate::consts::*;
use crate::types::*;

use bevy::prelude::*;

/// Keeps the textures and materials for Arrows
#[derive(Resource)]
struct ArrowMaterialResource {
    red_texture: Handle<Image>,
    blue_texture: Handle<Image>,
    green_texture: Handle<Image>,
    border_texture: Handle<Image>,
}

impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) ->Self {
        let world = world.cell();
        let mut textures = world.get_resource_mut::<Assets<Image>>().unwrap();    
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let red_texture: Handle<Image> = asset_server.load("images/arrow_red.png");
        let blue_texture = asset_server.load("images/arrow_blue.png");
        let green_texture = asset_server.load("images/arrow_green.png");
        let border_texture = asset_server.load("images/arrow_border.png");

        ArrowMaterialResource {
            red_texture: red_texture,
            blue_texture: blue_texture, 
            green_texture: green_texture,
            border_texture: border_texture,
        }
    }
}

#[derive(Component)]
struct Arrow;

// make timer a resouce
#[derive(Resource)]
struct SpawnTimer {
    pub repeated_timer: Timer,
} // to distiguish the other timer

impl SpawnTimer {
    pub fn new() -> Self {
        Self {
            repeated_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}
impl Default for SpawnTimer {
    fn default() -> Self {
        Self::new()
    }
}

fn spawn_arrows(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    textures: Res<ArrowMaterialResource>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    if !timer.repeated_timer.tick(time.delta()).just_finished() {
        return;
    }

    let transform = Transform::from_translation(Vec3::new(0.0, SPAWN_POSITION, 1.0));
    commands
        .spawn(SpriteBundle {
            texture: textures.red_texture.clone(),
            transform,
            sprite: Sprite {
                custom_size: Some(Vec2{
                    x: 100.0,
                    y: 100.0,
                }),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Arrow);
}

/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * BASE_SPEED;
    }
}

pub struct ArrowsPlugin;
impl Plugin for ArrowsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize Resources
            .init_resource::<ArrowMaterialResource>()
            .init_resource::<SpawnTimer>()
            // Add systems
            .add_system(spawn_arrows)
            .add_system(move_arrows);
    }
}