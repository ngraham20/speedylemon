use std::time::Duration;
use ratatui::{prelude::*, widgets::*};
use crate::speedylemon::{LemonContext, RaceState, App};

mod tui_utils;
mod track_selector;
mod speedometer;

pub use tui_utils::*;
pub use track_selector::*;
pub use speedometer::*;