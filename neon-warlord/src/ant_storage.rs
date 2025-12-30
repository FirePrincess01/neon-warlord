//! Manages all the ant objects on the GPU
//!

// use market_economy_simulation_server::{ants, point_lights::Attenuation};

// use crate::point_light_storage::{PointLightIndex, PointLightInterface, PointLightStorage};
use forward_renderer::AnimatedObjectStorage;

pub struct Ant {
    pub id: usize,

    pub pos: cgmath::Vector2<f32>,
    pub _rot_z: f32,

    pub _light_strength: f32,
    pub _light_color: cgmath::Vector3<f32>,
}

pub struct AntStorage {
    // pub point_light_storage: PointLightStorage,
    pub animated_object_storage: AnimatedObjectStorage,

    max_ants: usize,
}

impl AntStorage {
    pub fn new(
        // point_light_storage: PointLightStorage,
        animated_object_storage: AnimatedObjectStorage,
        max_ants: usize,
    ) -> Self {
        // assert_eq!(max_ants, point_light_storage.max_instances());
        assert_eq!(max_ants, animated_object_storage.max_instances());

        Self {
            // point_light_storage,
            animated_object_storage,
            max_ants,
        }
    }

    pub fn set_ant(&mut self, ant: &Ant) {
        if ant.id < self.max_ants {
            let pos = cgmath::Vector3::new(ant.pos.x + 10.0, ant.pos.y, 2.0);
            // let pos = cgmath::Vector3::new(0.0, 0.0, 0.0);
            self.animated_object_storage.set_pos(ant.id, pos);
            // self.point_light_storage.set_position(&PointLightIndex{ instance_index: ant.id }, pos);
            // self.point_light_storage.set_light(
            //     PointLightIndex {
            //         instance_index: ant.id,
            //     },
            //     pos,
            //     ant.light_color,
            //     Attenuation::_200,
            // );

            self.animated_object_storage.set_active(ant.id);
            // self.point_light_storage.set_active(
            //     &PointLightIndex {
            //         instance_index: ant.id,
            //     },
            //     true,
            // );
        }
    }
}
