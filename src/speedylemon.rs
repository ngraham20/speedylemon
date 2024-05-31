use anyhow::{Result, Context, anyhow};
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use itertools::{ConsTuples, Itertools};
use serde::de::Unexpected;
use crate::{basictui, checkpoint::Checkpoint, guild_wars_handler::GW2Data, racelog::RaceLogEntry, splits, track_selector::{self, StatefulList}, util::{euclidian_distance_3d, Exportable, Importable}, DEBUG};

use std::{collections::{HashMap, VecDeque}, time::{Duration, Instant}};

use crate::guild_wars_handler;
use crate::course::Course;

#[derive(PartialEq, Clone, Copy)]
pub enum ProgramState {
    Quit,
    Continue,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RaceState {
    WaitingToStart,
    Racing,
    Finished,
}

#[derive(PartialEq, Clone, Copy)]
pub struct TimePosition {
    time: Instant,
    position: [f32; 3],
}

impl TimePosition {
    pub fn new() -> TimePosition {
        TimePosition { 
            time: Instant::now(),
            position: [0f32; 3],
        }
    }
}

pub struct LemonContext {
    pub course: Course,
    pub current_checkpoint: usize,
    pub start_time: Instant,
    pub checkpoint_times: Vec<Duration>,
    pub race_state: RaceState,

    events: (TimePosition, TimePosition),
    distance_queue: VecDeque<f32>,
    time_queue: VecDeque<u128>,
    gw2_data: GW2Data,
}

impl LemonContext {

    // ----- PUBLIC METHODS -----

    pub fn new(data: GW2Data) -> LemonContext {
        LemonContext {
            course: Course::new(),
            current_checkpoint: 0usize,
            start_time: Instant::now(),
            checkpoint_times: Vec::new(),
            race_state: RaceState::WaitingToStart,
            events: (TimePosition::new(), TimePosition::new()),
            distance_queue: VecDeque::from(vec![0f32, 0f32]),
            time_queue: VecDeque::from(vec![0u128, 0u128]),
            gw2_data: data,
        }
    }

    pub fn x(&self) -> f32 {
        self.gw2_data.racer.position[0]
    }
    pub fn y(&self) -> f32 {
        self.gw2_data.racer.position[1]
    }
    pub fn z(&self) -> f32 {
        self.gw2_data.racer.position[2]
    }

    pub fn init_gw2_data(&mut self) -> Result<()> {
        self.gw2_data.init()?;
        Ok(())
    }

    pub fn start_timer(&mut self) {
        self.start_time = Instant::now()
    }

    pub fn restart_course(&mut self) {
        self.current_checkpoint = 0;
        self.clear_checkpoint_times();
    }

    pub fn peek_current_checkpoint(&self) -> Checkpoint {
        self.course.checkpoints[self.current_checkpoint]
    }

    fn update_state(&mut self) {
        self.race_state = match self.current_checkpoint {
            0 => RaceState::WaitingToStart,
            cp if cp < self.course.checkpoints.len() => RaceState::Racing ,
            _ => RaceState::Finished,
        }
    }

    pub fn collect_checkpoint(&mut self) {
        if self.race_state == RaceState::Finished {
            return;
        }
        if self.race_state == RaceState::WaitingToStart {
            self.start_timer();
        }
        self.record_checkpoint_time();
        self.current_checkpoint += 1;
    }

    pub fn is_in_current_checkpoint(&self) -> bool {
        if self.current_checkpoint < self.course.checkpoints.len() {
            if self.current_cp_distance() < self.peek_current_checkpoint().radius as f32 {
                return true
            }
        }
        
        false
    }

    pub fn is_in_reset_checkpoint(&self) -> bool {
        if let (Some(dst), Some(cp)) = (self.reset_cp_distance(), self.course.reset) {
            if dst < cp.radius as f32 {
                return true
            }
        }

        false
    }

    pub fn current_cp_distance(&self) -> f32 {
        let checkpoint = &self.course.checkpoints[self.current_checkpoint];
        euclidian_distance_3d(&self.gw2_data.racer.position, &checkpoint.point())
    }

    pub fn reset_cp_distance(&self) -> Option<f32> {
        if let Some(reset) = &self.course.reset {
            return Some(euclidian_distance_3d(&self.gw2_data.racer.position, &reset.point()))
        }

        None
    }

    pub fn update(&mut self) -> Result<()> {
        self.gw2_data.update()?;
        self.events.0 = self.events.1;
        self.events.1 = TimePosition {
            time: Instant::now(),
            position: self.gw2_data.racer.position,
        };
        self.distance_queue.push_back(self.dist_per_poll());
        self.time_queue.push_back(self.time_per_poll());
        if self.distance_queue.len() > 5 {
            self.distance_queue.pop_front();
        }
        if self.time_queue.len() > 10 {
            self.time_queue.pop_front();
        }
        Ok(())
    }

    /// Calculate the velocity based on the filtered distance and time
    /// 
    /// ## Constraining the velocity
    /// When boosting, 11430 is the maximum speed measured when boosting but not drifting
    /// 
    /// Set that speed to 100, so 11430/100 = 114.3
    /// 
    /// 100000 / 114.3 = 874.89 achieves the numbers we want
    /// 
    /// Alternatively, 100000 / 115.45 = 866.18 will make the max speed 137 when drifting, which matches the speedometer
    pub fn filtered_speed(&self) -> i32 {
        let duration = self.filtered_time();
        let distance = self.filtered_distance();
        // (distance * 866.18 / (duration as f32)) as i32
        (distance * 546.8 / duration as f32) as i32
    }

    // ----- PRIVATE METHODS -----

    fn filtered_distance(&self) -> f32 {
        *self.distance_queue.iter().max_by(|&a, &b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn filtered_time(&self) -> u128 {
        *self.time_queue.iter().max().unwrap()
    }

    fn dist_per_poll(&self) -> f32 {
        euclidian_distance_3d(&self.events.0.position, &self.events.1.position)
    }

    fn time_per_poll(&self) -> u128 {
        self.events.1.time.duration_since(self.events.0.time).as_millis()
    }

    fn record_checkpoint_time(&mut self) {
        self.checkpoint_times.push(self.start_time.elapsed())
    }

    fn clear_checkpoint_times(&mut self) {
        self.checkpoint_times = Vec::new();
    }
}

pub fn run_track_selector() -> Result<()> {
    let mut state = ProgramState::Continue;
    let tick_rate = Duration::from_millis(10);
    let mut last_tick = Instant::now();
    let mut dummydata: HashMap<String, Vec<String>> = HashMap::new();
    let cups = vec!["Cup 1".to_string(), "Cup 2".to_string()];
    dummydata.insert("Cup 1".to_string(), vec!["Seitung Circuit".to_string(), "Brisban Wildlands".to_string()]);
    dummydata.insert("Cup 2".to_string(), vec!["New Keineng Rooftops".to_string(), "Echovald Wilds Swamprace".to_string()]);
    let mut track_selector = track_selector::TrackSelector{
        state: track_selector::TrackSelectorState::Unselected,
        cups: StatefulList::with_items(cups),
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
                                    track_selector.tracks = StatefulList::with_items(dummydata.get(track_selector.cups.selected().unwrap()).unwrap().to_vec());
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
    basictui::init_terminal()?;
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
                    splits::update_track_data(&ctx.checkpoint_times, String::from(format!("./dev/{}-splits.toml", ctx.course.name))).context("Failed to export splits")?;
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
            basictui::blit(&mut ctx);
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

    basictui::restore_terminal()?;
    Ok(())
}