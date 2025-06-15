#[derive(Debug, PartialEq)]
pub struct Event {
    pub start: u32,
    pub end: u32,
}

pub fn greedy_event_selection(mut events: Vec<Event>) -> Vec<Event> {
    events.sort_by(|a, b| a.end.cmp(&b.end));

    let mut selected_events = Vec::new();
    let mut last_end_time = 0;

    for event in events {
        if event.start >= last_end_time {
            selected_events.push(event);
            last_end_time = selected_events.last().unwrap().end;
        }
    }

    selected_events
}
