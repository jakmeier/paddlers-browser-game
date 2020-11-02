use std::{cell::RefCell, rc::Rc};

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

use crate::prelude::{PadlError, PadlErrorCode, PadlResult};

#[must_use = "If this is dropped, the thread will be stopped"]
pub struct ThreadHandler {
    handle: i32,
    function: FunctionType,
}

enum FunctionType {
    AnimationFrame(Rc<RefCell<Option<Closure<dyn Fn(f64)>>>>),
    Interval(Closure<dyn Fn()>),
    Timeout(Closure<dyn Fn()>),
}

impl Drop for ThreadHandler {
    fn drop(&mut self) {
        match &mut self.function {
            FunctionType::AnimationFrame(f) => {
                let _ = web_sys::window()
                    .unwrap()
                    .cancel_animation_frame(self.handle);
                *f.borrow_mut() = None;
            }
            FunctionType::Interval(_) => {
                web_sys::window()
                    .unwrap()
                    .clear_interval_with_handle(self.handle);
            }
            FunctionType::Timeout(_) => {
                web_sys::window()
                    .unwrap()
                    .clear_timeout_with_handle(self.handle);
            }
        }
    }
}

/// Sets up a request animation frame loop
pub fn start_drawing_thread(f: impl Fn(f64) + 'static) -> PadlResult<ThreadHandler> {
    // Allocate some memory for a function pointer and initialize it with a null pointer
    let function = Rc::new(RefCell::new(None));
    let function_alias = function.clone();

    let closure = move |dt: f64| {
        if let Some(function) = function.borrow().as_ref() {
            f(dt);
            request_animation_frame(function).expect("RAF failed");
        }
        // else: Handle has been dropped, this means no more drawing
    };
    *function_alias.borrow_mut() = Some(Closure::<Fn(f64)>::wrap(
        Box::new(closure) as Box<dyn Fn(f64)>
    ));

    let handle = request_animation_frame(function_alias.borrow().as_ref().unwrap())?;
    Ok(ThreadHandler {
        function: FunctionType::AnimationFrame(function_alias),
        handle,
    })
}

// Only requests a single frame, needs to be called repeatedly for each frame.
fn request_animation_frame(function: &Closure<Fn(f64)>) -> PadlResult<i32> {
    let handle = web_sys::window()
        .unwrap()
        .request_animation_frame(function.as_ref().dyn_ref().unwrap())
        .map_err(|_| {
            PadlError::dev_err(PadlErrorCode::DevMsg("Failed on request_animation_frame"))
        })?;
    Ok(handle)
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
            function: FunctionType::Interval(function),
            handle,
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
        function: FunctionType::Timeout(function),
        handle: 0,
    }
}

impl ThreadHandler {
    pub fn set_timeout(&mut self, timeout_ms: i32) -> PadlResult<()> {
        match &mut self.function {
            FunctionType::Timeout(function) | FunctionType::Interval(function) => {
                let handle = web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        function.as_ref().dyn_ref().unwrap(),
                        timeout_ms,
                    )
                    .map_err(|_| {
                        PadlError::dev_err(PadlErrorCode::DevMsg(
                            "Failed on set_timeout_with_callback",
                        ))
                    })?;
                self.handle = handle;
            }
            FunctionType::AnimationFrame(_) => panic!("Called set_timeout on drawing thread."),
        }
        Ok(())
    }
}
