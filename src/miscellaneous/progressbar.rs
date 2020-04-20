use std::io;
use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;

pub struct ProgressBar {
    sender: mpsc::Sender<f64>,
}

impl ProgressBar {
    pub fn new(max_value: f64, number_of_lines: u32) -> ProgressBar {
        let (tx, rx): (mpsc::Sender<f64>, mpsc::Receiver<f64>) = mpsc::channel();

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

                print!("\r[{}] {}% [{}{}]",
                       loading[i],
                       percentage,
                       "*".repeat(completed),
                       " ".repeat(remaining));

                i = (i + 1) % 3;

                // TODO : Better error handling.
                io::stdout().flush().ok().expect("Could not flush stdout");
            }
        });

        ProgressBar { sender: tx }
    }

    pub fn inc(&mut self, n: f64) {
        self.sender.send(n).unwrap();
    }
}
