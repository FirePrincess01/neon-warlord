//! Implements the WorldInterface from AntAi

use cgmath::{InnerSpace, Zero};

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
    pub is_final: bool,
}

impl AntPosition {
    pub fn new() -> Self {
        let pos = InterpolatedPosition::zero();
        let look_at = cgmath::Vector3::zero();
        let is_final = true;

        Self {
            pos,
            look_at,
            is_final,
        }
    }
}

#[derive(Clone, Copy)]
pub struct AntPositionSnapshot {
    pub pos: cgmath::Vector3<f32>,
    pub look_at: cgmath::Vector3<f32>,
    pub time_stamp: instant::Instant,
}

#[derive(Clone, Copy)]
pub enum AntAction {
    UpdatePosition(AntPositionSnapshot),
    FinalPosition(AntPositionSnapshot),
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
    pub fn update_position(
        pos: cgmath::Vector3<f32>,
        look_at: cgmath::Vector3<f32>,
        time_stamp: instant::Instant,
        index: usize,
    ) -> Self {
        Self {
            action: AntAction::UpdatePosition(AntPositionSnapshot {
                pos,
                look_at,
                time_stamp,
            }),
            index,
        }
    }

    pub fn final_position(
        pos: cgmath::Vector3<f32>,
        look_at: cgmath::Vector3<f32>,
        time_stamp: instant::Instant,
        index: usize,
    ) -> Self {
        Self {
            action: AntAction::FinalPosition(AntPositionSnapshot {
                pos,
                look_at,
                time_stamp,
            }),
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

    animation: AntAnimation,

    state: State,

    index: usize,
}

impl AntController {
    pub fn new(position: cgmath::Vector2<f32>, index: usize) -> Self {
        let target_position = position;
        let animation = AntAnimation::Idle;
        let state = State::Idle;

        Self {
            position,
            target_position,
            animation,
            state,
            index,
        }
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
                let new_position =
                    self.position + (self.target_position - self.position).normalize() * speed;
                // println!("position{:?}", self.position);
                // println!("target position{:?}", self.target_position);
                // println!("new position {:?}", new_position);

                // check if position has been reached
                let finish_reached = (self.target_position - new_position).magnitude2()
                    < (self.target_position - self.position).magnitude2();

                if finish_reached {
                    self.position = new_position;
                } else {
                    self.position = self.target_position;
                }

                let pos = cgmath::Vector3 {
                    x: self.position.x,
                    y: self.position.y,
                    z: 0.0,
                };
                let target =
                    cgmath::Vector3::new(self.target_position.x, self.target_position.y, 0.0);

                actions.push(if finish_reached {
                    AntActionStruct::update_position(pos, target, *time_stamp, self.index)
                } else {
                    AntActionStruct::final_position(pos, target, *time_stamp, self.index)
                });
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
