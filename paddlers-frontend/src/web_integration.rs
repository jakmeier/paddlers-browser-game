use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::prelude::{PadlError, PadlErrorCode, PadlResult};

#[must_use = "If this is dropped, the thread will be stopped"]
pub struct ThreadHandler {
    handle: i32,
    typ: ThreadType,
    // only kept here to be deleted on drop. not for use.
    function: Closure<dyn Fn()>,
}

enum ThreadType {
    AnimationFrame,
    Interval,
    Timeout,
    Unscheduled,
}

impl Drop for ThreadHandler {
    fn drop(&mut self) {
        match self.typ {
            ThreadType::AnimationFrame => {
                let _ = web_sys::window()
                    .unwrap()
                    .cancel_animation_frame(self.handle);
            }
            ThreadType::Interval => {
                web_sys::window()
                    .unwrap()
                    .clear_interval_with_handle(self.handle);
            }
            ThreadType::Timeout => {
                web_sys::window()
                    .unwrap()
                    .clear_timeout_with_handle(self.handle);
            }
            ThreadType::Unscheduled => {}
        }
    }
}

pub fn start_drawing_thread(f: impl Fn() + 'static) -> PadlResult<ThreadHandler> {
    let function = Closure::wrap(Box::new(f) as Box<dyn Fn()>);

    web_sys::window()
        .unwrap()
        .request_animation_frame(function.as_ref().dyn_ref().unwrap())
        .map(|handle| ThreadHandler {
            typ: ThreadType::AnimationFrame,
            handle,
            function,
        })
        .map_err(|_| PadlError::dev_err(PadlErrorCode::DevMsg("Failed on request_animation_frame")))
}

pub fn start_thread(f: impl Fn() + 'static, timeout_ms: i32) -> PadlResult<ThreadHandler> {
    let function = Closure::wrap(Box::new(f) as Box<dyn Fn()>);

    web_sys::window()
        .unwrap()
        .set_interval_with_callback_and_timeout_and_arguments_0(
            function.as_ref().dyn_ref().unwrap(),
            timeout_ms,
        )
        .map(|handle| ThreadHandler {
            typ: ThreadType::Interval,
            handle,
            function,
        })
        .map_err(|_| {
            PadlError::dev_err(PadlErrorCode::DevMsg(
                "Failed on set_interval_with_callback",
            ))
        })
}

pub fn create_thread(f: impl Fn() + 'static) -> ThreadHandler {
    let function = Closure::wrap(Box::new(f) as Box<dyn Fn()>);
    ThreadHandler {
        typ: ThreadType::Unscheduled,
        handle: 0,
        function,
    }
}

impl ThreadHandler {
    pub fn set_timeout(&mut self, timeout_ms: i32) -> PadlResult<()> {
        let handle = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                self.function.as_ref().dyn_ref().unwrap(),
                timeout_ms,
            )
            .map_err(|_| {
                PadlError::dev_err(PadlErrorCode::DevMsg("Failed on set_timeout_with_callback"))
            })?;
        match self.typ {
            ThreadType::Unscheduled | ThreadType::Timeout => {
                self.handle = handle;
                self.typ = ThreadType::Timeout;
            }
            ThreadType::AnimationFrame | ThreadType::Interval => {}
        }
        self.typ = ThreadType::Timeout;
        Ok(())
    }
}
