//! Runs the physics simulation

use std::{sync::mpsc, time::Duration};

use forward_renderer::{HeightMap, TerrainTextureDetails};
use wgpu_renderer::performance_monitor::{Fps, watch::{self, Watch}};

use crate::{ant_ai::AntAi, ant_ai_controller::AntAiController, ant_state::{AntPosition, AntState}, game_board::{Faction, GameBoard}, heightmap_generator::HeightMapGenerator};

const WATCH_POINTS_SIZE: usize = 10;

pub enum MainMessage {
    GetTerrain(TerrainTextureDetails), // Requests the terrain heightmap
}

pub enum WorkerMessage {
    UpdateWatchPoints(watch::WatchViewerData<10>), // all the data for a point of the performance monitor
    Ups(u32),
    TerrainData(HeightMap),
    AntState(AntPosition),
}

pub struct Worker {
    channel_0_rx: mpsc::Receiver<MainMessage>,
    channel_1_tx: mpsc::Sender<WorkerMessage>,

    // Debug
    ups: Fps,

    watch_ups: Watch<WATCH_POINTS_SIZE>,

    // Terrain
    terrain_generator: HeightMapGenerator,

    // Game board
    game_board:GameBoard,

    // Ant
    ant_state: AntState,
    ant_ai: AntAi,
}

impl Worker {
    pub fn new(
        channel_0_rx: mpsc::Receiver<MainMessage>,
        channel_1_tx: mpsc::Sender<WorkerMessage>,
    ) -> Self {
        // Debug
        let ups = Fps::new();
        let watch_ups = Watch::new();

        // Terrain
        let terrain_generator = HeightMapGenerator::new();

        // Game board
        let game_board = GameBoard::new();

        // Ant
        let ant_state = AntState::new();
        let ant_ai = AntAi::new(Faction::Blue);

        Self {
            channel_0_rx,
            channel_1_tx,

            ups,
            watch_ups,

            terrain_generator,

            game_board,

            ant_state,
            ant_ai,
        }
    }

    pub fn update(&mut self, _tick: u64, dt: Duration) {
        let main = &self.channel_1_tx;
        let messages = &self.channel_0_rx;

        // Update watch
        self.watch_ups.update();
        let watch_ups_data = self.watch_ups.get_viewer_data();
        let _ = main.send(WorkerMessage::UpdateWatchPoints(watch_ups_data));
        let mut watch_index = 0;

        // update ups
        self.ups.update(dt);
        let _ = main.send(WorkerMessage::Ups(self.ups.get()));

        // Process messages
        let mut terrain_detail = Vec::new();
        self.watch_ups.start(watch_index, "Messages");
        {
            for message in messages.try_iter() {
                match message {
                    MainMessage::GetTerrain(terrain_texture_details) => {
                        // ##########################################################
                        terrain_detail.push(terrain_texture_details);
                    }
                }
            }
        }
        self.watch_ups.stop(watch_index);

        // Terrain
        watch_index += 1;
        self.watch_ups.start(watch_index, "Update Terrain");
        {
            for elem in terrain_detail {
                let terrain_part = self.terrain_generator.generate(&elem);
                let _ = main.send(WorkerMessage::TerrainData(terrain_part));
            }
        }
        self.watch_ups.stop(watch_index);

        // Ant
        watch_index += 1;
        self.watch_ups.start(watch_index, "Update Ant");
        {
            // update ant_state by ant_ai
            self.ant_ai.update(&mut AntAiController {
                game_board: &mut self.game_board,
                ant_state: &mut self.ant_state,
                ant_index: 0,
            });

            // update state
            let ant_pos = self.ant_state.update();

            // send to main
            let _ = main.send(WorkerMessage::AntState(ant_pos));
        }
        self.watch_ups.stop(watch_index);
    }
}
