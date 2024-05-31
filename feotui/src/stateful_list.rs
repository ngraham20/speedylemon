pub struct StatefulList<T> {
    pub selected: Option<usize>,
    pub items: Vec<T>,
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