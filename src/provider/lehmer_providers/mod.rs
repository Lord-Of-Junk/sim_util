use provider::Provider;

pub static M_VAL: i64 = 2147483647;
pub static A_VAL: i64 = 48271;
pub static Q_VAL: i64 = 44488; // Quotient of M/A
pub static R_VAL: i64 = 3399; // M mod A

// An implementation of the Lehmer Generator presented on page 54
// of "Discrete-Event Simulation: A First Course" by Lawerence M. Leemis
// and Stephen K. Park
pub struct LehmerProvider {
    x: i64, // The seed and current value of our generator
}

impl Provider<f64> for LehmerProvider {
    fn next(&mut self) -> f64 {
        let m = M_VAL.clone();
        let a = A_VAL.clone();
        let q = Q_VAL.clone();
        let r = R_VAL.clone();
        let t = a * (self.x % q) - r * (self.x / q);
        if t > 0i64 {
            self.x = t;
        }
        else {
            self.x = t + m;
        }
        self.x as f64 / m as f64
    }
}

impl LehmerProvider {

    pub fn new(seed: i64) -> LehmerProvider {
        LehmerProvider {
            x: seed,
        }
    }

    pub fn set_seed(&mut self, seed: i64) {
        self.x = seed;
    }

    pub fn see_state(&self) -> i64 {
        self.x.clone()
    }
}
