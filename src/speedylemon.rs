use anyhow::{Result, Context};
use crate::{util::euclidian_distance, checkpoint::Checkpoint, guild_wars_handler::GW2Data, lemontui};

use std::time::{Duration, Instant};

use crate::guild_wars_handler;
use crate::course::Course;
use log;
use device_query::{DeviceQuery, DeviceState, Keycode};

use std::thread;

#[derive(PartialEq)]
pub enum ProgramState {
    Quit,
    Continue,
    RestartCourse,
}

pub struct LemonContext {
    pub course: Course,
    pub current_checkpoint: usize,
    pub start_time: Instant,
    pub checkpoint_times: Vec<Duration>,
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
            gw2_data: data,
        }
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

    pub fn collect_checkpoint(&mut self) {
        if self.current_checkpoint == 0 {
            self.start_time = Instant::now();
        }
        if self.current_checkpoint < self.course.checkpoints.len() {
            // TODO: time for RaceState to be implemented to guard against trying to collect a checkpoint after the race is finished
            self.record_checkpoint_time();
            self.current_checkpoint += 1;
        }
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

    pub fn update_gw2_data(&mut self) -> Result<()> {
        self.gw2_data.update()?;
        Ok(())
    }

    // ----- PRIVATE METHODS -----

    fn record_checkpoint_time(&mut self) {
        self.checkpoint_times.push(self.start_time.elapsed())
    }

    fn clear_checkpoint_times(&mut self) {
        self.checkpoint_times = Vec::new();
    }
}

pub struct State {
    program: ProgramState,
}

pub fn global_input() -> Result<ProgramState> {
    let device_state = DeviceState::new();

    let keys = device_state.get_keys();
    // log::debug!("All keys currently down: {:?}", keys);

    // if P: quit the program
    if keys.contains(&Keycode::P) {
        return Ok(ProgramState::Quit)
    }
    if keys.contains(&Keycode::R) {
        return Ok(ProgramState::RestartCourse)
    }
    
    Ok(ProgramState::Continue)
}

pub fn run() -> Result<()> {
    let mut terminal = lemontui::init_terminal()?;
    let mut ctx = LemonContext::new(guild_wars_handler::GW2Data::new()?);
    ctx.course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    ctx.init_gw2_data()?;

    let state = ProgramState::Continue;

    while state != ProgramState::Quit {
        ctx.update_gw2_data().context(format!("Failed to update GW2 Data"))?;

        // restart course if needed
        if ctx.is_in_reset_checkpoint() {
            ctx.restart_course();
        }
        
        // collect checkpoint if needed
        if ctx.is_in_current_checkpoint() {
            ctx.collect_checkpoint();
        }

        terminal.draw(|f| {lemontui::ui(f, &mut ctx)})?;
        thread::sleep(Duration::from_millis(500));
    }

    lemontui::restore_terminal()?;
    Ok(())
}