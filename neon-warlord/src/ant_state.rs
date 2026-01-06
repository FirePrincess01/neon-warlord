//! Tracks the state of an ant

use cgmath::InnerSpace;

use crate::ant_storage::{Ant, AntStorage};

type Vec2 = cgmath::Vector2<f32>;

#[derive(Clone)]
pub struct AntState {
    // state
    pub index: usize,
    pub position: Vec2,
    pub is_shot_ready: bool,

    // commands
    pub target_position: Option<Vec2>,
    pub charge_shot: bool,
    pub shoot: bool,
}

impl AntState {
    pub fn new() -> Self {
        let index = 0;
        let position: Vec2 = Vec2::new(0.0, 0.0);
        let is_shot_ready: bool = false;

        let target_position: Option<Vec2> = None;
        let charge_shot: bool = false;
        let shoot: bool = false;

        Self {
            index,
            position,
            is_shot_ready,
            target_position,
            charge_shot,
            shoot,
        }
    }

    pub fn update(&mut self) -> AntPosition {
        let target_position = match self.target_position {
            Some(pos) => pos,
            None => self.position,
        };
        
        // calculate vector to to new position
        let delta = (target_position - self.position).normalize() * 0.1;

        // set new position
        let new_position = self.position + delta;

        // check if position has been reached
        if (target_position - new_position).magnitude2()
            < (target_position - self.position).magnitude2()
        {
            self.position = new_position;
        } else {
            self.position = target_position;
        }

        AntPosition {
            index: self.index,
            pos: cgmath::Vector3 {
                x: self.position.x,
                y: self.position.y,
                z: 0.0,
            },
            look_at: cgmath::Vector3 {
                x: target_position.x,
                y: target_position.y,
                z: 0.0,
            },
        }
    }
}

pub struct AntPosition {
    pub index: usize,
    pub pos: cgmath::Vector3<f32>,
    pub look_at: cgmath::Vector3<f32>,
}
