// TODO: implement packet handler + event handler

#[derive(Debug)]
struct Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventPriority {
    Lowest = -2,
    Low = -1,
    Normal = 0,
    High = 1,
    Highest = 2,
 // Monitor = 3, // can not mutate the event but has the highest priority
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventResult<T, E> {
    result: Result<Option<T>, E>,
    is_canceled: bool,
}

impl<T, E> EventResult<T, E> {
    fn new() -> Self {
        Self {result: Ok(None), is_canceled: false}
    }
}

struct EventScheduler<T: ClientEvent> {
    events: Vec<T>,
    results: Vec<EventResult<T::EventOutput, T::EventError>>,
}

impl<T: ClientEvent> EventScheduler<T> {
    #[inline]
    fn new() -> Self {
        Self { events: Vec::new(), results: Vec::new() }
    }

    pub fn register_event(&mut self, event: T) -> bool {
        if !self.events.contains(&event) {
            self.events.push(event);
            true
        } else {
            false
        }
    }

    pub fn process_all(&mut self) -> &Vec<EventResult<T::EventOutput, T::EventError>> {
        self.events.sort();
        
        self.results = Vec::with_capacity(self.events.len());
        for event in &mut self.events {
            event.process();
            self.results.push(event.get_result());
        }
        &self.results
    }
}

// base for events
trait ClientEvent: Clone + PartialEq + Ord + PartialOrd {
    type EventOutput;
    type EventError;

    // stores the priority of the event
    const PRIORITY: EventPriority;

    fn set_result(&mut self, result: EventResult<Self::EventOutput, Self::EventError>);
    fn get_result(&self) -> EventResult<Self::EventOutput, Self::EventError>;
    fn process(&mut self);
}