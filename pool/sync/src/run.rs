use std::thread;
use anyhow::Result;

pub trait Run : Sync {
    fn run(&self) -> Result<()>;
}

pub fn run_in_parallel(runnable: Vec<&dyn Run>) {
    thread::scope(|s| {
        for run in runnable {
            s.spawn(|| {
                run.run().unwrap();
            });
        }
    });
}