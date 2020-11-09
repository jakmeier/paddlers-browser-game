pub mod error;
pub mod statistics;
use error::*;
use paddle::*;
use std::collections::VecDeque;

struct ErrorQueue {
    queue: VecDeque<PadlError>,
}

/// Set up an error queue activity running in the background that displays any published PadlError objects.
pub fn init_error_handling() {
    let errq = ErrorQueue::new();
    let errq_id = nuts::new_activity(errq);
    errq_id.subscribe(|q, err: &PadlError| {
        q.route_err(err);
        q.run();
    });
    paddle::enable_nuts_checks(|error| match error.channel {
        MessageChannel::Technical => {
            web_sys::console::error_1(&error.text.into());
        }
        MessageChannel::UserFacing => {
            TextBoard::display_error_message(error.text);
        }
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
            println!("Failed to display error. Reason of failure: {:?}", err);
        }
    }
}
