// This is an implementation of the Uniform(a, b) function implemented on
// page of 63 of "Discrete-Event Simulation: A First Course" by Lawerence M. Leemis
// and Stephen K. Park. You simply give it an r, generated from a lehmer generator in this
// library
pub fn uniform(a: f64, b: f64, r: f64) -> f64 {
    a + (b - a) * r
}

// This is an implementation of Equilikely defined on page 64 of the above
// referenced book by Leemis et. al.
pub fn equlikely(a: i64, b: i64, r: f64) -> i64 {
    a + ((b - a) as f64 * r) as i64
}

// This is an implementation of Exponential(mu) defined on page 101, as above
pub fn exponential(mu: f64, r: f64) -> f64 {
    -mu * (1.0f64 - r).ln()
}

// This is an implemtation of Generic(p) defined on page 105, as above
pub fn geometric(p: f64, r:f64) -> i64 {
    ((1.0f64 - r).ln() / p.ln()) as i64
}


