use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode, EnterAlternateScreen, enable_raw_mode};
use std::{io, time::Duration};
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;
use crate::speedylemon::{LemonContext, RaceState};

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

trait Timestamp {
    fn timestamp(&self) -> String;
}

impl Timestamp for Duration {
    fn timestamp(&self) -> String {
        format!("{:02}:{:02}:{:03}", self.as_secs()/60, self.as_secs()%60, self.subsec_millis())
    }
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