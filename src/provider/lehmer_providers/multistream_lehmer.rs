use provider::Provider;
use provider::lehmer_providers::LehmerProvider;

struct MultiStreamLehmer {
    streams: Vec<LehmerProvider>,
}

impl MultiStreamLehmer {

    // Create a new Lehmer generator that manages multiple streams
    // seed -> Starting value for the first stream
    // streams -> The number of streams to use
    pub fn new(seed: i64, streams: u64) -> MultiStreamLehmer {

    }

}
