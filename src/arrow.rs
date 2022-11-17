use std::time::Duration;
use crate::consts::*;
use crate::types::*;

use bevy::prelude::*;
use crate::ScoreResource;

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
struct Arrow {
    speed: Speed,
    direction: Directions,
}

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
    mut song_config: ResMut<SongConfig>,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
) {
    let secs = time.elapsed_seconds() as f64;
    let secs_last = secs - time.delta_seconds_f64();

    // Counter of how many arrows we need to spawn and remove from the list
    let mut remove_counter = 0;
    for arrow in &song_config.arrows {
        // List is ordered, so we can just check until an item fails
        // Check if arrow should be spawned at any point between last frame and this frame
        if secs_last < arrow.spawn_time && arrow.spawn_time < secs {
            remove_counter += 1;

            // Get the correct material according to speed
            let texture = match arrow.speed {
                Speed::Slow => textures.red_texture.clone(),
                Speed::Medium => textures.blue_texture.clone(),
                Speed::Fast => textures.green_texture.clone(),
            };

            let mut transform =
                Transform::from_translation(Vec3::new(arrow.direction.x(), SPAWN_POSITION, 1.));
            // Rotate the arrow acording to direction
            transform.rotate(Quat::from_rotation_z(arrow.direction.rotation()));
            commands
                .spawn(SpriteBundle {
                    texture,
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
                .insert(Arrow {
                    speed: arrow.speed,
                    direction: arrow.direction,
                });
        } else {
            break;
        }
    }

    // Remove the arrows we have spawned from the list
    for _ in 0..remove_counter {
        song_config.arrows.remove(0);
    }
}

/// Moves the arrows forward
fn move_arrows(time: Res<Time>, mut query: Query<(&mut Transform, &Arrow)>) {
    for (mut transform, _arrow) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * _arrow.speed.value();
    }
}

/// Despawns arrows when they reach the end if the correct button is clicked
fn despawn_arrows(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Arrow)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut score: ResMut<ScoreResource>,
) {
    for (entity, transform, arrow) in query.iter() {
        let pos = transform.translation.y;

        // Check if arrow is inside clicking threshold
        if (TARGET_POSITION - THRESHOLD..=TARGET_POSITION + THRESHOLD).contains(&pos)
            && arrow.direction.key_just_pressed(&keyboard_input)
        {
            commands.entity(entity).despawn();
            let _points = score.increase_corrects(TARGET_POSITION - pos);
        }

        // Despawn arrows after they leave the screen
        if pos <= 2. * TARGET_POSITION {
            commands.entity(entity).despawn();
            score.increase_fails();
        }
    }
}

#[derive(Component)]
struct TargetArrow;

fn setup_target_arrows(mut commands: Commands, texture: Res<ArrowMaterialResource>) {
    use Directions::*;
    let directions = [Up, Down, Left, Right];

    for direction in directions.iter() {
        let mut transform =
            Transform::from_translation(Vec3::new(direction.x(), TARGET_POSITION, 1.));
        transform.rotate(Quat::from_rotation_z(direction.rotation()));
        commands
            .spawn(SpriteBundle {
                texture: texture.border_texture.clone(),
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
            .insert(TargetArrow);
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
            .add_startup_system(setup_target_arrows)
            .add_system(spawn_arrows)
            .add_system(move_arrows)
            .add_system(despawn_arrows);
    }
}