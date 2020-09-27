pub struct WelfordTracker {
    count: u64,
    mean: f64,
    variance: f64
}

impl WelfordTracker {

    pub fn new() -> WelfordTracker {
        WelfordTracker {
            count: 0u64,
            mean: 0f64,
            variance: 0f64,
        }
    }

    pub fn get_count(&self) -> u64 {
        self.count.clone()
    }

    pub fn get_mean(&self) -> f64 {
        self.mean.clone()
    }

    pub fn get_variation(&self) -> f64 {
        self.variance.clone()
    }

    pub fn get_deviation(&self) -> f64 {
        (self.variance / self.count as f64).sqrt()
    }

    pub fn add_datum(&mut self, datum: f64) {
        self.count = self.count + 1;
        let temp = datum - self.mean;
        self.variance = self.variance + temp * temp * ((self.count as f64 - 1.0) / self.count as f64);
        self.mean = self.mean + temp / self.count as f64;
    }

}

#[cfg(test)]
mod stat_test {

    use stat::*;

    // Test to be sure we count the number data properly
    #[test]
    fn test_count() {
        let mut subject = get_subject();
        assert_eq!(subject.get_count(), 6);
    }

    #[test]
    fn test_mean() {
        let mut subject = get_subject();
        assert_eq!(subject.get_mean(), 3.5);
    }

    fn get_subject() -> WelfordTracker {
        let mut subject = WelfordTracker::new();
        subject.add_datum(1.0);
        subject.add_datum(2.0);
        subject.add_datum(3.0);
        subject.add_datum(4.0);
        subject.add_datum(5.0);
        subject.add_datum(6.0);
        subject
    }

}