use anyhow::{Result, Context};
use beetlerank::BeetleRank;
use itertools::Itertools;
use crate::{speedometer::{checkpoint::Stepname, util::{Importable, Timestamp}}, track_selector::TrackSelectorState};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use feotui::{Border, Padding, Render, StatefulScrollingList};
use crate::speedometer::{splits::*, checkpoint::Checkpoint, course::Course, guild_wars_handler::{self}, racelog::RaceLogEntry, splits::update_track_data, util::Exportable, RaceContext, RaceState};
use std::{fmt::Display, fs, path::Path, time::{Duration, Instant}};
use feotui::Popup;
use crate::DEBUG;
use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq, Clone, Copy)]
pub enum ProgramState {
    Quit,
    TrackSelector,
    TrackCreator,
    Speedometer,
}

impl Display for ProgramState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::TrackSelector => "Track Selector",
            Self::TrackCreator => "Track Creator",
            Self::Speedometer => "Speedometer",
            Self::Quit => "Quit",
        })
    }
}

pub fn run() -> Result<()> {
    feotui::init_terminal()?;
    let mut ctx = RaceContext::new(guild_wars_handler::GW2Data::new()?);
    // ctx.course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    ctx.init_gw2_data()?;
    let tick_rate = Duration::from_millis(10);
    let log_delta = Duration::from_millis(30);

    let mut state = ProgramState::TrackSelector;
    let mut last_tick = Instant::now();
    let mut last_log = Instant::now();
    let mut race_log: Vec<RaceLogEntry> = Vec::new();
    let mut old_racestate: RaceState;
    let mut creating_course: Course = Course::new();
    let mut upload_response: Vec<String> = Vec::new();

    let mut trackselstate = TrackSelectorState::SelectCup;

    let mut beetlerank = BeetleRank::new();
    let mut beetlestatelist = StatefulScrollingList::with_items(beetlerank.get_cups()?.clone()).with_scroll_style(feotui::ScrollStyle::Paging).with_viewport_length(10);
    beetlestatelist.select(0);
    let mut cup_window: Vec<String>;
    let mut pb: Option<RaceLap> = None;

    while state != ProgramState::Quit {
        ctx.update().context(format!("Failed to update SpeedyLemon Context Object"))?;

        if last_log.elapsed() >= log_delta && ctx.race_state == RaceState::Racing {
            last_log = Instant::now();
            race_log.push(RaceLogEntry {
                x: ctx.x(),
                y: ctx.y(),
                z: ctx.z(),
                speed: ctx.filtered_speed() as f32,
                cam_angle: 0.0,
                beetle_angle: 0.0,
                timestamp: ctx.start_time.elapsed().as_millis() as f64 / 1000f64,
                acceleration: 0.0,
                map_angle: 0.0,
            });
        }

        if let Some(_) = &ctx.selected_course {
            // restart course if needed
            if ctx.is_in_reset_checkpoint() {
                ctx.restart_course();
                race_log = Vec::new();
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
                        let track = &ctx.selected_course.clone().unwrap().name;
                        let latest_laptime = ctx.checkpoint_times.last().unwrap().as_millis() as u64;
                        let logfilepath = format!("./data/logs/{}_{}.csv", track, latest_laptime);
                        race_log.export(String::from(&logfilepath)).context("Failed to export race log")?;
                        let racelap = update_track_data(&ctx.checkpoint_times, String::from(format!("./data/splits/{}.toml", track))).context("Failed to export splits")?;
                        pb = Some(racelap.clone());
                        if *ctx.selected_cup.as_ref().unwrap() != "CUSTOM TRACKS".to_string() {
                            upload_response = beetlerank.post_log(ctx.racer_name().clone(), track.clone(), logfilepath)?;
                        }
                    },
                    _ => {},
                }
            }
        }

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => state = ProgramState::Quit,
                        KeyCode::Char('r') => { match state {
                            ProgramState::Speedometer => {
                                ctx.restart_course();
                                race_log = Vec::new();
                            },
                            ProgramState::TrackCreator => { 
                                creating_course.checkpoints = Vec::new();
                                creating_course.reset = None;
                            },
                            _ => {},
                            }
                        },
                        KeyCode::Char('R') => match state {
                            ProgramState::TrackCreator => {
                                creating_course.reset = Some(Checkpoint{
                                    step: -1,
                                    stepname: Stepname::Reset,
                                    x: ctx.x(),
                                    y: ctx.y(),
                                    z: ctx.z(),
                                    radius: 15i32,
                                })
                            },
                            _ => {},
                        }
                        KeyCode::Char('d') => DEBUG.set(!DEBUG.get()),
                        KeyCode::Char('c') => { state = match state {
                            ProgramState::Speedometer => ProgramState::TrackCreator,
                            ProgramState::TrackCreator => ProgramState::Speedometer,
                            _ => state
                        }},
                        KeyCode::Char('n') => { match state {
                            ProgramState::TrackCreator => {
                                creating_course.push_cp(ctx.x(), ctx.y(), ctx.z(), 15i32);
                            },
                            _ => {},
                        }},
                        KeyCode::Char('e') => { match state {
                            ProgramState::TrackCreator => {
                                std::fs::create_dir_all("data/custom_courses")?;
                                creating_course.export(format!("data/custom_courses/created-course.csv"))?;
                            },
                            _ => {},
                        }},
                        KeyCode::Char('t') => {state = match state {
                            ProgramState::Speedometer => ProgramState::TrackSelector,
                            ProgramState::TrackSelector => ProgramState::Speedometer,
                            _ => state
                        }},
                        KeyCode::Up => beetlestatelist.prev(),
                        KeyCode::Down => beetlestatelist.next(),
                        KeyCode::Right => {
                            let selected: String = beetlestatelist.selected().unwrap().clone();
                            match trackselstate {
                            TrackSelectorState::SelectCup => {
                                ctx.selected_cup = Some(selected.clone());
                                if selected == "CUSTOM TRACKS".to_string() {
                                    fs::create_dir_all(Path::new("data/courses/custom_courses")).context("Failed to create custom_courses directory")?;
                                    fs::create_dir_all(Path::new("data/splits/custom_courses")).context("Failed to create custom_courses directory")?;

                                    let paths = fs::read_dir("data/courses/custom_courses").unwrap();
                                    beetlestatelist.items = paths.into_iter().map(|p| {
                                        let path = p.unwrap().path();
                                        let mut components = path.components();
                                        components.next();
                                        components.next();
                                        components.as_path().with_extension("").to_string_lossy().to_string()
                                    }).collect_vec();
                                } else { beetlestatelist.items = beetlerank.get_tracks(&selected)?; }
                                // BUG: if custom tracks is empty, selecting 0 crashes
                                beetlestatelist.select(0);
                                trackselstate = TrackSelectorState::SelectTrack;
                            },
                            TrackSelectorState::SelectTrack => {
                                ctx.load_course(&selected)?;
                                std::fs::create_dir_all("data/splits")?;
                                pb = RaceLap::import(&format!("data/splits/{}.toml", selected))?;                         
                                state = ProgramState::Speedometer;
                            }
                            _ => {},
                        }},
                        KeyCode::Left => { match trackselstate {
                            TrackSelectorState::SelectTrack => {
                                beetlestatelist.items = beetlerank.get_cups()?.clone();
                                beetlestatelist.select(0);
                                trackselstate = TrackSelectorState::SelectCup;
                            },
                            _ => {},
                        }},
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            print!("{esc}[2J{esc}[1;1H", esc=27 as char);
            if DEBUG.get() {
                println!("Program State: {}", state);
                println!("Debug mode: {}", DEBUG.get());
                println!("Tick rate: {}", last_tick.elapsed().as_millis());
                println!("Racer: {}", &ctx.racer_name());
                println!("---");
            }
            cup_window = beetlestatelist.viewport().pad(1).border(feotui::BorderStyle::Bold);
            if let None = &ctx.selected_course {
                println!("{}", cup_window.pad(1).border(feotui::BorderStyle::Bold).render());
            } else {
                let primary_window = speedometer(&mut ctx, &mut beetlerank, &pb, state)?.pad(1).border(feotui::BorderStyle::Bold);
                println!("{}", match state {
                    ProgramState::Speedometer => {
                        match ctx.race_state {
                            RaceState::Finished => primary_window.popup(&race_finished(&ctx, &mut beetlerank, &pb, &upload_response)?.pad(1).border(feotui::BorderStyle::Bold), 2, 2).render(),
                            _ => primary_window.render()
                        }
                    },
                    ProgramState::TrackCreator => primary_window.popup(&track_creator(&creating_course).pad(1).border(feotui::BorderStyle::Bold), 2, 2).render(),
                    ProgramState::TrackSelector => primary_window.popup(&cup_window, 2, 2).render(),
                    _ => {String::new()},
                });
            }
            
            last_tick = Instant::now();
        }
        
    }

    feotui::restore_terminal()?;
    Ok(())
}

