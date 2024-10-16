use std::{collections::VecDeque, path::Path, time::{Duration, Instant}};

use beetlerank::BeetleRank;
use checkpoint::Checkpoint;
use course::Course;
use csv::Reader;
use guild_wars_handler::GW2Data;
use util::{euclidian_distance_2d, euclidian_distance_3d};

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

pub struct RaceContext {
    pub selected_cup: Option<String>,
    pub selected_course: Option<Course>,
    pub current_checkpoint: usize,
    pub start_time: Instant,
    pub checkpoint_times: Vec<Duration>,
    pub race_state: RaceState,

    instants: (TimePosition, TimePosition),
    distance_queue: VecDeque<f32>,
    gw2_data: GW2Data,
}

impl RaceContext {

    // ----- PUBLIC METHODS -----

    pub fn new(data: GW2Data) -> RaceContext {
        RaceContext {
            selected_cup: None,
            selected_course: None,
            current_checkpoint: 0usize,
            start_time: Instant::now(),
            checkpoint_times: Vec::new(),
            race_state: RaceState::WaitingToStart,
            instants: (TimePosition::new(), TimePosition::new()),
            distance_queue: VecDeque::from(vec![0f32, 0f32]),
            gw2_data: data,
        }
    }

    pub fn load_course(&mut self, track: &String) -> Result<()> {
        std::fs::create_dir_all("data/courses")?;
        let filepath = format!("data/courses/{}.csv", track);
        if Path::new(&filepath).is_file() {
            self.selected_course = Some(Course::from_reader(track, &mut csv::Reader::from_path(&filepath)?)?);
            return Ok(())
        }
        let data = BeetleRank::get_checkpoints(&track)?;
        let course = Course::from_reader(track, &mut csv::Reader::from_reader(data.as_bytes()))?;
        course.export(filepath)?;
        self.selected_course = Some(course);
        Ok(())
    }

    pub fn racer_name(&self) -> &String {
        &self.gw2_data.racer.name
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
        self.selected_course.as_ref().unwrap().checkpoints[self.current_checkpoint]
    }

    pub fn update_state(&mut self) {
        self.race_state = match self.current_checkpoint {
            0 => RaceState::WaitingToStart,
            cp if cp < self.selected_course.as_ref().unwrap().checkpoints.len() => RaceState::Racing ,
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
        if self.current_checkpoint < self.selected_course.as_ref().unwrap().checkpoints.len() {
            if self.current_cp_distance() < self.peek_current_checkpoint().radius as f32 {
                return true
            }
        }
        
        false
    }

    pub fn is_in_reset_checkpoint(&self) -> bool {
        if let (Some(dst), Some(cp)) = (self.reset_cp_distance(), self.selected_course.as_ref().unwrap().reset) {
            if dst < cp.radius as f32 {
                return true
            }
        }

        false
    }

    pub fn current_cp_distance(&self) -> f32 {
        let checkpoint = &self.selected_course.as_ref().unwrap().checkpoints[self.current_checkpoint];
        euclidian_distance_3d(&self.gw2_data.racer.position, &checkpoint.point())
    }

    pub fn reset_cp_distance(&self) -> Option<f32> {
        if let Some(reset) = &self.selected_course.as_ref().unwrap().reset {
            return Some(euclidian_distance_3d(&self.gw2_data.racer.position, &reset.point()))
        }

        None
    }

    pub fn update(&mut self) -> Result<()> {
        self.gw2_data.update()?;
        self.instants.0 = self.instants.1;
        self.instants.1 = TimePosition {
            time: Instant::now(),
            position: self.gw2_data.racer.position,
        };
        self.distance_queue.push_back(self.dist_per_poll());
        if self.distance_queue.len() > 5 {
            self.distance_queue.pop_front();
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
        // let duration = self.filtered_time();
        let duration = 10f32;
        let distance = self.filtered_distance();
        // (distance * 866.18 / (duration as f32)) as i32
        // (distance * 100000f32 / (18288f32 / 99f32)  / (duration as f32)) as i32
        // (distance * 9866f32 / (duration as f32)) as i32
        (distance * 10000f32 * (2940f32/2987f32) * (99f32 / 1800f32) / duration) as i32
        // ((distance * 9866f32 * 99f32 / 1800f32) / (duration)) as i32
        // (distance * 100000f32 / (18000f32 / 99f32)  / (duration as f32)) as i32
        // 298 * x = 294
        // x = 294/298
        // (distance * 546.8 / duration as f32) as i32
    }

    // ----- PRIVATE METHODS -----

    fn filtered_distance(&self) -> f32 {
        *self.distance_queue.iter().max_by(|&a, &b| a.partial_cmp(b).unwrap()).unwrap()
    }

    fn dist_per_poll(&self) -> f32 {
        euclidian_distance_2d(&self.instants.0.position, &self.instants.1.position)
    }

    fn time_per_poll(&self) -> u128 {
        self.instants.1.time.duration_since(self.instants.0.time).as_millis()
    }

    fn record_checkpoint_time(&mut self) {
        self.checkpoint_times.push(self.start_time.elapsed())
    }

    fn clear_checkpoint_times(&mut self) {
        self.checkpoint_times = Vec::new();
    }
}