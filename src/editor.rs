use nih_plug::prelude::{Editor};

use crate::MidiInverterParams;

struct IcedEditorHandle<Message: 'static + Send> {
    iced_state: Arc<IcedState>,
    window: iced_baseview::WindowHandle<Message>,
}

pub(crate) fn create(_params: Arc<MidiInverterParams>) -> Option<Box<dyn Editor>> {
    Box::new(IcedEditorHandle {
        iced_state: self.iced_state.clone(),
        window,
    })
}