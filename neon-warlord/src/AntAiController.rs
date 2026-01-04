//! Implements the WorldInterface from AntAi

use crate::{
    ant_ai::{AntAiInterface, AntBodyInterface, WorldInterface},
    game_board::{self, GameBoard},
};

type Vec2 = cgmath::Vector2<f32>;

pub struct AntAiController<'a> {
    pub game_board: &'a GameBoard,
    pub ant_index: usize,
}

impl WorldInterface for AntAiController<'_> {
    fn world_get_agents(&self) -> &[game_board::Agent] {
        &self.game_board.agents
    }
}

impl AntBodyInterface for AntAiController<'_> {
    fn get_position(&self) -> Vec2 {
        self.game_board.agents[self.ant_index].position
    }

    fn move_to(&mut self, target_position: Vec2) -> Vec2 {
        todo!()
    }

    fn is_moving(&self) -> bool {
        todo!()
    }

    fn charge_shot(&mut self) {
        todo!()
    }

    fn is_shot_ready(&self) -> bool {
        todo!()
    }

    fn shoot(&mut self) {
        todo!()
    }
}

impl AntAiInterface for AntAiController<'_> {}
