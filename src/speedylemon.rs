use anyhow::{Result, Context, anyhow};
use crossterm::event::{self, Event, KeyEventKind, KeyCode};
use crate::{util::euclidian_distance, checkpoint::Checkpoint, guild_wars_handler::GW2Data, lemontui, racelog::{RaceLogEntry, RaceLog}};

use std::{time::{Duration, Instant}, collections::VecDeque, fs::{create_dir_all, File}, path::Path, io::Write};

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

    pub fn save_splits(&self, path: String) -> Result<()> {
        log::info!("Exporting checkpoint splits to {}", path);
        create_dir_all(Path::new(&path).parent().unwrap()).context("Failed to create splits directory")?;
        
        let mut writer = csv::Writer::from_writer(vec![]);
        for (idx, split) in self.checkpoint_times.iter().enumerate() {
            match idx {
                0 => {},
                _ => writer.serialize(split.saturating_sub(self.checkpoint_times[idx-1]).as_millis())?
            };
        }
        let mut file = File::create(path).context("Failed to create splits file")?;
        file.write_all(&writer.into_inner()?)?;
        Ok(())
    }

    fn load_splits(path: String) -> Result<Vec<u128>> {
        log::info!("Importing checkpoint splits from {}", path);
        let mut reader = csv::Reader::from_path(&path)?;
        let iter = reader.deserialize();
        let mut splits: Vec<u128> = Vec::new();
        for record in iter {
            let split: u128 = record?;
            splits.push(split);
        }
        Ok(splits)
    }

    pub fn save_best_splits(&self, path: String) -> Result<()> {
        let old_best = Self::load_splits(path)?;
        let splits: Vec<u128> = self.checkpoint_times[1..].iter().enumerate()
            .map(|(idx, split)| split.saturating_sub(self.checkpoint_times[idx-1]).as_millis()).collect();

        if old_best.len() != splits.len() {
            return Err(anyhow!("Split lengths are not equal"))
        }

        let zipped = old_best.iter().zip(splits.iter());
        let final_splits: Vec<u128> = zipped.map(|(old, new)| std::cmp::max(*old, *new)).collect();
        Ok(())
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
        euclidian_distance(&self.gw2_data.racer.position, &checkpoint.point())
    }

    pub fn reset_cp_distance(&self) -> Option<f32> {
        if let Some(reset) = &self.course.reset {
            return Some(euclidian_distance(&self.gw2_data.racer.position, &reset.point()))
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
    pub fn filtered_velocity(&self) -> i32 {
        let duration = self.filtered_time();
        let distance = self.filtered_distance();
        (distance * 866.18 / (duration as f32)) as i32
    }

    // ----- PRIVATE METHODS -----

    fn filtered_distance(&self) -> f32 {
        *self.distance_queue.iter().max_by(|&a, &b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn filtered_time(&self) -> u128 {
        *self.time_queue.iter().max().unwrap()
    }

    fn dist_per_poll(&self) -> f32 {
        euclidian_distance(&self.events.0.position, &self.events.1.position)
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

pub fn run() -> Result<()> {
    let mut terminal = lemontui::init_terminal()?;
    let mut ctx = LemonContext::new(guild_wars_handler::GW2Data::new()?);
    ctx.course = Course::from_path(String::from("maps/TYRIACUP/TYRIA GENDARRAN.csv"))?;
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
                    race_log.export(String::from(format!("./dev/{}-racelog.csv", ctx.course.name))).context("Failed to export race log")?;
                    ctx.save_splits(String::from(format!("./dev/{}-splits.csv", ctx.course.name))).context("Failed to export splits")?;
                },
                _ => {},
            }
        }

        terminal.draw(|f| {lemontui::ui(f, &mut ctx)})?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('p') => state = ProgramState::Quit,
                        _ => {}
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
        if last_log.elapsed() >= log_delta {
            last_log = Instant::now();
            race_log.push(RaceLogEntry {
                x: ctx.x(),
                y: ctx.y(),
                z: ctx.z(),
                speed: ctx.filtered_velocity() as f32,
                cam_angle: 0.0,
                beetle_angle: 0.0,
                timestamp: ctx.start_time.elapsed().as_millis() as f64,
                acceleration: 0.0,
                map_angle: 0.0,
            });
        }
    }

    lemontui::restore_terminal()?;
    Ok(())
}