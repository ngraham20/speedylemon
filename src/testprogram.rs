pub fn run_test_program() -> Result<()> {
    // TODO: this should be a full state machine to run the contents of the window

    let mut state = ProgramState::TrackSelector;
    let tick_rate = Duration::from_millis(10);
    let mut last_tick = Instant::now();
    let mut beetlerank = BeetleRank::new();
    let mut beetlestatelist = StatefulScrollingList::with_items(beetlerank.get_cups()?.clone()).with_scroll_style(feotui::ScrollStyle::Paging).with_viewport_length(10);
    beetlestatelist.select(0);
    let mockbackground = r"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed ut sem quis ex iaculis ullamcorper. Sed hendrerit placerat odio, eu ultricies ante vestibulum eget. Curabitur vehicula sodales felis, at scelerisque nunc consectetur nec. In hac habitasse platea dictumst. Vivamus consectetur porttitor hendrerit. Morbi vehicula lacinia rhoncus. Maecenas tempus orci vitae urna tristique molestie. Fusce condimentum mi sed vulputate posuere.

Suspendisse quis velit eu felis bibendum imperdiet. Donec nisi purus, suscipit ac diam quis, accumsan lobortis enim. Phasellus vulputate enim dui, ut consectetur lacus blandit et. Curabitur congue, nunc sit amet lacinia sodales, mi mauris cursus nulla, a tempor sem neque id neque. Donec eu nisi at ante aliquam facilisis. Quisque non augue a diam commodo vehicula. Morbi condimentum nulla non leo iaculis, vel scelerisque dui congue. Fusce tincidunt neque sed tellus vestibulum facilisis. Maecenas vitae interdum sapien. Nunc in velit sapien. Aliquam at auctor dui. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Praesent in dapibus urna. Nam ornare urna eu pulvinar posuere. Pellentesque dapibus felis ac justo aliquet aliquam. Vestibulum feugiat vel augue et porttitor.";

    let mut trackselstate = TrackSelectorState::SelectCup;
    let lorem_ipsum: Vec<String> = textwrap::wrap(mockbackground, 70).iter().map(|s| format!("{: <width$}", s.as_ref().to_string(), width = 70)).collect_vec();

    let mut cup_window: Vec<String>;    
    while state != ProgramState::Quit {
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => state = ProgramState::Quit,
                        KeyCode::Char('d') => DEBUG.set(!DEBUG.get()),
                        KeyCode::Char('t') => {state = match state {
                            ProgramState::Speedometer => ProgramState::TrackSelector,
                            ProgramState::TrackSelector => ProgramState::Speedometer,
                            _ => state
                        }},
                        KeyCode::Up => beetlestatelist.prev(),
                        KeyCode::Down => beetlestatelist.next(),
                        KeyCode::Right => {match trackselstate {
                            TrackSelectorState::SelectCup => {
                                beetlestatelist.items = beetlerank.get_tracks(&beetlestatelist.selected().unwrap().clone())?;
                                beetlestatelist.select(0);
                                trackselstate = TrackSelectorState::SelectTrack;
                            },
                            _ => {},
                        }},
                        KeyCode::Left => { match trackselstate {
                            TrackSelectorState::SelectTrack => {
                                beetlestatelist.items = beetlerank.get_cups()?.clone();
                                beetlestatelist.select(0);
                                trackselstate = TrackSelectorState::SelectCup;
                            },
                            _ => {},
                        }}
                        _ => {},
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            print!("{esc}[2J{esc}[1;1H", esc=27 as char);
            println!("Program State: {}", state);
            println!("Debug mode: {}", DEBUG.get());
            println!("Tick rate: {}", last_tick.elapsed().as_millis());
            println!("---");
            cup_window = beetlestatelist.viewport().pad(1).border(feotui::BorderStyle::Bold);
            match state {
                ProgramState::Speedometer => {
                    println!("{}", lorem_ipsum.pad(1).border(feotui::BorderStyle::Bold).render());
                },
                ProgramState::TrackSelector => println!("{}", lorem_ipsum.pad(1).border(feotui::BorderStyle::Bold).popup(&cup_window, 56, 15).render()),
                _ => {},
            }
            last_tick = Instant::now();
        }
    }
    Ok(())
}