use crate::DEBUG;

#[derive(Debug)]
pub enum TrackSelectorState {
    Unselected,
    SelectCup,
    SelectTrack,
}

pub struct TrackSelector {
    pub state: TrackSelectorState,
    pub cups: StatefulScrollingList<String>,
    pub tracks: StatefulScrollingList<String>,
}
use feotui::StatefulScrollingList;