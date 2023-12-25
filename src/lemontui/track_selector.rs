use ratatui::{Frame, layout::{Layout, Direction, Constraint}, widgets::{ListItem, List, Borders, Block}, style::{Style, Color, Modifier}};
use crate::lemontui::App;

use super::StatefulList;

pub enum TrackSelectorState {
    Unselected,
    SelectCup,
    SelectTrack,
}
pub struct StatefulTrackSelector {
    pub state: TrackSelectorState,
    pub cups: StatefulList<String>,
    pub tracks: StatefulList<String>,
}

impl StatefulTrackSelector {
    pub fn new() -> StatefulTrackSelector {
        StatefulTrackSelector {
            state: TrackSelectorState::Unselected,
            cups: StatefulList::with_items(vec![]),
            tracks: StatefulList::with_items(vec![]),
        }
    }
}

pub fn map_selection(f: &mut Frame, app: &mut StatefulTrackSelector) {
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