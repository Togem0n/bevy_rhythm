use bevy::prelude::Resource;

use crate::consts::*;

#[derive(Default, Resource)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,
    score: f64,
}

impl ScoreResource {
    pub fn increase_corrects(&mut self, distance: f32) -> f64{
        self.corrects += 1;
        
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;
        let points = (score_multiplier * 100.0).min(100.0).max(100.0) as f64;
        self.score += points;

        points
    }
    
    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }

    pub fn score(&mut self) -> f64 {
        self.score
    }

    pub fn corrects(&mut self) -> usize {
        self.corrects
    }
    
    pub fn fails(&mut self) -> usize {
        self.fails
    }
}