//! Implements the WorldInterface from AntAi

use cgmath::InnerSpace;

use crate::{
    ant_ai::{AntAiInterface, AntBodyInterface, WorldInterface},
    ant_state::AntState,
    game_board::{self, GameBoard},
    worker::InterpolatedPosition::InterpolatedPosition,
};

type Vec2 = cgmath::Vector2<f32>;

#[derive(Clone, Copy, PartialEq)]
pub enum AntAnimation {
    Idle,
    Walk,
    ChargeShot,
}

#[derive(Clone, Copy, Debug)]
pub struct AntPosition {
    pub pos: InterpolatedPosition,
    pub look_at: cgmath::Vector3<f32>,
}

#[derive(Clone, Copy)]
pub enum AntAction {
    SetPosition(AntPosition),
    SetAnimation(AntAnimation),
}

#[derive(Clone, Copy)]
pub struct AntActionStruct {
    pub action: AntAction,
    pub index: usize,
}

impl AntActionStruct {
    pub fn from_animation(action: AntAnimation, index: usize) -> Self {
        Self {
            action: AntAction::SetAnimation(action),
            index,
        }
    }
    pub fn from_translation(pos: InterpolatedPosition, look_at: cgmath::Vector3<f32>, index: usize) -> Self {
        Self {
            action: AntAction::SetPosition(AntPosition { pos, look_at }),
            index,
        }
    }
}

#[derive(PartialEq)]
enum State {
    Idle,
    Move,
    ChargeShot,
    ShotCharged,
    Shoot,
}

pub struct AntController {
    // pub game_board: &'a GameBoard,
    // pub ant_state: &'a mut AntState,
    position: cgmath::Vector2<f32>,
    target_position: cgmath::Vector2<f32>,

    interpolated_position: InterpolatedPosition,
    interpolated_position_initialized: bool,
    animation: AntAnimation,

    state: State,

    index: usize,
}

impl AntController {
    pub fn new(position: cgmath::Vector2<f32>, index: usize) -> Self {
        let target_position = position;
        let interpolated_position = InterpolatedPosition::zero();
        let interpolated_position_initialized = false;
        let animation = AntAnimation::Idle;
        let state = State::Idle;


        Self { position, target_position, interpolated_position, interpolated_position_initialized, animation, state, index }
    }
    
    pub fn update(&mut self, time_stamp: &instant::Instant, actions: &mut Vec<AntActionStruct>) {
        match self.state {
            // ##################################################
            State::Idle => {
                // Set animation
                if self.animation != AntAnimation::Idle {
                    self.animation = AntAnimation::Idle;
                    actions.push(AntActionStruct::from_animation(
                        AntAnimation::Idle,
                        self.index,
                    ));
                }
            }
            // ##################################################
            State::Move => {
                // Check if position has been reached
                if self.position == self.target_position {
                    self.state = State::Idle;
                    return;
                }

                // Set animation
                if self.animation != AntAnimation::Walk {
                    self.animation = AntAnimation::Walk;
                    actions.push(AntActionStruct::from_animation(
                        AntAnimation::Walk,
                        self.index,
                    ));
                }

                // set position
                let speed = 0.1;
                let new_position = self.position + (self.target_position -self.position).normalize() * speed;
                // println!("position{:?}", self.position);
                // println!("target position{:?}", self.target_position);
                // println!("new position {:?}", new_position);

                // check if position has been reached
                if (self.target_position - new_position).magnitude2()
                    < (self.target_position - self.position).magnitude2()
                {
                    self.position = new_position;
                } else {
                    self.position = self.target_position;
                }

                let pos = cgmath::Vector3 { x: self.position.x, y: self.position.y, z: 0.0 };
                if !self.interpolated_position_initialized {
                    self.interpolated_position_initialized = true;
                    self.interpolated_position = InterpolatedPosition::new(pos, *time_stamp);
                }
                else {
                    self.interpolated_position.add(pos, *time_stamp);
                }

                let target = cgmath::Vector3::new(self.target_position.x, self.target_position.y, 0.0);
                actions.push(AntActionStruct::from_translation(self.interpolated_position, target, self.index));
            }
            // ##################################################
            State::ChargeShot => {}
            // ##################################################
            State::ShotCharged => {}
            // ##################################################
            State::Shoot => {}
        }
    }
}

// impl WorldInterface for AntController {
//     fn world_get_agents(&self) -> &[game_board::Agent] {
//         &self.game_board.agents
//     }
// }

impl AntBodyInterface for AntController {
    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn move_to(&mut self, target_position: Vec2) {
        self.target_position = target_position;
        self.state = State::Move
    }

    fn is_moving(&self) -> bool {
        self.state == State::Move
    }

    fn charge_shot(&mut self) {
        self.state = State::ChargeShot
    }

    fn is_shot_ready(&self) -> bool {
        self.state == State::ShotCharged
    }

    fn shoot(&mut self) {
        self.state = State::Shoot;
    }
}

// impl AntAiInterface for AntController<'_> {}
