//! Manages all the orb objects on the GPU

use forward_renderer::particle_storage::ParticleStorage;

struct OrbStorage {
    particle_storage: ParticleStorage,

    max_instances: usize,
}

impl OrbStorage {
    fn new(
        wgpu_renderer: &mut dyn wgpu_renderer::wgpu_renderer::WgpuRendererInterface,
        max_instances: usize,
    ) -> Self {

        let particle_storage = ParticleStorage::new(wgpu_renderer, max_instances);

        Self { particle_storage, max_instances }
    }
}