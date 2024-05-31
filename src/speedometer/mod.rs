use std::{collections::VecDeque, time::{Duration, Instant}};

use checkpoint::Checkpoint;
use course::Course;
use guild_wars_handler::GW2Data;
use util::euclidian_distance_3d;

use anyhow::Result;

pub mod camera;
pub mod checkpoint;
pub mod course;
pub mod guild_wars_handler;
pub mod racelog;
pub mod racer;
pub mod splits;
pub mod util;

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

    pub fn update_state(&mut self) {
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