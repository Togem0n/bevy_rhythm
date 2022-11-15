use crate::consts::*;
use bevy::{input::{keyboard::KeyCode, Input}, prelude::Resource};
use core::f32::consts::PI;

#[derive(Copy, Clone, Debug, PartialEq)]
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
            Directions::Up => [KeyCode::Up, KeyCode::D],
            Directions::Down => [KeyCode::Down, KeyCode::F],
            Directions::Left => [KeyCode::Left, KeyCode::J],
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
            Directions::Up => 75.,
            Directions::Down => 20.,
            Directions::Left => -25.,
            Directions::Right => -75.,
        }
    }
}

#[derive(Copy, Clone, Debug)]
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
    fn new(click_time: f64, speed: Speed, direction: Directions) -> Self {
        let speed_value = speed.value();
        // println!("{} + {}", click_time, click_time - (DISTANCE / speed_value) as f64);
        Self {
            spawn_time: click_time - (DISTANCE / speed_value) as f64,
            speed,
            direction,
        }
    }
}

#[derive(Debug, Resource)]
pub struct SongConfig {
    pub arrows: Vec<ArrowTime>,
}

pub fn load_config() -> SongConfig {
    SongConfig {
        arrows: vec![
            ArrowTime::new(3., Speed::Medium, Directions::Up),
            ArrowTime::new(3.5, Speed::Medium, Directions::Down),
            ArrowTime::new(4., Speed::Medium, Directions::Left),
            ArrowTime::new(4.5, Speed::Medium, Directions::Right),
            ArrowTime::new(5., Speed::Medium, Directions::Up),
            ArrowTime::new(5.5, Speed::Medium, Directions::Down),
            ArrowTime::new(6., Speed::Medium, Directions::Left),
            ArrowTime::new(6.5, Speed::Medium, Directions::Right),
        ],
    }
}
