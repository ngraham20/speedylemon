use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode, EnterAlternateScreen, enable_raw_mode};
use std::io;
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;
use crate::speedylemon::LemonContext;

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

pub fn ui(f: &mut Frame, ctx: &mut LemonContext) {
    // let cp_text = match ctx.current_checkpoint {
    //     0 => { String::from("Waiting for player to cross starting line") },
    //     _idx if _idx > 0 && _idx < ctx.course.index_last_checkpoint() => {format!("Distance to checkpoint {}: {}", ctx.current_checkpoint, ctx.current_cp_distance())},
    //     _idx if _idx == ctx.course.index_last_checkpoint() => {format!("Distance to finish line: {}", ctx.current_cp_distance())},
    //     _ => { panic!("Unreachable State") },
    // };

    let cp_text = match ctx.current_checkpoint {
        idx if idx < ctx.course.checkpoints.len() => format!("Distance to checkpoint {}: {}", ctx.current_checkpoint, ctx.current_cp_distance()),
        _ => format!("Race Finished!")
    };
    
    let size = f.size();
    let layout = Layout::default()
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(size);
    let checkpoint = Block::default()
        .title(format!("Current Checkpoint: {} ", ctx.current_checkpoint))
        .borders(Borders::ALL);
    let cpdata = Paragraph::new(cp_text);

    let times: Vec<ListItem> = ctx.checkpoint_times
        .iter()
        .enumerate()
        .map(|(idx, _idx)| {
            ListItem::new(vec![
                Line::from(""),
                Line::from(format!("Checkpoint: {}, Time: {}", idx, ctx.checkpoint_times[idx].as_millis()))
            ])
        }).collect();

    f.render_widget(cpdata.clone().block(checkpoint), layout[0]);
    let checkpoints_list = List::new(times)
        .block(Block::default().borders(Borders::ALL).title("Checkpoints"));
    f.render_widget(checkpoints_list, layout[1]);
}