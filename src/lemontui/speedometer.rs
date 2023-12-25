use std::time::Duration;

use ratatui::{Frame, text::Line, layout::{Layout, Constraint}, widgets::{Block, Borders, Paragraph, ListItem, List}};

use crate::speedylemon::{LemonContext, RaceState};

trait Timestamp {
    fn timestamp(&self) -> String;
}

impl Timestamp for Duration {
    fn timestamp(&self) -> String {
        format!("{:02}:{:02}:{:03}", self.as_secs()/60, self.as_secs()%60, self.subsec_millis())
    }
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