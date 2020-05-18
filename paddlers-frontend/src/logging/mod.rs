pub mod error;
pub mod statistics;
pub mod text_to_user;
use error::*;
use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;
use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};
use text_to_user::*;

thread_local!(
    static STATIC_ERROR_QUEUE: Rc<RefCell<ErrorQueueSingleton>> =
        Rc::new(RefCell::new(ErrorQueueSingleton {
            queue: VecDeque::new(),
        }))
);

struct ErrorQueueSingleton {
    queue: VecDeque<PadlError>,
}

/// An endpoint to the globally shared error queue singleton.
/// A new instance of this endpoint can be retrieved anywhere, including event handlers.
pub struct ErrorQueue(Rc<RefCell<ErrorQueueSingleton>>);

pub struct AsyncErr {
    chan: Mutex<Sender<PadlError>>,
}

impl ErrorQueue {
    pub fn new_endpoint() -> Self {
        Self(STATIC_ERROR_QUEUE.with(|q| q.clone()))
    }
    pub fn push(&mut self, e: PadlError) {
        self.queue().push_front(e)
    }
    pub fn run(&mut self, tb: &mut TextBoard) {
        while let Some(e) = self.queue().pop_front() {
            self.route_err(e, tb);
        }
    }
    pub fn pull_async(&self, chan: &mut Receiver<PadlError>, tb: &mut TextBoard) {
        while let Ok(e) = chan.try_recv() {
            self.route_err(e, tb);
        }
    }
    fn queue(&self) -> RefMut<VecDeque<PadlError>> {
        RefMut::map(self.0.borrow_mut(), |singleton| &mut singleton.queue)
    }
    fn route_err(&self, e: PadlError, tb: &mut TextBoard) {
        let err = match e.channel {
            ErrorChannel::Technical => {
                println!("Error: {}", e);
                #[cfg(feature = "mobile_debug")]
                let err = tb.display_error_message(format!("DEBUG: {}", e));
                #[cfg(not(feature = "mobile_debug"))]
                let err = Ok(());
                err
            }
            ErrorChannel::UserFacing => tb.display_error_message(format!("{}", e)),
        };
        if let Err(err) = err {
            println!("Failed to display error. Reason of failure: {}", err);
        }
    }
}

impl AsyncErr {
    pub fn new(sender: Sender<PadlError>) -> Self {
        AsyncErr {
            chan: Mutex::new(sender),
        }
    }
    pub fn clone_sender(&self) -> Sender<PadlError> {
        self.chan.lock().expect("locking mutex").clone()
    }
}
impl Clone for AsyncErr {
    fn clone(&self) -> Self {
        Self::new(self.clone_sender())
    }
}
