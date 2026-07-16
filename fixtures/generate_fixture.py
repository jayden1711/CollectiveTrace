import json
import os
import random

random.seed(0)

NAMES = ["matmul", "layernorm", "attention", "ncclAllReduce", "ncclReduceScatter", "memcpy"]


def make_trace(num_events, path):
    events = []
    ts = 0.0
    for _ in range(num_events):
        dur = random.uniform(1.0, 200.0)
        events.append(
            {
                "name": random.choice(NAMES),
                "ts": ts,
                "dur": dur,
                "pid": random.randint(0, 7),
                "tid": random.randint(0, 3),
            }
        )
        ts += dur

    with open(path, "w") as f:
        json.dump({"traceEvents": events}, f)


if __name__ == "__main__":
    here = os.path.dirname(__file__)
    make_trace(50000, os.path.join(here, "sample_trace.json"))
    print("wrote fixtures/sample_trace.json")
