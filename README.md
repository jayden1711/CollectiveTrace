# CollectiveTrace

A profiling tool that identifies which CUDA kernels are blocking NCCL collectives during distributed training.

## Problem

Existing tools (PyTorch Profiler, Nsight Systems, ncclsee) don't correlate 
kernel-level SM activity with collective communication timing.

## Architecture

- **Rust**: trace parsing and timeline alignment
- **Python**: analysis and visualization

## Setup

```
pip install maturin
maturin develop --release
```

This builds the Rust extension and installs the `collectivetrace` Python package
into your active environment. Use `--release`, the debug build is noticeably
slower than pure Python on anything but tiny files.

## Generate a test fixture

```
python fixtures/generate_fixture.py
```

Writes `fixtures/sample_trace.json`, a synthetic 50k-event trace. This is not a
real Kineto trace, it exists to exercise the parser and the benchmark before
real GPU-generated traces are available.

## Tests

Rust:
```
cargo test
```

Python (requires the extension to be built first):
```
python -m pytest python/tests/
```

## Benchmarks

Pure Rust, no Python overhead:
```
cargo bench
```

Rust extension called from Python vs. a pure Python equivalent, same fixture:
```
python python/benchmarks/bench_vs_python.py
```

## Layout

```
src/parser.rs           trace parsing logic, no Python dependency
src/lib.rs               PyO3 bindings around parser.rs
tests/parse_test.rs      Rust tests
benches/parse_bench.rs   Rust-only timing benchmark
python/collectivetrace/  Python package wrapping the compiled extension
python/tests/            Python tests
python/benchmarks/       Python vs Rust benchmark
fixtures/                fixture generator and generated trace files
```


## Related Works
- [ncclsee](https://arxiv.org/abs/2301.10420): closest existing work; NCCL operation profiling without SM contention analysis
- [Holistic Trace Analysis](https://github.com/facebookresearch/HolisticTraceAnalysis): Meta's distributed trace analysis library; CollectiveTrace sits alongside HTA rather than replacing it
- [tlparse](https://github.com/meta-pytorch/tlparse): Meta's Rust-based PyTorch trace parser; architectural reference for the Rust parsing layer
- [Demystifying Parallel and Distributed Deep Learning](https://arxiv.org/abs/1802.09941) (Ben-Nun & Hoefler): theoretical framework for compute-communication overlap analysis
- [PyTorch Distributed: Experiences on Accelerating Data Parallel Training](https://arxiv.org/abs/2006.15704): DDP bucketing mechanism and overlap strategy
- [ZeRO: Memory Optimizations Toward Training Trillion Parameter Models](https://arxiv.org/abs/1910.02054): memory optimization context for distributed training
<<<<<<< HEAD
- [Lagom](https://arxiv.org/abs/2402.15512) — current frontier on compute-communication overlap contention
- CUPTI documentation: underlying interface for kernel-level GPU activity collection
=======
- [Lagom](https://arxiv.org/abs/2402.15512): current frontier on compute-communication overlap contention
- CUPTI documentation: underlying interface for kernel-level GPU activity collection
