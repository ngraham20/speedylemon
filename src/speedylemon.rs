use anyhow::{Result, Context};
use crate::{course::CourseState, util::euclidian_distance};

use super::guild_wars_handler;
use super::course::Course;
use log;
use device_query::{DeviceQuery, DeviceState, Keycode};
use enigo::*;

#[derive(PartialEq)]
pub enum ProgramState {
    Quit,
    Continue,
    RestartCourse,
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

    // load checkpoint file from command argument or tui
    // get live mumble api data
    // as a base level functionality, output to the console when the next checkpoint is reached

    // main-loop
    // read the mumble data
    // update the racer position
    // check if the position is inside the next checkpoint


    // state can be maintained with multiple structs, each of which simply has `.update()` called on it.
    // the struct's internal state can be handled from within.
    // this state may be exported if sending messages between structs is necessary.

    let mut data = guild_wars_handler::GW2Data::new()?;
    data.init()?;
    data.update().context(format!("Failed to update GW2 Data"))?;
    log::debug!("Name: {}, Racer Position: {:?}, Camera Position: {:?}", &data.racer.name, &data.racer.position, &data.camera.position);
    let mut course = Course::from_path(String::from("maps/TYRIACUP/TYRIA DIESSA PLATEAU.csv"))?;
    course.export_to_path(String::from("maps/RAEVENCUP/01-development.csv"))?;

    let mut state = ProgramState::Continue;
    let mut course_state = CourseState::WaitingToStart;
    let mut enigo = Enigo::new();
    while state != ProgramState::Quit {
        data.update().context(format!("Failed to update GW2 Data"))?;
        let next_checkpoint = course.peek_next();
        let dst = super::util::euclidian_distance(data.racer.position, next_checkpoint.point());
        if let Some(reset) = &course.reset {
            let restart_dst = euclidian_distance(data.racer.position, reset.point());
            if restart_dst < reset.radius as f32 {
                course.restart();
                enigo.key_down(Key::K);
                std::thread::sleep(std::time::Duration::from_millis(50));
                enigo.key_up(Key::K);
    
            }
        }
        if dst < next_checkpoint.radius as f32 {
            course_state = course.collect_checkpoint();
            // TODO: make a Livesplit struct with `split` and `reset` keys, then just call a function for split and reset
            enigo.key_down(Key::L);
            std::thread::sleep(std::time::Duration::from_millis(50));
            enigo.key_up(Key::L);
        }

        // TODO: Move course state functionality into a function
        if course_state == CourseState::WaitingToStart {
            log::debug!("Waiting for player to cross starting line")
        } else if course_state == CourseState::ApproachingFinishLine {
            log::debug!("Distance to finish line: {}", dst);
        } else if course_state == CourseState::Racing {
            log::debug!("Distance to checkpoint {}: {}", course.current_checkpoint, dst);
        } else if course_state == CourseState::Finished {
            log::debug!("Race Finished!");
            break;
        }
        state = global_input()?;
    }

    log::info!("Terminating program.");
    std::thread::sleep(std::time::Duration::from_secs(1));
    Ok(())
}
