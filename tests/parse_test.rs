use collectivetrace::parser::parse_file;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn parses_event_count_and_duration() {
    let mut file = NamedTempFile::new().unwrap();
    let json = r#"{"traceEvents": [
        {"name": "matmul", "ts": 100.0, "dur": 50.0, "pid": 1, "tid": 1},
        {"name": "ncclAllReduce", "ts": 200.0, "dur": 75.0, "pid": 1, "tid": 2}
    ]}"#;
    write!(file, "{}", json).unwrap();

    let summary = parse_file(file.path()).unwrap();

    assert_eq!(summary.event_count, 2);
    assert_eq!(summary.total_duration_us, 125.0);
}

#[test]
fn handles_missing_duration_field() {
    let mut file = NamedTempFile::new().unwrap();
    let json = r#"{"traceEvents": [
        {"name": "marker", "ts": 10.0}
    ]}"#;
    write!(file, "{}", json).unwrap();

    let summary = parse_file(file.path()).unwrap();

    assert_eq!(summary.event_count, 1);
    assert_eq!(summary.total_duration_us, 0.0);
}

#[test]
fn errors_on_missing_file() {
    let result = parse_file(std::path::Path::new("does_not_exist.json"));
    assert!(result.is_err());
}
