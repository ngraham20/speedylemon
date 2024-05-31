#[derive(Debug)]
pub enum TrackSelectorState {
    Unselected,
    SelectCup,
    SelectTrack,
}

pub struct StatefulList<T> {
    selected: Option<usize>,
    items: Vec<T>,
}

impl<T: Clone> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            selected: None,
            items,
        }
    }
    pub fn select(&mut self, idx: usize) {
        if idx < self.items.len() {
            self.selected = Some(idx);
        }
    }
    pub fn next(&mut self) {
        if let Some(s) = self.selected {
            if s + 1 < self.items.len() {
            self.selected = Some(s + 1);
            }
        } else {
            self.selected = Some(0);
        }
    }
    pub fn prev(&mut self) {
        if let Some(s) = self.selected {
            self.selected = Some(usize::saturating_sub(s, 1));
        }else {
            self.selected = Some(0);
        }
    }
    pub fn clear(&mut self) {
        self.selected = None;
        self.items = vec![];
    }
    pub fn selected(&self) -> Option<&T> {
        if let Some(idx) = self.selected {
            return Some(&self.items[idx])
        }
        None
    }
}

pub struct TrackSelector {
    pub state: TrackSelectorState,
    pub cups: StatefulList<String>,
    pub tracks: StatefulList<String>,
}

impl TrackSelector {
    pub fn build_pane(&self) -> String {
        const SEPARATOR: &str = " | ";
        let mut lines = String::new();
        lines += "Track Selector\n";
        lines += &format!("State: {:?}\n", self.state);
        lines += &format!("Cup Selected: {:?}\n", self.cups.selected);
        lines += &format!("Track Selected: {:?}\n", self.tracks.selected);
        let length = usize::max(self.cups.items.len(), self.tracks.items.len());
        for idx in 0..length {
            let mut cuptext = self.cups.items.get(idx).unwrap_or(&String::new()).clone();
            let mut tracktext = self.tracks.items.get(idx).unwrap_or(&String::new()).clone();
            if let Some(sel) = self.cups.selected {
                if idx == sel {
                    cuptext = format!("*{}", cuptext);
                }
            }
            if let Some(sel) = self.tracks.selected {
                if idx == sel {
                    tracktext = format!("*{}", tracktext);
                }
            }
            lines += &format!("| {:<20}{}{:<20} |\n", cuptext, SEPARATOR, tracktext);
        }
        lines
    }
}