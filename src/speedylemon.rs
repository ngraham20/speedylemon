use anyhow::{Result, Context};
use itertools::Itertools;
use crate::{beetlerank::BeetleRank, track_selector::{TrackSelector, TrackSelectorState}};
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use feotui::{restore_terminal, StatefulScrollingList, Window};
use crate::speedometer::{checkpoint::Checkpoint, course::Course, guild_wars_handler::{self, GW2Data}, racelog::RaceLogEntry, splits::update_track_data, util::{euclidian_distance_3d, Exportable}, LemonContext, RaceState};
use std::{collections::VecDeque, fmt::Display, time::{Duration, Instant}};
use feotui::Popup;
use crate::{basictui::blit, track_selector, DEBUG};

#[derive(PartialEq, Clone, Copy)]
pub enum ProgramState {
    Quit,
    TrackSelector,
    Speedometer,
}

impl Display for ProgramState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::TrackSelector => "Track Selector",
            Self::Speedometer => "Speedometer",
            Self::Quit => "Quit",
        })
    }
}

pub fn run_program() -> Result<()> {
    // TODO: this should be a full state machine to run the contents of the window

    let mut state = ProgramState::TrackSelector;
    let tick_rate = Duration::from_millis(10);
    let mut last_tick = Instant::now();
    let mut beetlerank = BeetleRank::new();
    let mut beetlestatelist = StatefulScrollingList::with_items(beetlerank.get_cups()?.clone()).with_scroll_style(feotui::ScrollStyle::Paging).with_viewport_length(10);
    beetlestatelist.select(0);
    let mockbackground = r"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed ut sem quis ex iaculis ullamcorper. Sed hendrerit placerat odio, eu ultricies ante vestibulum eget. Curabitur vehicula sodales felis, at scelerisque nunc consectetur nec. In hac habitasse platea dictumst. Vivamus consectetur porttitor hendrerit. Morbi vehicula lacinia rhoncus. Maecenas tempus orci vitae urna tristique molestie. Fusce condimentum mi sed vulputate posuere.

Suspendisse quis velit eu felis bibendum imperdiet. Donec nisi purus, suscipit ac diam quis, accumsan lobortis enim. Phasellus vulputate enim dui, ut consectetur lacus blandit et. Curabitur congue, nunc sit amet lacinia sodales, mi mauris cursus nulla, a tempor sem neque id neque. Donec eu nisi at ante aliquam facilisis. Quisque non augue a diam commodo vehicula. Morbi condimentum nulla non leo iaculis, vel scelerisque dui congue. Fusce tincidunt neque sed tellus vestibulum facilisis. Maecenas vitae interdum sapien. Nunc in velit sapien. Aliquam at auctor dui. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Praesent in dapibus urna. Nam ornare urna eu pulvinar posuere. Pellentesque dapibus felis ac justo aliquet aliquam. Vestibulum feugiat vel augue et porttitor.";

    let mut trackselstate = TrackSelectorState::SelectCup;
    let lorem_ipsum: Window = Window::new().with_lines(textwrap::wrap(mockbackground, 70).iter().map(|s| format!("{: <width$}", s.as_ref().to_string(), width = 70)).collect_vec());

    let mut cup_window: Window = Window::new().with_lines(beetlestatelist.viewport()).with_border(feotui::BorderStyle::Bold).with_padding(1);    
    while state != ProgramState::Quit {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => state = ProgramState::Quit,
                        KeyCode::Char('d') => DEBUG.set(!DEBUG.get()),
                        KeyCode::Char('t') => {state = match state {
                            ProgramState::Speedometer => ProgramState::TrackSelector,
                            ProgramState::TrackSelector => ProgramState::Speedometer,
                            ProgramState::Quit => ProgramState::Quit,
                        }},
                        KeyCode::Up => beetlestatelist.prev(),
                        KeyCode::Down => beetlestatelist.next(),
                        KeyCode::Right => {match trackselstate {
                            TrackSelectorState::SelectCup => {
                                beetlestatelist.items = beetlerank.get_tracks(beetlestatelist.selected().unwrap().clone())?;
                                beetlestatelist.select(0);
                                trackselstate = TrackSelectorState::SelectTrack;
                            },
                            _ => {},
                        }},
                        KeyCode::Left => { match trackselstate {
                            TrackSelectorState::SelectTrack => {
                                beetlestatelist.items = beetlerank.get_cups()?.clone();
                                beetlestatelist.select(0);
                                trackselstate = TrackSelectorState::SelectCup;
                            },
                            _ => {},
                        }}
                        _ => {},
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            print!("{esc}[2J{esc}[1;1H", esc=27 as char);
            println!("Program State: {}", state);
            println!("Debug mode: {}", DEBUG.get());
            println!("---");
            cup_window.lines = beetlestatelist.viewport();
            match state {
                ProgramState::Speedometer => {
                    println!("{}", lorem_ipsum.build_lines().join("\n"));
                },
                ProgramState::TrackSelector => println!("{}", lorem_ipsum.lines.popup(&cup_window.build_lines(), 5, 5)),
                _ => {},
            }
            last_tick = Instant::now();
        }
    }
    Ok(())
}

pub fn run() -> Result<()> {
    feotui::init_terminal()?;
    let mut ctx = LemonContext::new(guild_wars_handler::GW2Data::new()?);
    ctx.course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    ctx.init_gw2_data()?;
    let tick_rate = Duration::from_millis(10);
    let log_delta = Duration::from_millis(30);

    let mut state = ProgramState::Speedometer;
    let mut last_tick = Instant::now();
    let mut last_log = Instant::now();
    let mut race_log: Vec<RaceLogEntry> = Vec::new();
    let mut old_racestate: RaceState;
    while state != ProgramState::Quit {
        ctx.update().context(format!("Failed to update SpeedyLemon Context Object"))?;

        // restart course if needed
        if ctx.is_in_reset_checkpoint() {
            ctx.restart_course();
        }
        
        // collect checkpoint if needed
        if ctx.is_in_current_checkpoint() {
            ctx.collect_checkpoint();
        }

        old_racestate = ctx.race_state;
        ctx.update_state();

        // trigger events if the state has changed
        if ctx.race_state != old_racestate {
            match ctx.race_state {
                RaceState::Finished => {
                    // TODO: double check if the log has the final timestamp. If it doesn't, make sure to append it before exporting.
                    race_log.export(String::from(format!("./dev/{}-racelog.csv", ctx.course.name))).context("Failed to export race log")?;
                    update_track_data(&ctx.checkpoint_times, String::from(format!("./dev/{}-splits.toml", ctx.course.name))).context("Failed to export splits")?;
                },
                _ => {},
            }
        }

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('p') => state = ProgramState::Quit,
                        KeyCode::Char('-') => {ctx.restart_course()},
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            blit(&mut ctx);
            last_tick = Instant::now();
        }
        if last_log.elapsed() >= log_delta {
            last_log = Instant::now();
            race_log.push(RaceLogEntry {
                x: ctx.x(),
                y: ctx.y(),
                z: ctx.z(),
                speed: ctx.filtered_speed() as f32,
                cam_angle: 0.0,
                beetle_angle: 0.0,
                timestamp: ctx.start_time.elapsed().as_millis() as f64,
                acceleration: 0.0,
                map_angle: 0.0,
            });
        }
    }

    restore_terminal()?;
    Ok(())
}