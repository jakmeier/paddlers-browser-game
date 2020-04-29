use crate::init::quicksilver_integration::Signal;
use crate::prelude::*;
use crate::view::*;
use core::marker::PhantomData;
use quicksilver::prelude::Window;

pub(crate) struct ReportFrame<'a, 'b> {
    _phantom: PhantomData<(&'a (), &'b ())>,
}

impl<'a, 'b> ReportFrame<'a, 'b> {
    pub fn new() -> Self {
        ReportFrame {
            _phantom: Default::default(),
        }
    }
}

impl<'a, 'b> Frame for ReportFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;
    type Event = PadlEvent;
    type Signal = Signal;
}
