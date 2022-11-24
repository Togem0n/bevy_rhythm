use std::{thread::__FastLocalKeyInner, f32::consts::PI};

use bevy::{prelude::*, input::keyboard, math};

use crate::consts::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Directions {
    Left,
    Down,
    Up,
    Right,
}

impl Directions {
    pub fn key_just_pressed(&self, input: &Input<KeyCode>) -> bool{
        let key = match self {
            Directions::Left  => [KeyCode::Left, KeyCode::D],
            Directions::Down  => [KeyCode::Down, KeyCode::F],
            Directions::Up    => [KeyCode::Up, KeyCode::J],
            Directions::Right => [KeyCode::Right, KeyCode::K],
        };

        key.iter().any(|keycode| input.just_released(*keycode))
    }

    pub fn rotation(&self) -> f32 {
        match self {
           Directions::Left   =>  PI * 0.5,
           Directions::Down   => -PI * 0.5,
           Directions::Up     =>  PI,
           Directions::Right  =>  0.,
        } 
    }

    pub fn y(&self) -> f32 {
        match self {
            Directions::Left  => -90.,
            Directions::Down  => -30.,
            Directions::Up    =>  30.,
            Directions::Right =>  90.,
        } 
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Speed {
    Slow,
    Medium,
    Fast,
}

impl Speed {
    pub fn value(&self) ->f32 {
        BASE_SPEED * self.multiplier()
    }

    pub fn multiplier(&self) ->f32 {
        match self {
            Speed::Slow   => 0.8,
            Speed::Medium => 1.,
            Speed::Fast   => 1.2,
        } 
    }
}
