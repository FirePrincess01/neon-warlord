//! A pendulum on a cart

use crate::{advanced_composition::motor_linear::MotorLinear, pendulum_simulation::Vec3, verlet_physics_simd::VerletPhysicsSimd};

pub struct Pendulum {
    pub verlet_physics: VerletPhysicsSimd,

    particles_static: [usize; 2],
    particles_static_pos: [Vec3; 2],
    _particle_cart: usize,
    particle_pendulum: usize,

    motor_linear: MotorLinear,
}

impl Pendulum {
    pub fn new() -> Self {
        let mut verlet_physics = VerletPhysicsSimd::new();

        let radius = 0.1;
        let mass = 0.1;
        
        let particles_static_pos_0 = Vec3::new(-5.0, 0.0, 0.0);
        let particles_static_0 = verlet_physics.push_particle(
            particles_static_pos_0, 
            radius, 
            mass
        );

        let _particle_cart = verlet_physics.push_particle(
            Vec3::new(0.0, 0.0, 0.0), 
            radius, 
            mass
        );

        let particles_static_pos_1 = Vec3::new(5.0, 0.0, 0.0);
        let particles_static_1 = verlet_physics.push_particle(
            particles_static_pos_1, 
            radius, 
            mass
        );

        let particle_pendulum = verlet_physics.push_particle(
            Vec3::new(0.1, 0.0, 1.0), 
            radius, 
            mass
        );

        verlet_physics.push_constraint_distance(
            _particle_cart, 
            particle_pendulum, 
            1.0, 
            0.8
        );

        verlet_physics.push_constraint_none(particles_static_0, _particle_cart);
        verlet_physics.push_constraint_none(particles_static_1, _particle_cart);

        let motor_linear = MotorLinear::new(
            _particle_cart, 
            particles_static_0, 
            particles_static_1,
        );

        Self { 
            verlet_physics, 
            particles_static: [
                particles_static_0,
                particles_static_1,
            ], 
            particles_static_pos: [
                particles_static_pos_0,
                particles_static_pos_1,
            ], 
            _particle_cart, 
            particle_pendulum,
            motor_linear,
        }
    }

    pub fn update(&mut self, action: PendulumAction) -> PendulumState {
        self.apply_static_constraint();
        self.apply_cart_constraint(action);

        PendulumState { alpha: 0.0, angular_velocity: 0.0, cart_pos: 0.0, cart_velocity: 0.0 }
    }

    fn apply_static_constraint(&mut self) 
    {
        self.verlet_physics.particles.set_position(
            self.particles_static[0], 
            self.particles_static_pos[0],
        );

        self.verlet_physics.particles.set_position(
            self.particles_static[1], 
            self.particles_static_pos[1],
        );
    }

    fn apply_cart_constraint(&mut self, action: PendulumAction) {
        self.motor_linear.update_simd(&mut self.verlet_physics.particles);

        match action {
            PendulumAction::Left => self.motor_linear.accelerate(-0.1),
            PendulumAction::Right => self.motor_linear.accelerate(0.1),
            PendulumAction::None => { },
        }
    }
    
    pub fn update_verlet_physics(&mut self, dt: f32) {
        self.verlet_physics.update(dt);
    }
}

pub enum PendulumAction {
    Left,
    Right,
    None,
}

pub struct PendulumState {
    pub alpha: f32,
    pub angular_velocity: f32,
    pub cart_pos: f32,
    pub cart_velocity: f32,
}