fn race_finished(ctx: &RaceContext, beetlerank: &mut BeetleRank, pb: &Option<RaceLap>, upload_response: &Vec<String>) -> Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    let track = &ctx.selected_course.as_ref().unwrap().name;
    lines.push("Race Finished!".to_string());
    let laptime = ctx.checkpoint_times.last().unwrap();
    if ctx.selected_cup != Some("CUSTOM TRACKS".to_string()) {
        if let Some(you) = &beetlerank.rankings[track].you {
            let best_time = (you[1].laptime * 1000f64) as u64;
            lines.push(format!("Beetlerank Best Time: {}", Duration::from_millis(best_time).timestamp()));
        }
    }
    if let Some(rl) = pb {
        lines.push(format!("Local Best Time: {}", Duration::from_millis(rl.pb_laptime).timestamp()));
    }
    
    lines.push(format!("Lap Time: {}", laptime.timestamp()));
    if *upload_response != Vec::<String>::new() {
        lines.append(&mut upload_response.clone());
    }
    Ok(lines)
}

fn track_creator(course: &Course) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    lines.push("Track Creator".to_string());
    lines.push("-------------".to_string());
    for cp in &course.checkpoints {
        lines.push(format!("CP {}: ({:.2}, {:.2}, {:.2}", cp.step, cp.x, cp.y, cp.z));
    }
    if let Some(reset) = &course.reset {
        lines.push(format!("Reset: ({:.2}, {:.2}, {:.2}", reset.x, reset.y, reset.z));
    }
    lines
}

