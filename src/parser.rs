use serde::Deserialize;

// A single event from a trace source (Kineto, Nsight, NCCL)
#[derive(Debug, Deserialize)]
pub struct TraceEvent {
    pub name: String,
    pub ts: f64,        // timestamp in microseconds
    pub dur: f64,       // duration in microseconds
    pub cat: String,    // category: "kernel", "collective"
}

// Each parser takes in one trace source and returns a vec of events
pub trait TraceParser {
    fn name(&self) -> &'static str;
    fn parse(&self, path: &str) -> anyhow::Result<Vec<TraceEvent>>;
}

// Kineto JSON trace parser
pub struct KinetoParser;
impl TraceParser for KinetoParser {
    fn name(&self) -> &'static str { "kineto" }
    fn parse(&self, path: &str) -> anyhow::Result<Vec<TraceEvent>> {
        let content = std::fs::read_to_string(path)?;
        let events: Vec<TraceEvent> = serde_json::from_str(&content)?;
        Ok(events)
    }
}