//! Definition of the game board

use crate::ant_ai::WorldInterface;

type Vec2 = cgmath::Vector2<f32>;

#[derive(PartialEq, Clone, Copy)]
pub enum Faction {
    Red,
    Blue,
}

#[derive(Clone, Copy)]
pub struct Agent {
    pub faction: Faction,
    pub position: Vec2,
}

pub struct GameBoard {
    pub agents: Vec<Agent>,
}

impl GameBoard {
    pub fn new() -> Self {
        let agents: Vec<Agent> = vec![
            Agent {
                faction: Faction::Blue,
                position: Vec2::new(0.0, 0.0),
            },
            Agent {
                faction: Faction::Red,
                position: Vec2::new(0.0, 10.0),
            },
        ];

        Self { agents }
    }
}

impl WorldInterface for GameBoard {
    fn world_get_agents(&self) -> &[self::Agent] {
        &self.agents
    }
}