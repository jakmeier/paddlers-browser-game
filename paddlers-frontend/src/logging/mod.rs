pub mod text_to_user;
pub mod error;
use error::*;
use text_to_user::*;
use std::collections::VecDeque;
use std::sync::{Mutex, mpsc::{Receiver, Sender}};

#[derive(Default)]
pub struct ErrorQueue {
    queue: VecDeque<PadlError>,
}
pub struct AsyncErr {
    chan: Mutex<Sender<PadlError>>,
}

impl ErrorQueue {
    pub fn push(&mut self, e: PadlError) { self.queue.push_front(e) }
    pub fn run(&mut self, tb: &mut TextBoard) {
        while let Some(e) = self.queue.pop_front() {
            self.route_err(e, tb);
        }
    }
    pub fn pull_async(&self, chan: &mut Receiver<PadlError>, tb: &mut TextBoard) {
        while let Ok(e) = chan.try_recv() {
            self.route_err(e, tb);
        }
    }
    fn route_err(&self, e: PadlError, tb: &mut TextBoard) {
        match e.channel {
            ErrorChannel::Technical => {
                println!("Error: {}", e);
                #[cfg(feature="mobile_debug")]
                tb.display_error_message(format!("DEBUG: {}", e));
            },
            ErrorChannel::UserFacing => 
                tb.display_error_message(format!("{}", e)),
        }
    }
}

impl AsyncErr {
    pub fn new(sender: Sender<PadlError>) -> Self {
        AsyncErr {
            chan: Mutex::new(sender)
        }
    } 
    pub fn clone_sender(&self) -> Sender<PadlError> {
        self.chan.lock().expect("locking mutex").clone()
    }
}