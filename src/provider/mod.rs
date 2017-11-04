pub mod trace_provider;
pub mod lehmer_providers;

pub trait Provider<T> {
    fn next(&mut self) -> T;
}