fn rank(ctx: &mut RaceContext, beetlerank: &mut BeetleRank) -> Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    let ranks = beetlerank.get_rank(&ctx.selected_course.as_ref().unwrap().name, &ctx.racer_name())?;

    let top_ranks = &ranks.top_3;
    let top_timestamp_padding: usize = top_ranks.iter().map(|r| r.name.graphemes(true).count()).max().unwrap();

    if let Some(you) = &ranks.you {
        let local_timestamp_padding: usize = you.iter().map(|r| r.name.graphemes(true).count()).max().unwrap();
        let padding = usize::max(local_timestamp_padding, top_timestamp_padding);

        for rank in top_ranks {
            lines.push(format!("{: >2}: {: <padding$} {}", rank.rank, rank.name, rank.timestamp));
        }
        lines.push(format!("..."));
        for rank in you{
            lines.push(format!("{: >2}: {: <padding$} {}", rank.rank, rank.name, rank.timestamp));
        }

    } else {
        for rank in top_ranks{
            lines.push(format!("{: >2}: {: <top_timestamp_padding$} {}", rank.rank, rank.name, rank.timestamp));
        }
    }
    Ok(lines)
}

fn speedometer(ctx: &mut RaceContext, beetlerank: &mut BeetleRank, pb: &Option<RaceLap>, state: ProgramState) -> Result<Vec<String>> {
    let mut lines: Vec<String> = Vec::new();
    lines.push(format!("Track: {}", ctx.selected_course.as_ref().unwrap().name));
    
    if state == ProgramState::Speedometer && ctx.selected_cup != Some("CUSTOM TRACKS".to_string()) {
        lines.append(&mut rank(ctx, beetlerank)?);
    }
    
    lines.push(format!("---"));
    lines.push(format!("Checkpoint: {}", ctx.current_checkpoint));
    lines.push(format!("Distance to next checkpoint: {:.4}", if ctx.current_checkpoint < ctx.selected_course.as_ref().unwrap().checkpoints.len() {
        ctx.current_cp_distance()} else {
            -1.0
        }));
    lines.push(format!("Distance to reset checkpoint: {:.4}", ctx.reset_cp_distance().unwrap_or(-1.0)));
    lines.push(format!("Speed: {:?}", ctx.filtered_speed()));
    if let Some(c) = &ctx.selected_course {
        lines.push("----- Checkpoint Times -----".to_string());
        for idx in 1..c.checkpoints.len() {
            let blank = Duration::new(0,0);
            let dur = ctx.checkpoint_times.get(idx).unwrap_or(&blank);
            let cpdelta = dur.saturating_sub(*ctx.checkpoint_times.get(idx.saturating_sub(1)).unwrap_or(&Duration::new(0,0)));
            let mut delta: String = String::new();
            if let Some(lap) = pb {
                // BUG: since the pb is updated immediately, then reloaded immediately, the delta will suddenly be 00:00:000 when finishing a lap with a new best time
                if let Some(split) = lap.splits.pb.get(idx.saturating_sub(1)) {
                    if cpdelta == blank {
                        delta = "".to_string();
                    }
                    else if *split > cpdelta.as_millis() as u64 {
                        delta = format!("-{}", Duration::from_millis(split.saturating_sub(cpdelta.as_millis() as u64)).timestamp())
                    } else {
                        delta = format!("+{}", Duration::from_millis((cpdelta.as_millis() as u64).saturating_sub(*split)).timestamp())
                    }
                }
            }
        
            lines.push(format!("Checkpoint: {: >2}, Time: {: <9}, PB: {: <9}", idx, if *dur > blank { dur.timestamp() } else { "".to_string() }, delta));
        }
    }
    Ok(lines)
    
}