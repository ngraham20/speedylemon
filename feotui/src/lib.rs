use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use anyhow::Result;

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

pub enum BorderStyle {
    None,
    Solid,
    Bold,
}

pub struct Window {
    width: usize,
    height: usize,
    lines: Vec<String>,
}

pub trait Blit {
    fn blit(&self);
}

pub struct WindowBuilder {
    data: Window,
}

impl WindowBuilder {
    pub fn build(self) -> String {
        self.data.lines.join("\n")
    }
    pub fn with_width(&mut self, width: usize) -> &Self {
        self.data.width = width;
        self
    }
    pub fn with_height(&mut self, height: usize) -> &Self {
        self.data.height = height;
        self
    }
}

pub fn chain_hook() {
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        restore_terminal().unwrap();
        original_hook(panic);
    }));
}

pub fn restore_terminal() -> Result<()> {
    // restore terminal
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn init_terminal() -> Result<()> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    chain_hook();

    Ok(())
}