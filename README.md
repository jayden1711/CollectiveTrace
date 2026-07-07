# CollectiveTrace

A profiling tool that identifies which CUDA kernels are blocking NCCL collectives during distributed training.

## Problem

Existing tools (PyTorch Profiler, Nsight Systems, ncclsee) don't correlate 
kernel-level SM activity with collective communication timing.

## Architecture

- **Rust**: trace parsing and timeline alignment
- **Python**: analysis and visualization

## Related Works
- [ncclsee](https://arxiv.org/abs/2301.10420): closest existing work; NCCL operation profiling without SM contention analysis
- [Holistic Trace Analysis](https://github.com/facebookresearch/HolisticTraceAnalysis): Meta's distributed trace analysis library; CollectiveTrace sits alongside HTA rather than replacing it
- [tlparse](https://github.com/meta-pytorch/tlparse): Meta's Rust-based PyTorch trace parser; architectural reference for the Rust parsing layer
- [Demystifying Parallel and Distributed Deep Learning](https://arxiv.org/abs/1802.09941) (Ben-Nun & Hoefler): theoretical framework for compute-communication overlap analysis
- [PyTorch Distributed: Experiences on Accelerating Data Parallel Training](https://arxiv.org/abs/2006.15704): DDP bucketing mechanism and overlap strategy
- [ZeRO: Memory Optimizations Toward Training Trillion Parameter Models](https://arxiv.org/abs/1910.02054): memory optimization context for distributed training
- [Lagom](https://arxiv.org/abs/2402.15512) — current frontier on compute-communication overlap contention
- CUPTI documentation: underlying interface for kernel-level GPU activity collection



