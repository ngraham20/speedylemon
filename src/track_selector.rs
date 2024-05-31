use crate::{basictui::StatefulList, DEBUG};

#[derive(Debug)]
pub enum TrackSelectorState {
    Unselected,
    SelectCup,
    SelectTrack,
}

pub struct TrackSelector {
    pub state: TrackSelectorState,
    pub cups: StatefulList<String>,
    pub tracks: StatefulList<String>,
}
use colored::Colorize;
impl TrackSelector {

    pub fn build_pane(&self) -> String {
        const SEPARATOR: &str = " ┃ ";
        let mut lines: Vec<String> = Vec::new();
        lines.push("Track Selector".to_string());

        if DEBUG.get() {
            lines.push("━━━━━ DEBUG ━━━━━".to_string());
            lines.push(format!("State: {:?}", self.state));
            lines.push(format!("Cup Selected: {:?}", self.cups.selected));
            lines.push(format!("Track Selected: {:?}", self.tracks.selected));
            lines.push("━━━━━━━━━━━━━━━━━".to_string());
        }

        let length = usize::max(self.cups.items.len(), self.tracks.items.len());
        for idx in 0..length {
            let cuptext = self.cups.items.get(idx).unwrap_or(&String::new()).clone();
            let tracktext = self.tracks.items.get(idx).unwrap_or(&String::new()).clone();
            let csel = if let Some(sel) = self.cups.selected {
                idx == sel
            } else { false };
            let tsel = if let Some(sel) = self.tracks.selected {
                idx == sel
            } else { false };
            lines.push(format!("┃ {:<20}{}{:<30} ┃", if csel {format!("*{}", cuptext).blue()} else { cuptext.white()}, SEPARATOR, if tsel {format!("*{}", tracktext).blue()} else {tracktext.white()}));
        }
        lines.join("\n")
    }
}