use serde::Deserialize;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Deserialize)]
struct TraceFile {
    #[serde(rename = "traceEvents")]
    trace_events: Vec<TraceEvent>,
}

#[derive(Deserialize)]
struct TraceEvent {
    #[allow(dead_code)]
    name: Option<String>,
    #[allow(dead_code)]
    ts: Option<f64>,
    dur: Option<f64>,
    #[allow(dead_code)]
    pid: Option<i64>,
    #[allow(dead_code)]
    tid: Option<i64>,
}

pub struct TraceSummary {
    pub event_count: usize,
    pub total_duration_us: f64,
}

pub fn parse_file(path: &Path) -> io::Result<TraceSummary> {
    let data = fs::read_to_string(path)?;
    let trace: TraceFile = serde_json::from_str(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let total_duration_us: f64 = trace
        .trace_events
        .iter()
        .filter_map(|event| event.dur)
        .sum();

    Ok(TraceSummary {
        event_count: trace.trace_events.len(),
        total_duration_us,
    })
}
