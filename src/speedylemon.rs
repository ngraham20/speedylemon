use anyhow::{Result, Context};
use crate::beetlerank::BeetleRank;
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use feotui::{restore_terminal, StatefulList};
use crate::speedometer::{checkpoint::Checkpoint, course::Course, guild_wars_handler::{self, GW2Data}, racelog::RaceLogEntry, splits::update_track_data, util::{euclidian_distance_3d, Exportable}, LemonContext, RaceState};
use std::{collections::VecDeque, time::{Duration, Instant}};

use crate::{basictui::blit, track_selector, DEBUG, TRACK_SELECT};

#[derive(PartialEq, Clone, Copy)]
pub enum ProgramState {
    Quit,
    Continue,
}

pub fn run_program() -> Result<()> {
    // TODO: this should be a full state machine to run the contents of the window

    let mut state = ProgramState::Continue;
    let tick_rate = Duration::from_millis(10);
    let mut last_tick = Instant::now();
    let mut beetlerank = BeetleRank::new();

    // track_selector should stay in memory to maintain the cache
    let mut track_selector = track_selector::TrackSelector{
        state: track_selector::TrackSelectorState::Unselected,
        cups: StatefulList::with_items(beetlerank.get_cups()?.clone()),
        tracks: StatefulList::with_items(vec![]),
    };

    while state != ProgramState::Quit {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => state = ProgramState::Quit,
                        KeyCode::Char('d') => DEBUG.set(!DEBUG.get()),
                        KeyCode::Char('t') => TRACK_SELECT.set(!TRACK_SELECT.get()),
                        _ => {},
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            print!("{esc}[2J{esc}[1;1H", esc=27 as char);
            println!("Track Select: {}", TRACK_SELECT.get());
            last_tick = Instant::now();
        }
    }
    Ok(())
}


// TODO: instead of a separate function, break this into self-contained modules
// the input should be passed to the module based on state
// the UI should be drawn based on state 
pub fn run_track_selector() -> Result<()> {
    let mut state = ProgramState::Continue;
    let tick_rate = Duration::from_millis(10);
    let mut last_tick = Instant::now();
    // let mut dummydata: HashMap<String, Vec<String>> = HashMap::new();
    let mut beetlerank = BeetleRank::new();
    // dummydata.insert("Cup 1".to_string(), vec!["Seitung Circuit".to_string(), "Brisban Wildlands".to_string()]);
    // dummydata.insert("Cup 2".to_string(), vec!["New Keineng Rooftops".to_string(), "Echovald Wilds Swamprace".to_string()]);
    let mut track_selector = track_selector::TrackSelector{
        state: track_selector::TrackSelectorState::Unselected,
        cups: StatefulList::with_items(beetlerank.get_cups()?.clone()),
        tracks: StatefulList::with_items(vec![]),
    };
    while state != ProgramState::Quit {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('p') => state = ProgramState::Quit,
                        KeyCode::Char('d') => DEBUG.set(!DEBUG.get()),
                        KeyCode::Up => {
                            match track_selector.state {
                                track_selector::TrackSelectorState::Unselected => {
                                    track_selector.state = track_selector::TrackSelectorState::SelectCup;
                                    track_selector.cups.select(0);
                                },
                                track_selector::TrackSelectorState::SelectCup => {
                                    track_selector.cups.prev();
                                },
                                track_selector::TrackSelectorState::SelectTrack => {
                                    track_selector.tracks.prev()
                                },
                            }
                        },
                        KeyCode::Down => {
                            match track_selector.state {
                                track_selector::TrackSelectorState::Unselected => {
                                    track_selector.state = track_selector::TrackSelectorState::SelectCup;
                                    track_selector.cups.select(0);
                                },
                                track_selector::TrackSelectorState::SelectCup => {
                                    track_selector.cups.next();
                                },
                                track_selector::TrackSelectorState::SelectTrack => {
                                    track_selector.tracks.next()
                                },
                            }
                        },
                        KeyCode::Left => {
                            match track_selector.state {
                                track_selector::TrackSelectorState::Unselected => {
                                    track_selector.state = track_selector::TrackSelectorState::SelectCup;
                                    track_selector.cups.select(0);
                                },
                                track_selector::TrackSelectorState::SelectCup => {
                                    track_selector.cups.select(0);
                                },
                                track_selector::TrackSelectorState::SelectTrack => {
                                    track_selector.tracks.clear();
                                    track_selector.state = track_selector::TrackSelectorState::SelectCup;
                                },
                            }
                        },
                        KeyCode::Right => {
                            match track_selector.state {
                                track_selector::TrackSelectorState::Unselected => {
                                    track_selector.state = track_selector::TrackSelectorState::SelectCup;
                                    track_selector.cups.select(0);
                                },
                                track_selector::TrackSelectorState::SelectCup => {
                                    track_selector.state = track_selector::TrackSelectorState::SelectTrack;
                                    track_selector.tracks = StatefulList::with_items(beetlerank.get_tracks(track_selector.cups.selected().unwrap().clone()).unwrap().clone());
                                    track_selector.tracks.select(0);
                                },
                                track_selector::TrackSelectorState::SelectTrack => {/* do nothing */},
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    
        if last_tick.elapsed() >= tick_rate {
            print!("{esc}[2J{esc}[1;1H", esc=27 as char);
            println!("{}", track_selector.build_pane());
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

    let mut state = ProgramState::Continue;
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