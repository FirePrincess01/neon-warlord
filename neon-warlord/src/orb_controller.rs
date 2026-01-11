//! Implements the WorldInterface from AntAi

use std::time::Instant;

use cgmath::InnerSpace;


type Vec2 = cgmath::Vector2<f32>;
type Vec3 = cgmath::Vector3<f32>;

#[derive(Clone, Copy)]
pub enum OrbAction {
    SetPosition(Vec3),
    EnableCharge(bool),
    SetScale(f32),
}

#[derive(PartialEq)]
enum State {
    Idle,
    StartCharge,
    Charge,
    ChargingDone,
    StartMove,
    Move,
    Explode,
}

pub struct OrbController {
    position: Vec3,
    target_position: Vec3,

    state: State,

    index: usize,

    // State::Charge
    start_charge_time: Instant,
    charge: f32,

}

impl OrbController {
    pub fn new(position: Vec3, index: usize) -> Self {
        let target_position = position;
        let state = State::Charge;

        Self {
            position,
            target_position,
            state,
            index,
            start_charge_time: Instant::now(),
            charge: 0.0,
        }
    }

    pub fn update(&mut self, time_stamp: &Instant, actions: &mut Vec<OrbAction>) {
        const CHARGE_DURATION: f32 = 5.0;
        const SPEED: f32 = 0.01;
        const MAX_SCALE: f32 = 0.1;

        match self.state {
            // ##################################################
            State::Idle => {
                // wait for user action
            },
            // ##################################################
            State::StartCharge => {
                actions.push(OrbAction::SetPosition(self.position));
                actions.push(OrbAction::SetScale(0.01));
                actions.push(OrbAction::EnableCharge(true));

                self.start_charge_time = *time_stamp;
                self.state = State::Charge;
            },
            // ##################################################
            State::Charge => {
                let duration = *time_stamp - self.start_charge_time;
                let t = duration.as_secs_f32();

                let scale = (t / CHARGE_DURATION) * MAX_SCALE;
                self.charge = scale;
                actions.push(OrbAction::SetScale(scale));

                if t >= CHARGE_DURATION {
                    actions.push(OrbAction::EnableCharge(false));
                    self.state = State::ChargingDone;
                }                
            },
            // ##################################################
            State::ChargingDone => {
                // wait for user action
            },
            // ##################################################
            State::StartMove => {
                self.state = State::Move;
            },
            // ##################################################
            State::Move => {
                // has the position been reached
                let finish_reached = (self.target_position - self.position).magnitude2() <= SPEED * SPEED;
                
                // calculate next step
                self.position = self.position + (self.target_position - self.position).normalize() * SPEED;
                
                if finish_reached {
                    self.position = self.target_position;
                    self.state = State::Explode;
                }
            },
            // ##################################################
            State::Explode => {
                self.charge = 0.0;
                self.state = State::Idle;
            },
        }
    }
}

trait OrbInterface {
    fn set_position(&mut self, pos: Vec3);
    fn start_charge(&mut self);
    fn get_charge(&self) -> f32;
    fn move_to(&mut self, target_position: Vec3);
}

impl OrbInterface for OrbController {
    fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    fn start_charge(&mut self) {
        if self.state == State::Idle {
            self.state = State::StartCharge;
        }
    }

    fn get_charge(&self) -> f32 {
        return self.charge;
    }

    fn move_to(&mut self, target_position: Vec3) {
        if self.state == State::ChargingDone {
            self.target_position = target_position;
            self.state = State::StartMove;
        }
    }
}