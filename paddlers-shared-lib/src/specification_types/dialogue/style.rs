use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
pub enum SlideTextStyle {
    SpeechBubbleToLeft,
    PlayerSpeech,
    SystemMessage,
}

impl Default for SlideTextStyle {
    fn default() -> Self {
        Self::SpeechBubbleToLeft
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize)]
pub enum ButtonLayout {
    SingleColumn,
    SingleRow,
}

impl Default for ButtonLayout {
    fn default() -> Self {
        Self::SingleRow
    }
}
