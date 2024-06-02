use std::fmt::Display;

pub enum ScrollStyle {
    None,
    Paging,
    Scrolling,
}

pub struct StatefulScrollingList<T> {
    pub selected: Option<usize>,
    pub viewport_length: usize,
    pub offset: usize,
    pub items: Vec<T>,
    pub scroll_style: ScrollStyle,
}

impl<T: Clone + Display> StatefulScrollingList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            selected: None,
            offset: 0,
            viewport_length: 0,
            items,
            scroll_style: ScrollStyle::None,
        }
    }
    pub fn with_scroll_style(mut self, style: ScrollStyle) -> Self {
        self.scroll_style = style;
        self
    }
    pub fn with_viewport_length(mut self, length: usize) -> Self {
        self.viewport_length = length;
        self
    }
    pub fn select(&mut self, idx: usize) {
        if idx < self.items.len() {
            self.selected = Some(idx);
            match self.scroll_style {
                ScrollStyle::Paging => {
                    // idx / self.viewport_length is *integer division*. This means that the division
                    // and multiplication *do not cancel out*
                    self.offset = (idx / self.viewport_length) * self.viewport_length;
                },
                ScrollStyle::Scrolling => {self.offset = if idx >= self.viewport_length {idx - self.viewport_length+1} else { 0 }},
                _ => {},
            }
        }
    }
    pub fn next(&mut self) {
        if let Some(s) = self.selected {
            self.select(s + 1);
        } else {
            self.select(0);
        }
    }
    pub fn prev(&mut self) {
        if let Some(s) = self.selected {
            self.select(usize::saturating_sub(s, 1));
        }else {
            self.select(0);
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
    pub fn viewport_string(&self) -> String {
        self.viewport().join("\n")
    }

    pub fn viewport(&self) -> Vec<String> {
        self.items[self.offset..usize::min(self.viewport_length+self.offset, self.items.len())].iter().enumerate()
            .map(|(idx, line)| if let Some(sel_idx) = self.selected {
                if idx+self.offset == sel_idx {
                    format!("*{}", line)
                } else {
                    format!("{}", line)
                }
            } else {
                format!("{}", line)
            }).collect::<Vec<String>>()
    }
}