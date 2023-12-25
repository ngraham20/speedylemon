mod tui_utils;
mod track_selector;
mod speedometer;

pub use tui_utils::*;
pub use track_selector::*;
pub use speedometer::*;

#[derive(Clone, Copy)]
pub enum AppState {
    Speedometer,
    PickCup,
    PickTrack,
}

pub struct App {
    pub state: AppState,
    pub track_selector: StatefulTrackSelector,
}