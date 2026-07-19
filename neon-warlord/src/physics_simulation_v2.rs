//! Next iteration of the verlet physics simulation

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::{agents::{agent_definitions::get_agent_0_definition, agent_factory::{self, AgentFactory}}, game_board::Agent, verlet_physics::{solver::Solver, verlet_composition::VerletComposition}};

pub struct PhysicsSimulationV2 {
    // Physics
    verlet_compositions: Vec<VerletComposition>,
    solver: Solver,
    ticks: u64,

    // Draw

    // Factories
    agent_factory: AgentFactory,
}

impl PhysicsSimulationV2 {
    
    pub fn new() -> Self {
        let verlet_compositions = Vec::new();
        let solver = Solver::new();
        let agent_factory = AgentFactory::new();
        Self { verlet_compositions, solver, agent_factory, ticks: 0 }
    }
    
    // Creation

    pub fn create_agent_0(&mut self) {
        let layers = get_agent_0_definition();
        let nodes = self.agent_factory.create_agent(&layers);
        let composition = VerletComposition::create(&nodes);
    
        self.verlet_compositions.push(composition);
    }  

    // Update

    pub fn update_physics(&mut self) 
    {
        let dt = 1.0 / 60.0;
        self.ticks += 1;

        self.solver.update_composites(
            &mut self.verlet_compositions, 
            dt,
        );
    }

    pub fn update_device(&mut self, 
        wgpu_renderer: &mut dyn WgpuRendererInterface
    ) {
        
    }
}

