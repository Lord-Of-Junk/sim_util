use des::event::Event;
use std::collections::BinaryHeap;

pub struct EventQueue {
    queue: BinaryHeap<Event>,
    event_total: u64,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue {
            queue: BinaryHeap::<Event>::new(),
            event_total: 0,
        }
    }

    pub fn add_event(&mut self, arrival: u64, event_type: &str, description: &str) {
        let e = Event::new(self.event_total, arrival, event_type, description);
        self.event_total += 1;
        self.queue.push(e);
    }

    pub fn next(&mut self) -> Option<Event> {
        self.queue.pop()
    }

    pub fn events_seen(&self) -> u64 {
        self.event_total.clone()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

}

#[cfg(test)]
mod event_queue_test {

    use des::event_queue::*;

    // A test to be sure we can pop events on and get them in the proper order
    #[test]
    fn test_queue() { 
        let mut q = EventQueue::new();
        q.add_event(0, "test_event", "First event");
        q.add_event(4, "test_event", "Second event");
        q.add_event(2, "test_event", "Third event");
        q.add_event(3, "test_event", "Fourth event");
        for id in [0u64, 2u64, 3u64, 1u64].iter() {
            assert!(q.next().unwrap().get_id() == *id);
        }
    }

    // Test to be sure we count the number of events properly
    #[test]
    fn test_counts() {
        let mut q = EventQueue::new();
        q.add_event(0, "test_event", "First");
        q.add_event(4, "test_event", "First");
        q.add_event(6, "test_event", "First");
        assert!(q.events_seen() == 3);
        assert!(q.len() == 3);
        q.next();
        q.next();
        q.next();
        q.add_event(7, "test_event", "First");
        q.add_event(8, "test_event", "First");
        q.add_event(9, "test_event", "First");
        assert!(q.events_seen() == 6);
        assert!(q.len() == 3);
    }
}
