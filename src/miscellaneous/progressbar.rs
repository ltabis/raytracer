use std::io;
use std::io::prelude::*;
use std::sync::mpsc;
use std::{
    thread::{self},
    time,
};
use time::Instant;

pub struct ProgressBar {
    sender: mpsc::Sender<f64>,
}

impl ProgressBar {
    pub fn new(max_value: f64, number_of_lines: u32) -> ProgressBar {
        let (tx, rx): (mpsc::Sender<f64>, mpsc::Receiver<f64>) = mpsc::channel();

        // USED FOR A SMOOTHER SPINNING THING IN FRONT OF THE LOADING BAR.
        let mut now = Instant::now();
        let mut total_time: f64 = 0.0f64;

        thread::spawn(move || {
            let max = max_value;
            let mut progress = 0.0;
            let lines = number_of_lines;
            let loading: [char; 4] = ['-', '\\', '|', '/'];

            let mut i = 0;
            for received in rx {
                progress += received;
                let completed = (progress / max * lines as f64) as usize;
                let remaining = (lines as usize) - completed;
                let percentage = (progress / max * 100.0) as usize;

                print!(
                    "\r[{}] {}% [{}{}]",
                    loading[i],
                    percentage,
                    "*".repeat(completed),
                    " ".repeat(remaining)
                );

                total_time = now.elapsed().as_secs_f64();
                i = (i + if total_time > 0.25f64 {
                    now = Instant::now();
                    1
                } else {
                    0
                }) % 4;

                // TODO : Better error handling.
                io::stdout().flush().ok().expect("Could not flush stdout");
            }

            println!("\nDone!");
        });

        ProgressBar { sender: tx }
    }

    pub fn inc(&mut self, n: f64) {
        self.sender.send(n).unwrap();
    }
}
