use crate::{consts::*};
use bevy::{input::{keyboard::KeyCode, Input}, prelude::{Resource, Handle, AudioSource, AssetServer}};
use core::f32::consts::PI;

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct SongConfigToml {
    pub name: String,
    pub filename: String,
    pub arrows: Vec<ArrowTimeToml>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ArrowTimeToml {
    pub click_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
}
impl Directions {
    /// Checks if a key that corresponds to this direction has been pressed
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool {
        let keys = match self {
            Directions::Left => [KeyCode::Left, KeyCode::D],
            Directions::Down => [KeyCode::Down, KeyCode::F],
            Directions::Up => [KeyCode::Up, KeyCode::J],
            Directions::Right => [KeyCode::Right, KeyCode::K],
        };

        keys.iter().any(|code| input.just_pressed(*code))
    }

    /// Returns the correct rotation for an arrow with this direction
    pub fn rotation(&self) -> f32 {
        match self {
            Directions::Up => PI * 0.5,
            Directions::Down => -PI * 0.5,
            Directions::Left => PI,
            Directions::Right => 0.,
        }
    }

    /// Returns the correct y coordinate for an arrow with this direction
    pub fn x(&self) -> f32 {
        match self {
            Directions::Left => -90.,
            Directions::Down => -30.,
            Directions::Up => 30.,
            Directions::Right => 90.,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}
impl Speed {
    /// Returns actual speed at which the arrow should move
    pub fn value(&self) -> f32 {
        BASE_SPEED * self.multiplier()
    }
    /// Speed multiplier
    pub fn multiplier(&self) -> f32 {
        match self {
            Speed::Slow => 1.,
            Speed::Medium => 1.2,
            Speed::Fast => 1.5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
/// Keeps track of when each arrow should spawn and it's speed and direction
pub struct ArrowTime {
    pub spawn_time: f64,
    pub speed: Speed,
    pub direction: Directions,
}

impl ArrowTime {
    fn new(arrow: &ArrowTimeToml) -> Self {
        let speed_value = arrow.speed.value();
        // println!("{} + {}", click_time, click_time - (DISTANCE / speed_value) as f64);
        Self {
            spawn_time: arrow.click_time - (DISTANCE / speed_value) as f64,
            speed: arrow.speed,
            direction: arrow.direction,
        }
    }
}

#[derive(Debug, Resource)]
pub struct SongConfig {
    pub arrows: Vec<ArrowTime>,
    pub name: String,
    pub song_audio: Handle<AudioSource>,
}

pub fn load_config(path: &str, asset_server: &AssetServer) -> SongConfig {
    // Open file and read contents
    // let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut file = File::open(format!("assets/songs/{}", path)).expect("Couldn't open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Couldn't read file into String");

    println!("debug 1");
    // Parse using toml and Serde
    let parsed: SongConfigToml =
        toml::from_str(&contents).expect("Couldn't parse into SongConfigToml");

    println!("debug 2");
    // Process arrows
    let mut arrows = parsed
        .arrows
        .iter()
        .map(|arr| ArrowTime::new(arr))
        .collect::<Vec<ArrowTime>>();
    // Sort arrows by spawn_time
    arrows.sort_by(|a, b| a.spawn_time.partial_cmp(&b.spawn_time).unwrap());

    // Load song audio and get the handle
    let song_audio = asset_server.load(&*format!("songs/{}", parsed.filename));
    println!("{}", parsed.filename);

    SongConfig {
        name: parsed.name,
        song_audio,
        arrows,
    }
}
