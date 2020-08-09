pub mod error;
pub mod statistics;
pub mod text_to_user;
use error::*;
use std::collections::VecDeque;
use text_to_user::*;

struct ErrorQueue {
    queue: VecDeque<PadlError>,
}

/// Set up an error queue activity running in the background that displays any published PadlError objects.
pub fn init_error_handling() {
    let errq = ErrorQueue::new();
    let errq_id = nuts::new_activity(errq, true);
    errq_id.subscribe(|q, err: &PadlError| {
        q.route_err(err);
        q.run();
    });
}

impl ErrorQueue {
    fn new() -> Self {
        ErrorQueue {
            queue: VecDeque::new(),
        }
    }
    fn run(&mut self) {
        while let Some(e) = self.queue().pop_front() {
            self.route_err(&e);
        }
    }
    fn queue(&mut self) -> &mut VecDeque<PadlError> {
        &mut self.queue
    }
    fn route_err(&self, e: &PadlError) {
        let err = match e.channel {
            ErrorChannel::Technical => {
                println!("Error: {}", e);
                #[cfg(feature = "mobile_debug")]
                let err = TextBoard::display_error_message(format!("DEBUG: {}", e));
                #[cfg(not(feature = "mobile_debug"))]
                let err = Ok(());
                err
            }
            ErrorChannel::UserFacing => TextBoard::display_error_message(format!("{}", e)),
        };
        if let Err(err) = err {
            println!("Failed to display error. Reason of failure: {}", err);
        }
    }
}

pub trait NutsCheck {
    fn nuts_check(self);
}

impl NutsCheck for PadlResult<()> {
    fn nuts_check(self) {
        if let Err(e) = self {
            nuts::publish(e)
        }
    }
}
