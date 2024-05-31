use speedometer::util::Timestamp;

use crate::speedylemon::{LemonContext, RaceState};


pub fn blit(ctx: &mut LemonContext) {
    // clear the screen
    print!("{esc}[2J{esc}[1;1H", esc=27 as char);

    let mut window = String::from(format!("Track: {}\n", ctx.course.name));

    match ctx.race_state {
        RaceState::Finished => {
            window.push_str("Race Finished!\n");
            window.push_str(&format!("Lap Time: {:?}", ctx.checkpoint_times.last().unwrap().timestamp()));
        },
        _ => {
            
            window.push_str(&format!("Checkpoint: {}\n", ctx.current_checkpoint));
            window.push_str(&format!("Distance to next checkpoint: {}\n", if ctx.current_checkpoint < ctx.course.checkpoints.len() {
                ctx.current_cp_distance()} else {
                    -1.0
                }));
            window.push_str(&format!("Distance to reset checkpoint: {}\n", ctx.reset_cp_distance().unwrap_or(-1.0)));
            window.push_str(&format!("Speed: {:?}\n", ctx.filtered_speed()));
            window.push_str("----- Checkpoint Times -----\n");
            for (idx, dur) in ctx.checkpoint_times.iter().enumerate() {
                window.push_str(&format!("Checkpoint: {}, Time: {}, Delta: {}\n", idx, dur.timestamp(), match idx {
                    0 => dur.timestamp(),
                    _ => dur.saturating_sub(ctx.checkpoint_times[idx-1]).timestamp()
                }))
            }
        }
    }
    print!("{}", window);
    
}