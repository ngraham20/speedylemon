use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode, EnterAlternateScreen, enable_raw_mode};
use core::fmt;
use std::{io, time::Duration};
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;
use crate::{speedylemon::{LemonContext, RaceState}, util::Timestamp};

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

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    chain_hook();

    Ok(terminal)
}

pub fn restore_terminal() -> Result<()> {
    // restore terminal
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn blit(ctx: &mut LemonContext) {
    // clear the screen
    print!("{esc}[2J{esc}[1;1H", esc=27 as char);

    let mut window = String::from(format!("Track: {}\n", ctx.course.name));

    match ctx.race_state {
        RaceState::Finished => {
            window.push_str("Race Finished!\n");
            window.push_str(&format!("Lap Time: {:?}", ctx.checkpoint_times.last().unwrap().timestamp()));
        },
        _ => {
            
            window.push_str(&format!("Checkpoint: {}\n", ctx.current_checkpoint));
            window.push_str(&format!("Distance to next checkpoint: {}\n", if ctx.current_checkpoint < ctx.course.checkpoints.len() {
                ctx.current_cp_distance()} else {
                    -1.0
                }));
            window.push_str(&format!("Distance to reset checkpoint: {}\n", ctx.reset_cp_distance().unwrap_or(-1.0)));
            window.push_str(&format!("Speed: {:?}\n", ctx.filtered_speed()));
            window.push_str("----- Checkpoint Times -----\n");
            for (idx, dur) in ctx.checkpoint_times.iter().enumerate() {
                window.push_str(&format!("Checkpoint: {}, Time: {}, Delta: {}\n", idx, dur.timestamp(), match idx {
                    0 => dur.timestamp(),
                    _ => dur.saturating_sub(ctx.checkpoint_times[idx-1]).timestamp()
                }))
            }
        }
    }
    print!("{}", window);
    
}