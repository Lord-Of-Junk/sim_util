pub mod trace_provider;
pub mod lehmer_providers;
pub mod variates;

pub trait Provider<T> {
    fn next(&mut self) -> T;
}
