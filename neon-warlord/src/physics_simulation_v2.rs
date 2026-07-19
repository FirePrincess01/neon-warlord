//! Next iteration of the verlet physics simulation

use std::ops::DerefMut;

use wgpu_renderer::wgpu_renderer::WgpuRendererInterface;

use crate::{agents::{agent_definitions::get_agent_0_definition, agent_drawer::AgentDrawer, agent_factory::{self, AgentFactory}}, game_board::Agent, verlet_physics::{solver::Solver, verlet_composition::VerletComposition}};

pub struct PhysicsSimulationV2 {
    // Physics
    verlet_compositions: Vec<VerletComposition>,
    solver: Solver,
    ticks: u64,

    // Draw
    drawer: Vec<Drawer>,

    // Factories
    agent_factory: AgentFactory,
}

impl PhysicsSimulationV2 {
    
    pub fn new() -> Self {
        let verlet_compositions = Vec::new();
        let solver = Solver::new();
        let drawer = Vec::new();
        let agent_factory = AgentFactory::new();
        
        Self {
            verlet_compositions,
            solver,
            ticks: 0,
            drawer,
            agent_factory,
        }
    }
    
    // Creation

    pub fn create_agent_0(
        &mut self,
        wgpu_renderer: &mut dyn WgpuRendererInterface,
        
    ) {
        let layers = get_agent_0_definition();
        let nodes = self.agent_factory.create_agent(&layers);
        let composition = VerletComposition::create(&nodes);
        let drawer = AgentDrawer::new(
            wgpu_renderer, 
            &composition
        );

        self.verlet_compositions.push(composition);
        self.drawer.push(Drawer::AgentDrawer(drawer));
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

    pub fn update_device(
        &mut self, 
        wgpu_renderer: &mut dyn WgpuRendererInterface
    ) {
        assert!(self.drawer.len() == self.verlet_compositions.len());

        let size = self.drawer.len();
        for i in 0..size {
            match &mut self.drawer[i] {
                Drawer::AgentDrawer(agent_drawer) => {
                    agent_drawer.update(
                        wgpu_renderer, 
                        &self.verlet_compositions[i]
                    );
                },
            }
        }

    }
}

enum Drawer {
    AgentDrawer(AgentDrawer)
}