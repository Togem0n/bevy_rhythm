use crate::consts::*;
use crate::time::ControlledTime;
use crate::types::*;

use bevy::prelude::*;
use crate::ScoreResource;

/// Keeps the textures and materials for Arrows
/// check how resorce means in bevy: https://bevy-cheatbook.github.io/programming/res.html
/// now just treat them as something need to be accessed globally like assets
/// typically  globally from anywhere (resources), 
///            or using ECS patterns (entities/components).

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

fn spawn_arrows(
    mut commands: Commands,
    textures: Res<ArrowMaterialResource>,
    mut song_config: ResMut<SongConfig>,
    time: Res<ControlledTime>,
) {
    let secs = time.seconds_since_startup() as f64;
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
fn move_arrows(time: Res<ControlledTime>, mut query: Query<(&mut Transform, &Arrow)>) {
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
        if (TARGET_POSITION - THRESHOLD..= TARGET_POSITION + THRESHOLD).contains(&pos)
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

fn setup_target_arrows(
    mut commands: Commands, 
    texture: Res<ArrowMaterialResource>,
) {
    println!("set up target arrows");
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
            // Add systems
            .add_system_set(
                SystemSet::on_enter(AppState::Game)
                .with_system(setup_target_arrows)
            )
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                .with_system(spawn_arrows)
                .with_system(move_arrows)
                .with_system(despawn_arrows)
            );
            // .add_startup_system(setup_target_arrows)
            // .add_system(spawn_arrows)
            // .add_system(move_arrows)
            // .add_system(despawn_arrows);
    }
}
// here we have a lot of systems, and we see each system as the game logic you want bevy to do
// you need to feed systems(functions) specific parameter types, for example:
// accessing resources using Res/ResMut
// -- accessing components of entities using queries (Query)
// -- creating/destroying entities, components, and resources using Commands (Commands)
// -- sending/receiving events using EventWriter/EventReader