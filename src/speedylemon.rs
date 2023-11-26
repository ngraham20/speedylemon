use anyhow::{Result, Context};
use crossterm::event::{Event, self, KeyEventKind, KeyCode};
use crate::{util::euclidian_distance, checkpoint::Checkpoint, guild_wars_handler::GW2Data, lemontui};

use std::time::{Duration, Instant};
use crossbeam_channel::{unbounded, bounded, Receiver, select, tick};

use crate::guild_wars_handler;
use crate::course::Course;
use log;
use device_query::{DeviceQuery, DeviceState, Keycode};

use std::thread;

#[derive(PartialEq, Clone, Copy)]
pub enum ProgramState {
    Quit,
    Continue,
    RestartCourse,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum RaceState {
    WaitingToStart,
    Racing,
    Finished,
}

pub struct LemonContext {
    pub course: Course,
    pub current_checkpoint: usize,
    pub start_time: Instant,
    pub checkpoint_times: Vec<Duration>,
    pub race_state: RaceState,
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

fn ctrl_channel() -> Result<Receiver<ProgramState>> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(ProgramState::Quit);
    })?;

    Ok(receiver)
}

pub fn run() -> Result<()> {
    // let ctrl_c_events = ctrl_channel()?;
    // let ticks = tick(Duration::from_secs(1));

    let mut terminal = lemontui::init_terminal()?;
    let mut ctx = LemonContext::new(guild_wars_handler::GW2Data::new()?);
    ctx.course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    ctx.init_gw2_data()?;
    let tick_rate = Duration::from_millis(5);

    let mut state = ProgramState::Continue;
    let mut last_tick = Instant::now();
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

        ctx.update_state();

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
    }

    lemontui::restore_terminal()?;
    Ok(())
}