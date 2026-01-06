//! Routines controlling an ant npc

use cgmath::prelude::*;

use crate::game_board::{self, Agent, Faction};

type Vec2 = cgmath::Vector2<f32>;

pub struct StateData {
    pub target_agent: Option<Agent>,
}

impl StateData {
    pub fn new() -> Self {
        Self { target_agent: None }
    }
}

pub struct AntAi {
    // consts
    faction: Faction,
    range: f32,

    // vars
    state: State,
    state_data: StateData,
}

impl AntAi {
    pub fn new(faction: Faction) -> Self {
        let state = State::LookForEnemies;
        let range = 10.0;
        let state_data = StateData::new();

        Self {
            state,
            faction,
            range,
            state_data,
        }
    }

    pub fn update(&mut self, interface: &mut dyn AntAiInterface) {
        match self.state {
            State::LookForEnemies => {
                // #######################################################
                // get closest enemy
                let position = interface.get_position();
                let agents = interface.world_get_agents();
                let closest_enemy = self.get_closest_enemy(position, agents);

                // state transition
                if closest_enemy.is_some() {
                    self.state_data.target_agent = closest_enemy;
                    self.state = State::MoveUntilInRange;
                }

                self.state_data.target_agent = Some(Agent { faction: Faction::Red, position: Vec2::new(0.0, 80.0) })

            }
            State::MoveToTarget => {
                // #######################################################
                // get positions
                let position = interface.get_position();
                let target_agent = self.state_data.target_agent.unwrap();
                let target_agent_position = target_agent.position;

                // check if in range
                if self.is_in_range(position, target_agent_position) {
                    self.state = State::ChargeShot;
                    return;
                }

                let direction = (position - target_agent_position).normalize();
                let target_position = target_agent_position + self.range * direction;

                // state transition
                interface.move_to(target_position);
                self.state = State::MoveUntilInRange;
            }
            State::MoveUntilInRange => {
                // #######################################################

                // wait for finish moving
                if interface.is_moving() {
                    return;
                }

                // state transition
                self.state = State::MoveToTarget;
            }
            State::ChargeShot => {
                // #######################################################
                interface.charge_shot();
                self.state = State::WaitUntilShotCharged;
            }
            State::WaitUntilShotCharged => {
                // #######################################################
                if interface.is_shot_ready() {
                    self.state = State::Shoot;
                }
            }
            State::Shoot => {
                // #######################################################
                interface.shoot();
                self.state = State::LookForEnemies;
            }
        }
    }

    fn get_closest_enemy(&self, position: Vec2, agents: &[Agent]) -> Option<Agent> {
        let mut distance = f32::MAX;
        let mut res = None;

        for agent in agents {
            if agent.faction != self.faction {
                let agent_distance = (agent.position - position).magnitude();

                if agent_distance < distance {
                    distance = agent_distance;
                    res = Some(*agent);
                }
            }
        }

        res
    }

    fn is_in_range(
        &self,
        position: cgmath::Vector2<f32>,
        target_agent_position: cgmath::Vector2<f32>,
    ) -> bool {
        (target_agent_position - position).magnitude() <= self.range
    }
}

enum State {
    LookForEnemies,
    MoveToTarget,
    MoveUntilInRange,
    ChargeShot,
    WaitUntilShotCharged,
    Shoot,
}

pub trait AntBodyInterface {
    fn get_position(&self) -> Vec2;
    fn move_to(&mut self, target_position: Vec2);
    fn is_moving(&self) -> bool;
    fn charge_shot(&mut self);
    fn is_shot_ready(&self) -> bool;
    fn shoot(&mut self);
}

pub trait WorldInterface {
    fn world_get_agents(&self) -> &[game_board::Agent];
}

pub trait AntAiInterface: AntBodyInterface + WorldInterface {}

// #[derive(PartialEq, Clone, Copy)]
// pub enum Faction {
//     Red,
//     Blue,
// }

// #[derive(Clone, Copy)]
// pub struct Agent {
//     faction: Faction,
//     position: cgmath::Vector2<f32>,
// }
