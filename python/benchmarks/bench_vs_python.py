import json
import os
import time

from collectivetrace import parse_trace_summary

FIXTURE = os.path.join(os.path.dirname(__file__), "..", "..", "fixtures", "sample_trace.json")


def parse_with_python(path):
    with open(path) as f:
        trace = json.load(f)
    events = trace["traceEvents"]
    total = sum(e.get("dur", 0.0) for e in events)
    return len(events), total


def time_it(fn, path, runs=5):
    times = []
    for _ in range(runs):
        start = time.perf_counter()
        fn(path)
        times.append(time.perf_counter() - start)
    return min(times)


if __name__ == "__main__":
    if not os.path.exists(FIXTURE):
        raise SystemExit("run fixtures/generate_fixture.py first")

    python_time = time_it(parse_with_python, FIXTURE)
    rust_time = time_it(parse_trace_summary, FIXTURE)

    print(f"python parse time: {python_time:.4f}s")
    print(f"rust parse time:   {rust_time:.4f}s")
    print(f"speedup: {python_time / rust_time:.1f}x")
