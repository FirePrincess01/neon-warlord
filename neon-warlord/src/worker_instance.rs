//! Instantiate a worker either single threaded or multi threaded

use std::{sync::mpsc, thread, time::Duration};

use crate::worker::{MainMessage, Worker, WorkerMessage};

pub struct WorkerInstance {
    instance: WorkerExecution,
}

impl WorkerInstance {
    pub fn new() -> Self {
        let instance;
        #[cfg(target_arch = "wasm32")]
        {
            instance = WorkerExecution::SingleThreaded(WorkerSingleThreaded::new());
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            instance = WorkerExecution::Multithreaded(WorkerMultiThreaded::new());
        }

        Self { instance }
    }

    pub fn update(&mut self, dt: Duration) {
        match &mut self.instance {
            WorkerExecution::SingleThreaded(instance) => {
                instance.update(dt);
            }
            WorkerExecution::Multithreaded(_instance) => {
                // update is done in another thread
            }
        }
    }

    pub fn send(&self) -> &mpsc::Sender<MainMessage> {
        match &self.instance {
            WorkerExecution::SingleThreaded(instance) => &instance.channel_0_tx,
            WorkerExecution::Multithreaded(instance) => &instance.channel_0_tx,
        }
    }

    pub fn receive(&self) -> &mpsc::Receiver<WorkerMessage> {
        match &self.instance {
            WorkerExecution::SingleThreaded(instance) => &instance.channel_1_rx,
            WorkerExecution::Multithreaded(instance) => &instance.channel_1_rx,
        }
    }
}

#[allow(dead_code)] // unused in wasm
enum WorkerExecution {
    SingleThreaded(WorkerSingleThreaded),
    Multithreaded(WorkerMultiThreaded),
}

pub struct WorkerSingleThreaded {
    channel_0_tx: mpsc::Sender<MainMessage>,
    channel_1_rx: mpsc::Receiver<WorkerMessage>,

    worker: Box<Worker>,

    tick: u64,
}

#[allow(dead_code)] // unused in x86
impl WorkerSingleThreaded {
    pub fn new() -> Self {
        let (channel_0_tx, channel_0_rx) = mpsc::channel();
        let (channel_1_tx, channel_1_rx) = mpsc::channel();

        let worker = Box::new(Worker::new(channel_0_rx, channel_1_tx));

        let tick = 0;

        Self {
            channel_0_tx,
            channel_1_rx,
            worker,
            tick,
        }
    }

    fn update(&mut self, dt: Duration) {
        self.worker.update(self.tick, dt);

        self.tick += 1;
    }
}

pub struct WorkerMultiThreaded {
    channel_0_tx: mpsc::Sender<MainMessage>,
    channel_1_rx: mpsc::Receiver<WorkerMessage>,

    _worker: thread::JoinHandle<()>,
}

#[allow(dead_code)] // unused in wasm
impl WorkerMultiThreaded {
    pub fn new() -> Self {
        let (channel_0_tx, channel_0_rx) = mpsc::channel();
        let (channel_1_tx, channel_1_rx) = mpsc::channel();

        let worker = thread::spawn(move || {
            let mut worker = Worker::new(channel_0_rx, channel_1_tx);

            let mut tick = 0;

            let mut last_update_time = instant::Instant::now();

            loop {
                let start_time = instant::Instant::now();
                let dt = start_time - last_update_time;
                last_update_time = start_time;
                worker.update(tick, dt);
                tick += 1;
                let stop_time = instant::Instant::now();

                let time_passed = stop_time - start_time;

                let interval = Duration::from_millis(16);
                if time_passed < interval {
                    thread::sleep(interval - time_passed);
                }
            }
        });

        Self {
            channel_0_tx,
            channel_1_rx,
            _worker: worker,
        }
    }
}
