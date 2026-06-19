| Algoritmo | Qubits | Python | Rust | Mejora (Speedup) |
|-----------|--------|--------|------|------------------|
| Cancellation Chain | 1 | 0.4058 s | 0.000471 s | 862.1x |
| Cancellation Chain | 2 | 1.0724 s | 0.000999 s | 1,073.2x |
| Cancellation Chain | 4 | 3.2246 s | 0.002398 s | 1,344.5x |
| Cancellation Chain | 8 | 11.2398 s | 0.006288 s | 1,787.5x |
| CNOT Cascade | 2 | 0.0967 s | 0.001267 s | 76.4x |
| CNOT Cascade | 4 | 1.0589 s | 0.009877 s | 107.2x |
| CNOT Cascade | 8 | 10.4086 s | 0.194538 s | 53.5x |
| Irreducible Brick Wall | 1 | 1.2277 s | 0.000360 s | 3,413.8x |
| Irreducible Brick Wall | 2 | 7.2644 s | 0.001876 s | 3,873.1x |
| Irreducible Brick Wall | 4 | 123.7389 s | 0.023126 s | 5,350.7x |
| Irreducible Brick Wall | 8 | 300.0290 s | 0.470591 s | 637.6x |
