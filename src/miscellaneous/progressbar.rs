use std::io::prelude::*;
use std::sync::mpsc;
use std::thread;
use std::io;


pub struct ProgressBar {

    sender:     mpsc::Sender<f64>,
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

                print!("\r[{}] {}% [", loading[i], (progress / max * 100.0) as u32);
                for _ in 0..(progress / max * lines as f64) as u32 {
                    print!("*");
                }
                for _ in 0..(lines - (progress / max * lines as f64) as u32) {
                    print!(" ");
                }
                print!("]");
                i = if i < 3 { i + 1 } else { 0 };

                // TODO : Better error handling.
                io::stdout().flush().ok().expect("Could not flush stdout");
            }
        });

        ProgressBar {
            sender: tx
        }
    }

    pub fn inc(&mut self, n: f64) {
        self.sender.send(n).unwrap();
    }
}