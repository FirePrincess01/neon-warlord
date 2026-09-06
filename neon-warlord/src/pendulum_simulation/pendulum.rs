//! A pendulum on a cart

use crate::{advanced_composition::motor_linear::MotorLinear, pendulum_simulation::Vec3, verlet_physics_simd::VerletPhysicsSimd};

pub struct Pendulum {
    pub verlet_physics: VerletPhysicsSimd,

    particles_static: [usize; 2],
    particles_static_pos: [Vec3; 2],
    particle_cart: usize,
    particle_pendulum: usize,

    motor_linear: MotorLinear,

    // variables
    previous_angle: f32,
    unwrapped_angle: f32,
    previous_cart_position: f32,
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

        let particle_cart = verlet_physics.push_particle(
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
            particle_cart, 
            particle_pendulum, 
            1.0, 
            0.8
        );

        verlet_physics.push_constraint_none(particles_static_0, particle_cart);
        verlet_physics.push_constraint_none(particles_static_1, particle_cart);

        let motor_linear = MotorLinear::new(
            particle_cart, 
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
            particle_cart, 
            particle_pendulum,
            motor_linear,
            previous_angle: 0.0,
            unwrapped_angle: 0.0,
            previous_cart_position: 0.0,
        }
    }

    pub fn update(&mut self, action: PendulumAction, dt: f32) -> PendulumState {
        self.apply_static_constraint();
        self.apply_cart_constraint(action);

        let (alpha, angular_velocity) = self.calculate_angle(dt);
        let (cart_pos, cart_velocity) = self.calculate_cart_position(dt);

        PendulumState { alpha, angular_velocity, cart_pos, cart_velocity }
    }

    // calculates the angle between the cart and the pendulum and returns the angle and the angular velocity
    fn calculate_angle(&mut self, dt:f32) -> (f32, f32) {
        let cart_index: usize = self.particle_cart;
        let pendulum_index: usize = self.particle_pendulum;
        let cart: cgmath::Vector3<f32> = self.verlet_physics.particles.position(cart_index);
        let pendulum: cgmath::Vector3<f32> = self.verlet_physics.particles.position(pendulum_index);

        let dx = pendulum.x - cart.x;
        let dy = pendulum.z - cart.z;

        // Raw angle in [-PI, PI].
        let angle = dy.atan2(dx);

        // Difference from the previous angle.
        let mut delta = angle - self.previous_angle;

        // Wrap the difference to [-PI, PI].
        // This removes the artificial 2*PI jump at the atan2 boundary.
        const PI: f32 = std::f32::consts::PI;
        const TWO_PI: f32 = 2.0 * PI;

        if delta > PI {
            delta -= TWO_PI;
        } else if delta < -PI {
            delta += TWO_PI;
        }

        // Accumulate the small change rather than using the raw angle.
        self.unwrapped_angle += delta;
        self.previous_angle = angle;

        // Angular velocity in radians/sec.
        let angular_velocity = if dt > 0.0 {
            delta / dt
        } else {
            0.0
        };

        (self.unwrapped_angle, angular_velocity)
    }

    // Calculates the position of the cart ranging from -1.0 to 1.0 and the velocity
    fn calculate_cart_position(&mut self, dt:f32) -> (f32, f32) {
        let cart_index: usize = self.particle_cart;
        let lef_index: usize = self.particles_static[0];
        let right_index: usize = self.particles_static[1];
        let cart: cgmath::Vector3<f32> = self.verlet_physics.particles.position(cart_index);
        let left: cgmath::Vector3<f32> = self.verlet_physics.particles.position(lef_index);
        let right: cgmath::Vector3<f32> = self.verlet_physics.particles.position(right_index);

        // Normalize cart position from [left.x, right.x] to [-1.0, 1.0].
        let position = 2.0 * (cart.x - left.x) / (right.x - left.x) - 1.0;

        // Calculate velocity.
        let velocity = (position - self.previous_cart_position) / dt;

        self.previous_cart_position = position;

        (position, velocity)
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
            PendulumAction::_Left => self.motor_linear.accelerate(-0.1),
            PendulumAction::_Right => self.motor_linear.accelerate(0.1),
            PendulumAction::None => { },
        }
    }
    
    pub fn update_verlet_physics(&mut self, dt: f32) {
        self.verlet_physics.update(dt);
    }
}

pub enum PendulumAction {
    _Left,
    _Right,
    None,
}

pub struct PendulumState {
    pub alpha: f32,
    pub angular_velocity: f32,
    pub cart_pos: f32,
    pub cart_velocity: f32,
}
