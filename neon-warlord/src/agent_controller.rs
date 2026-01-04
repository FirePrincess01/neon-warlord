//! Controls the agents and the game board 

use crate::ant_ai::AntBodyInterface;

struct AgentController {

}

impl AntBodyInterface for AgentController {
    fn get_position(&self) -> Vec2 {
        todo!()
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