pub mod text_to_user;
pub mod error;
use error::*;
use text_to_user::*;

#[derive(Default)]
pub struct ErrorQueue(std::collections::VecDeque<PadlError>);

impl ErrorQueue {
    pub fn push(&mut self, e: PadlError) { self.0.push_front(e) }
    pub fn run(&mut self, tb: &mut TextBoard) {
        if let Some(e) = self.0.pop_front() {
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
}