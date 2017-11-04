use provider::Provider;

// An implementation of the Lehmer Generator presented on page 54
// of "Discrete-Event Simulation: A First Course" by Lawerence M. Leemis
// and Stephen K. Park
pub struct LehmerProvider {
    x: i64, // The seed and current value of our generator
    a: i64,
    m: i64,
    q: i64,
    r: i64,
}

impl Provider<f64> for LehmerProvider {
    fn next(&mut self) -> f64 {
        let t = self.a * (self.x % self.q) - self.r * (self.x / self.q);
        if t > 0i64 {
            self.x = t;
        }
        else {
            self.x = t + self.m;
        }
        self.x as f64 / self.m as f64
    }
}

impl LehmerProvider {

    pub fn new(seed: i64) -> LehmerProvider {
        LehmerProvider {
            x: seed,
            a: 48271,
            m: 2147483647,
            q: 2147483647/48271,
            r: 2147483647%48271,
        }
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.x = seed;
    }

    pub fn see_state(&self) -> i64 {
        self.x.clone()
    }
}
