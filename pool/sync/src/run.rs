use std::thread;

pub trait Run : Sync {
    fn run(&self) -> Result<(), String>;
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