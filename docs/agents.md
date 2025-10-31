# Tiny Agent Trainer — Agents (Phase Overview)

This file defines minimal roles and hand-offs for the four phases. Keep it short for patch stability.

## Phase I — Architect Agent (Core)
- Goal: Implement a trainable encoder–decoder Transformer (CPU).
- Tasks:
  - Token + positional embeddings
  - Encoder/decoder stacks (6 layers, d_model=512, nhead=8)
  - Cross-entropy loss, AdamW, checkpoints
  - Greedy decoding
- Exit criteria: Overfit a 20-sample WGSL toy set; checkpoint save/load round-trip.

## Phase II — Optimizer Agent (CPU Perf)
- Goal: <60 ms inference on seq_len≈512.
- Tasks:
  - Batched matmul in attention; parallelize heads (rayon)
  - Cache masks/pos enc
  - Criterion benchmarks (attention/softmax/tokenizer)
- Exit criteria: Attention ≤15 ms; softmax ≤4 µs; stable micro-bench.

## Phase III — Accelerator Agent (GPU)
- Goal: Offload attention/FFN via wgpu; FP16 option.
- Tasks:
  - WGSL compute kernels; GPU buffer manager
  - CPU/GPU parity tests
- Exit criteria: ≥100× training throughput vs CPU on small batch; parity within tolerance.

## Phase IV — Integrator Agent (Polish)
- Goal: Release-ready UX and reproducibility.
- Tasks:
  - Beam search / top-k decoding
  - Property tests; CLI subcommands (`train`, `generate`)
  - Build artifacts + baseline benches
- Exit criteria: Reproducible build, passing tests, documented CLI.

## Manifests (Phase Handoff)
Agents write a minimal JSON to `target/manifests/<phase>.json`:
```json
{ "phase":"optimizer", "status":"verified", "bench": { "attention_ms": 12.4 } }
```

## Single Source of Truth
- Config: `config/train.toml` (dims, lr, dropout)
- Spec: `docs/FINAL_SPEC.md` (minimal model contract)
