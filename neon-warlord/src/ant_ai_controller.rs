//! Implements the WorldInterface from AntAi

use crate::{
    ant_ai::{AntAiInterface, AntBodyInterface, WorldInterface}, ant_state::AntState, game_board::{self, GameBoard}
};

type Vec2 = cgmath::Vector2<f32>;

pub struct AntAiController<'a> {
    pub game_board: &'a GameBoard,
    pub ant_state: &'a mut AntState,
    pub ant_index: usize,
}

impl WorldInterface for AntAiController<'_> {
    fn world_get_agents(&self) -> &[game_board::Agent] {
        &self.game_board.agents
    }
}

impl AntBodyInterface for AntAiController<'_> {
    fn get_position(&self) -> Vec2 {
        self.ant_state.position
    }

    fn move_to(&mut self, target_position: Vec2) {
        self.ant_state.target_position = Some(target_position);
    }

    fn is_moving(&self) -> bool {
        self.ant_state.target_position.is_some()
    }

    fn charge_shot(&mut self) {
        self.ant_state.charge_shot = true
    }

    fn is_shot_ready(&self) -> bool {
        self.ant_state.is_shot_ready
    }

    fn shoot(&mut self) {
        self.ant_state.shoot = true
    }
}

impl AntAiInterface for AntAiController<'_> {}
