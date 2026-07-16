import json
import os
import tempfile

from collectivetrace import parse_trace_summary


def test_parses_event_count_and_duration():
    trace = {
        "traceEvents": [
            {"name": "matmul", "ts": 100.0, "dur": 50.0, "pid": 1, "tid": 1},
            {"name": "ncclAllReduce", "ts": 200.0, "dur": 75.0, "pid": 1, "tid": 2},
        ]
    }

    fd, path = tempfile.mkstemp(suffix=".json")
    with os.fdopen(fd, "w") as f:
        json.dump(trace, f)

    count, total_dur = parse_trace_summary(path)
    os.remove(path)

    assert count == 2
    assert total_dur == 125.0


def test_missing_file_raises():
    try:
        parse_trace_summary("does_not_exist.json")
        assert False, "expected an error"
    except OSError:
        pass
