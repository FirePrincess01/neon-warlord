//! Manages all the orb objects on the GPU

use forward_renderer::{
    glow_storage::GlowStorage, particle_storage::ParticleStorage,
    plasma_orb_storage::PlasmaOrbStorage,
};

pub struct OrbStorage {
    particle_storage: ParticleStorage,
    glow_storage: GlowStorage,
    plasma_orb_storage: PlasmaOrbStorage,

    max_instances: usize,
}

impl OrbStorage {
    pub fn new(
        wgpu_renderer: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        max_instances: usize,
    ) -> Self {
        let particle_storage = ParticleStorage::new(wgpu_renderer, max_instances);
        let glow_storage = GlowStorage::new(wgpu_renderer, max_instances);
        let plasma_orb_storage = PlasmaOrbStorage::new(wgpu_renderer, max_instances);

        Self {
            particle_storage,
            glow_storage,
            plasma_orb_storage,
            max_instances,
        }
    }

    pub fn set_position(
        &mut self,
        index: usize,
        pos: cgmath::Vector3<f32>,
    ) {
        self.particle_storage.set_position(index, pos);
        self.glow_storage.set_position(index, pos);
        self.plasma_orb_storage.set_position(index, pos);
    }

    pub fn set_size(
        &mut self,
        index: usize,
        size: f32,
    ) {
        // self.particle_storage.set_size(index, size);
        self.glow_storage.set_size(index, size);
        self.plasma_orb_storage.set_size(index, size);
    }
}
