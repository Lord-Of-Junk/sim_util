use std::fmt;
use std::cmp::Ordering;

#[derive(Eq)]
pub struct Event {
    id: u64,
    arr_time: u64,
    event_type: String,
    description: String,
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        match self.arr_time.cmp(&other.arr_time) {
            Ordering::Less => Ordering::Greater, // Lower arrival times have a higher priority
            Ordering::Greater => Ordering::Less, // Later arrival times have a lower priority
            Ordering::Equal => match self.id < other.id { // Do they tie? Shit...
                true => Ordering::Greater,  // ... just take the one with the smaller id!
                false => Ordering::Less,
            },
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.id == other.id
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EVENT[id: {}, arr_time: {}, type: {}]\n\tDESCRIPTION: {}", 
                self.id,
                self.arr_time,
                self.event_type,
                self.description)
    }
}

impl Event {
    pub fn new(id: u64, arr_time: u64, event_type: &str, description: &str) -> Event {
        Event {
            id: id,
            arr_time: arr_time,
            event_type: String::from(event_type),
            description: String::from(description),
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id.clone()
    }

    pub fn get_type(&self) -> String {
        self.event_type.clone()
    }
}

#[cfg(test)]
mod event_tests {

    use des::event::*;

    // Tests to determine if two events with the same id are equal
    #[test]
    fn test_eq() {
        let t1 = Event::new(0, 0, "", "");
        let t2 = Event::new(1, 1, "", "");
        assert!(t1 != t2);
    }

    // Test to determine if the proper things are greater than the proper things
    #[test]
    fn test_gt() {
        let t1 = Event::new(0, 0, "", "");
        let t2 = Event::new(1, 1, "", "");
        let t3 = Event::new(2, 0, "", "");
        assert!(t1 > t2);
        assert!(t2 < t3);
        assert!(t3 < t1);
    }

    // Test to determine if the fmt function for our event works
    #[test]
    fn test_fmt() {
        let t1 = Event::new(0, 5, "test_event", "An event for testing");
        let d1 = format!("{}", t1);
        assert!("EVENT[id: 0, arr_time: 5, type: test_event]\n\tDESCRIPTION: An event for testing" == d1);
    }
}
