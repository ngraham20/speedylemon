use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode, EnterAlternateScreen, enable_raw_mode};
use std::{io, time::Duration};
use ratatui::{prelude::*, widgets::*};
use anyhow::Result;
use crate::speedylemon::{LemonContext, RaceState, App};

pub struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> where T: Clone {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn selected_item(&self) -> Option<T> {
        if let Some(idx) = self.state.selected() {
            Some(self.items[idx].clone())
        } else { None }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
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

trait Timestamp {
    fn timestamp(&self) -> String;
}

impl Timestamp for Duration {
    fn timestamp(&self) -> String {
        format!("{:02}:{:02}:{:03}", self.as_secs()/60, self.as_secs()%60, self.subsec_millis())
    }
}

pub fn map_selection(f: &mut Frame, app: &mut App) {
    let size = f.size();
    let layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
    .split(size);

    let cups_list: Vec<ListItem> = app
        .cups
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.clone()).style(Style::default().fg(Color::White).bg(Color::Reset))
        })
        .collect();

    let cups_widget = List::new(cups_list)
        .block(Block::default().borders(Borders::ALL).title("Select Cup"))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(cups_widget, layout[0], &mut app.cups.state);


    let tracks_list: Vec<ListItem> = app
        .tracks
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.clone()).style(Style::default().fg(Color::White).bg(Color::Reset))
        })
        .collect();

    let tracks_widget = List::new(tracks_list)
        .block(Block::default().borders(Borders::ALL).title("Select Track"))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    f.render_stateful_widget(tracks_widget, layout[1], &mut app.tracks.state);
}

pub fn ui(f: &mut Frame, ctx: &mut LemonContext) {

    let debug_data = vec![
        Line::from(match ctx.current_checkpoint {
            idx if idx < ctx.course.checkpoints.len() => format!("Distance to checkpoint {}: {}", ctx.current_checkpoint, ctx.current_cp_distance()),
            _ => format!("Race Finished!") 
        }),
        Line::from(format!("Race State: {:?}", ctx.race_state)),
        Line::from(format!("Time: {}", match ctx.race_state {
            RaceState::WaitingToStart => Duration::new(0,0).timestamp(),
            RaceState::Racing => ctx.start_time.elapsed().timestamp(),
            RaceState::Finished => ctx.checkpoint_times.last().unwrap().timestamp(),
        })),
        Line::from(format!("Current checkpoint: {}", ctx.current_checkpoint)),
        Line::from(format!("Velocity: {}", ctx.filtered_velocity())),
    ];
    
    let size = f.size();
    let layout = Layout::default()
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(size);
    let checkpoint = Block::default()
        .title(format!("{} Current Checkpoint: {} ", ctx.course.name, ctx.current_checkpoint))
        .borders(Borders::ALL);
    let cpdata = Paragraph::new(debug_data);

    let times: Vec<ListItem> = ctx.checkpoint_times
        .iter()
        .enumerate()
        .map(|(idx, dur)| {
            ListItem::new(vec![
                Line::from(""),
                Line::from(format!("Checkpoint: {}, Time: {}, Delta: {}", idx, dur.timestamp(), match idx {
                    0 => dur.timestamp(),
                    _ => dur.saturating_sub(ctx.checkpoint_times[idx-1]).timestamp()
                })),
            ])
        }).collect();

    f.render_widget(cpdata.clone().block(checkpoint), layout[0]);
    let checkpoints_list = List::new(times)
        .block(Block::default().borders(Borders::ALL).title("Checkpoints"));
    f.render_widget(checkpoints_list, layout[1]);
}