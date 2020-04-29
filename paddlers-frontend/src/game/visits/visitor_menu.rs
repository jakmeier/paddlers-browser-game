use crate::init::quicksilver_integration::Signal;
use crate::prelude::*;
use crate::view::*;
use core::marker::PhantomData;
use quicksilver::prelude::Window;

pub(crate) struct VisitorMenuFrame<'a, 'b> {
    _phantom: PhantomData<(&'a (), &'b ())>,
}

impl<'a, 'b> VisitorMenuFrame<'a, 'b> {
    pub fn new() -> Self {
        VisitorMenuFrame {
            _phantom: Default::default(),
        }
    }
}

impl<'a, 'b> Frame for VisitorMenuFrame<'a, 'b> {
    type Error = PadlError;
    type State = Game<'a, 'b>;
    type Graphics = Window;
    type Event = PadlEvent;
    type Signal = Signal;
    // TODO
    // fn left_click(
    //     &mut self,
    //     state: &mut Self::State,
    //     pos: (i32, i32),
    //     _signals: &mut ExperimentalSignalChannel,
    // ) -> Result<(), Self::Error> {
    //     Ok(())
    // }
}
