use bevy::prelude::*;
use bevy::render::texture;
use crate::types::*;
use crate::consts::*;

#[derive(Resource)]
struct ArrowMaterialResource {
    red_texture: Handle<Image>,
    blue_texture: Handle<Image>,
    green_texture: Handle<Image>,
    border_texture: Handle<Image>,
}

// for constructing complex resource (assets)
impl FromWorld for ArrowMaterialResource {
    fn from_world(world: &mut World) -> Self {
       let world = world.cell();
       
       let asset_server = world.get_resource::<AssetServer>().unwrap();
       let red_texture = asset_server.load("images/arrow_red.png");
       let blue_texture = asset_server.load("images/arrow_red.png");
       let green_texture = asset_server.load("images/arrow_red.png");
       let border_texture = asset_server.load("images/arrow_red.png");

       ArrowMaterialResource {
            red_texture,
            blue_texture,
            green_texture,
            border_texture,
       }
    } 
}

#[derive(Component)]
struct Arrow {
    speed: Speed,
    direction: Directions
}

fn spawn_arrow(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    textures: Res<ArrowMaterialResource>,
) {
    
}

fn move_arrow() {
    
}

fn despawn_arrow() {
    
}