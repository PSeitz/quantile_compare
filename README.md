# Quantile Compare / Benchmark

Compare different quantile algorithms in rust in terms of performance, memory usage and accuracy.

Test with different distributions and a real world data set based on air quality.

### Run Suite
`cargo run --release`

To observe memory consumption with multiple collectors collecting at the same time
`cargo run --release --features parallel-collect`

### Comments

- AllValues: Naive and Exact solution by storing all values in a sorted array.
- TDigest: Fork of https://github.com/MnO2/t-digest. Fixing the most severe performance issues, but there's still a lot of headroom.
- HDRHistogram: Supports only u64 values, and is not viable for some use cases.
- DDSketch: Fork of https://crates.io/crates/sketches-ddsketch. Added a simple serialization via serde.
- DDSketch2: https://crates.io/crates/sketches-rust. Pretty new crate, uses a faster alternative to `val.ln()` used by DDSketch.

#### Serialization
Only HDRHistogram has a specialized implementation. For the others simply `serde::to_json()` is used.

#### Counts
If there are multiple counts, that means they are collected and then merged.
Run with `cargo run --release --features parallel-collect`


COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.000s | 12k        | 7k             | 0.51 | 0.64 | 0.75 | 0.83 | 0.98 | 1.08 | 1.08  |
| Normal Distribution | TDigest      | 0.000s | 13k        | 11k            | 0.50 | 0.63 | 0.75 | 0.83 | 0.97 | 1.14 | 1.17  |
| Normal Distribution | HDRHistogram | 0.000s | 2k         | 43             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.000s | 6k         | 1765           | 0.50 | 0.64 | 0.75 | 0.83 | 0.95 | 1.26 | 1.26  |
| Normal Distribution | DDSketch2    | 0.000s | 6k         | unavailable    | 0.50 | 0.63 | 0.75 | 0.81 | 0.95 | 1.09 | 1.09  |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.509s | 85068k     | 39062k         | 0.50 | 0.63 | 0.76 | 0.83 | 0.96 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.180s | 13k        | 13k            | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.11 | 1.24  |
| Normal Distribution | HDRHistogram | 0.069s | 2k         | 47             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.174s | 16k        | 4k             | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.137s | 14k        | unavailable    | 0.50 | 0.64 | 0.75 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.400s | 52687k     | 31257k         | 0.50 | 0.63 | 0.76 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.143s | 31k        | 13k            | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.12 | 1.25  |
| Normal Distribution | HDRHistogram | 0.057s | 6k         | 47             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.141s | 32k        | 4k             | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.109s | 45k        | unavailable    | 0.50 | 0.64 | 0.75 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.092s | 12098k     | 7812k          | 0.50 | 0.64 | 0.76 | 0.83 | 0.96 | 1.12 | 1.23  |
| Normal Distribution | TDigest      | 0.058s | 9101k      | 13k            | 0.50 | 0.63 | 0.76 | 0.83 | 0.97 | 1.12 | 1.25  |
| Normal Distribution | HDRHistogram | 0.015s | 2205k      | 45             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.038s | 4944k      | 3k             | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.032s | 5103k      | unavailable    | 0.50 | 0.64 | 0.75 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.000s | 12k        | 7k             | 5.34 | 5.72 | 6.30 | 6.77 | 7.98 | 17.43 | 17.43 |
| Pareto Distribution | TDigest      | 0.000s | 13k        | 11k            | 5.36 | 5.73 | 6.35 | 6.88 | 8.32 | 9.45  | 9.75  |
| Pareto Distribution | HDRHistogram | 0.000s | 2k         | 51             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 13.00 | 13.00 |
| Pareto Distribution | DDSketch     | 0.000s | 1k         | 722            | 5.31 | 5.75 | 6.23 | 6.62 | 7.92 | 9.30  | 9.30  |
| Pareto Distribution | DDSketch2    | 0.000s | 1k         | unavailable    | 5.33 | 5.77 | 6.37 | 6.63 | 8.08 | 9.85  | 9.85  |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.552s | 85068k     | 39062k         | 5.36 | 5.74 | 6.29 | 6.75 | 7.92 | 9.99  | 12.58 |
| Pareto Distribution | TDigest      | 0.231s | 13k        | 12k            | 5.36 | 5.74 | 6.29 | 6.75 | 7.93 | 9.93  | 12.32 |
| Pareto Distribution | HDRHistogram | 0.136s | 2k         | 76             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.252s | 1k         | 882            | 5.31 | 5.75 | 6.23 | 6.75 | 7.92 | 10.07 | 12.55 |
| Pareto Distribution | DDSketch2    | 0.221s | 1k         | unavailable    | 5.33 | 5.77 | 6.24 | 6.76 | 7.93 | 10.04 | 12.49 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.439s | 52687k     | 31257k         | 5.36 | 5.74 | 6.30 | 6.75 | 7.93 | 9.95  | 12.46 |
| Pareto Distribution | TDigest      | 0.184s | 31k        | 12k            | 5.36 | 5.74 | 6.29 | 6.75 | 7.92 | 10.09 | 12.51 |
| Pareto Distribution | HDRHistogram | 0.109s | 6k         | 75             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.202s | 4k         | 879            | 5.31 | 5.75 | 6.23 | 6.75 | 7.92 | 9.88  | 12.55 |
| Pareto Distribution | DDSketch2    | 0.176s | 4k         | unavailable    | 5.33 | 5.77 | 6.24 | 6.76 | 7.93 | 10.04 | 12.74 |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.100s | 12098k     | 7812k          | 5.36 | 5.74 | 6.29 | 6.75 | 7.92 | 9.88  | 12.82 |
| Pareto Distribution | TDigest      | 0.067s | 9101k      | 12k            | 5.36 | 5.74 | 6.29 | 6.74 | 7.92 | 9.96  | 12.54 |
| Pareto Distribution | HDRHistogram | 0.028s | 2205k      | 68             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 10.00 | 12.00 |
| Pareto Distribution | DDSketch     | 0.053s | 1363k      | 840            | 5.31 | 5.75 | 6.23 | 6.75 | 7.92 | 10.07 | 12.55 |
| Pareto Distribution | DDSketch2    | 0.047s | 821k       | unavailable    | 5.33 | 5.77 | 6.24 | 6.76 | 7.93 | 10.04 | 12.74 |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.000s | 12k        | 7k             | 20.59 | 39.14 | 69.15 | 102.24 | 213.66 | 341.40 | 341.40 |
| LogNorm Distribution | TDigest      | 0.000s | 13k        | 11k            | 19.89 | 37.96 | 69.41 | 101.11 | 203.38 | 458.16 | 527.81 |
| LogNorm Distribution | HDRHistogram | 0.000s | 4k         | 226            | 19.00 | 39.00 | 73.00 | 110.00 | 212.00 | 595.00 | 595.00 |
| LogNorm Distribution | DDSketch     | 0.000s | 4k         | 1235           | 19.89 | 39.25 | 68.72 | 100.49 | 186.82 | 820.71 | 820.71 |
| LogNorm Distribution | DDSketch2    | 0.000s | 4k         | unavailable    | 19.70 | 37.88 | 67.28 | 92.28  | 180.94 | 361.88 | 361.88 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.519s | 85068k     | 39062k         | 20.01 | 38.72 | 70.11 | 100.12 | 194.34 | 411.44 | 756.07 |
| LogNorm Distribution | TDigest      | 0.196s | 13k        | 12k            | 20.05 | 38.78 | 70.16 | 100.43 | 195.78 | 408.05 | 743.58 |
| LogNorm Distribution | HDRHistogram | 0.108s | 8k         | 1081           | 20.00 | 38.00 | 70.00 | 100.00 | 195.00 | 411.00 | 763.00 |
| LogNorm Distribution | DDSketch     | 0.204s | 8k         | 2k             | 19.89 | 38.48 | 70.11 | 100.49 | 194.44 | 407.54 | 757.61 |
| LogNorm Distribution | DDSketch2    | 0.205s | 4k         | unavailable    | 20.09 | 38.63 | 70.00 | 99.92  | 195.89 | 415.97 | 768.17 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.409s | 52687k     | 31257k         | 20.01 | 38.70 | 70.15 | 100.04 | 194.86 | 413.97 | 762.12 |
| LogNorm Distribution | TDigest      | 0.156s | 31k        | 13k            | 20.02 | 38.75 | 70.03 | 99.94  | 195.62 | 409.44 | 771.18 |
| LogNorm Distribution | HDRHistogram | 0.086s | 20k        | 1054           | 20.00 | 38.00 | 70.00 | 100.00 | 195.00 | 415.00 | 767.00 |
| LogNorm Distribution | DDSketch     | 0.150s | 13k        | 2k             | 19.89 | 38.48 | 70.11 | 100.49 | 194.44 | 407.54 | 772.92 |
| LogNorm Distribution | DDSketch2    | 0.164s | 23k        | unavailable    | 20.09 | 38.63 | 70.00 | 99.92  | 195.89 | 407.74 | 768.17 |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.095s | 12098k     | 7812k          | 20.03 | 38.76 | 70.25 | 100.14 | 194.57 | 407.36 | 723.90 |
| LogNorm Distribution | TDigest      | 0.060s | 9101k      | 12k            | 19.98 | 38.74 | 70.23 | 100.46 | 195.19 | 412.78 | 796.00 |
| LogNorm Distribution | HDRHistogram | 0.025s | 4262k      | 873            | 19.00 | 38.00 | 70.00 | 100.00 | 194.00 | 415.00 | 751.00 |
| LogNorm Distribution | DDSketch     | 0.043s | 4367k      | 2k             | 19.89 | 38.48 | 70.11 | 100.49 | 194.44 | 407.54 | 772.92 |
| LogNorm Distribution | DDSketch2    | 0.046s | 4174k      | unavailable    | 20.09 | 38.63 | 70.00 | 99.92  | 195.89 | 415.97 | 783.61 |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.000s | 12k        | 7k             | 20589.08 | 39142.38 | 69147.17 | 102240.64 | 213661.03 | 341397.11 | 341397.11 |
| LogNorm Distribution 1000x | TDigest      | 0.000s | 13k        | 11k            | 19885.93 | 37963.12 | 69405.38 | 101113.31 | 203383.83 | 458157.44 | 527813.52 |
| LogNorm Distribution 1000x | HDRHistogram | 0.000s | 16k        | 750            | 19455.00 | 39423.00 | 73727.00 | 110591.00 | 212991.00 | 598015.00 | 598015.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.000s | 4k         | 1236           | 20136.32 | 38960.45 | 68208.26 | 101756.13 | 185415.47 | 814560.26 | 814560.26 |
| LogNorm Distribution 1000x | DDSketch2    | 0.000s | 4k         | unavailable    | 19782.72 | 38037.52 | 67551.62 | 92659.16  | 181692.15 | 363392.69 | 363392.69 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.520s | 85068k     | 39062k         | 20007.52 | 38721.75 | 70111.13 | 100124.28 | 194339.47 | 411437.07 | 756069.38 |
| LogNorm Distribution 1000x | TDigest      | 0.197s | 13k        | 12k            | 20045.27 | 38777.60 | 70163.30 | 100430.90 | 195776.68 | 408049.77 | 743583.38 |
| LogNorm Distribution 1000x | HDRHistogram | 0.107s | 20k        | 3k             | 20095.00 | 38911.00 | 70655.00 | 100351.00 | 195583.00 | 411647.00 | 765951.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.186s | 8k         | 2k             | 20136.32 | 38960.45 | 69586.21 | 99741.16  | 192982.67 | 412660.72 | 751931.89 |
| LogNorm Distribution 1000x | DDSketch2    | 0.177s | 4k         | unavailable    | 20176.12 | 38794.63 | 70292.52 | 100318.96 | 196688.92 | 417624.12 | 771311.53 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.409s | 52687k     | 31257k         | 20006.56 | 38700.01 | 70154.46 | 100044.19 | 194857.13 | 413968.20 | 762121.41 |
| LogNorm Distribution 1000x | TDigest      | 0.159s | 31k        | 13k            | 20017.37 | 38745.14 | 70031.03 | 99942.69  | 195618.14 | 409436.18 | 771179.67 |
| LogNorm Distribution 1000x | HDRHistogram | 0.086s | 52k        | 2k             | 20095.00 | 38911.00 | 70655.00 | 100351.00 | 195583.00 | 415743.00 | 765951.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.154s | 13k        | 2k             | 20136.32 | 38960.45 | 69586.21 | 99741.16  | 192982.67 | 412660.72 | 767122.43 |
| LogNorm Distribution 1000x | DDSketch2    | 0.142s | 23k        | unavailable    | 20176.12 | 38794.63 | 70292.52 | 100318.96 | 196688.92 | 409372.62 | 771311.53 |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.097s | 12098k     | 7812k          | 20032.78 | 38757.19 | 70245.05 | 100139.34 | 194565.97 | 407363.33 | 723900.67 |
| LogNorm Distribution 1000x | TDigest      | 0.063s | 9101k      | 12k            | 19978.59 | 38738.61 | 70230.77 | 100462.54 | 195187.70 | 412782.44 | 795996.09 |
| LogNorm Distribution 1000x | HDRHistogram | 0.040s | 18252k     | 2k             | 20095.00 | 38911.00 | 70655.00 | 100863.00 | 194559.00 | 415743.00 | 753663.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.046s | 4367k      | 2k             | 20136.32 | 38960.45 | 69586.21 | 99741.16  | 196881.31 | 404489.22 | 767122.43 |
| LogNorm Distribution 1000x | DDSketch2    | 0.042s | 4174k      | unavailable    | 20176.12 | 38794.63 | 70292.52 | 100318.96 | 196688.92 | 417624.12 | 786792.18 |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.000s | 12k        | 7k             | 35.00 | 57.00 | 74.00 | 82.00 | 106.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | TDigest      | 0.000s | 13k        | 8k             | 34.84 | 57.00 | 74.00 | 82.17 | 105.50 | 130.00 | 136.00 |
| PM10 Air Quality Dataset | HDRHistogram | 0.000s | 2k         | 157            | 35.00 | 57.00 | 74.00 | 82.00 | 105.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.000s | 2k         | 969            | 34.82 | 57.40 | 74.45 | 82.28 | 104.60 | 125.22 | 125.22 |
| PM10 Air Quality Dataset | DDSketch2    | 0.000s | 2k         | unavailable    | 35.00 | 57.46 | 74.28 | 81.96 | 103.99 | 124.38 | 124.38 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.289s | 85068k     | 39062k         | 36.00 | 54.00 | 76.00 | 92.00 | 129.00 | 213.00 | 410.00 |
| PM10 Air Quality Dataset | TDigest      | 0.121s | 13k        | 10k            | 35.91 | 54.00 | 76.41 | 92.40 | 129.35 | 214.33 | 378.86 |
| PM10 Air Quality Dataset | HDRHistogram | 0.048s | 8k         | 835            | 36.00 | 54.00 | 76.00 | 92.00 | 129.00 | 213.00 | 411.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.117s | 5k         | 1957           | 36.24 | 54.06 | 75.95 | 92.77 | 127.75 | 214.89 | 407.54 |
| PM10 Air Quality Dataset | DDSketch2    | 0.104s | 5k         | unavailable    | 35.70 | 54.11 | 75.76 | 92.28 | 129.30 | 212.18 | 407.74 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.238s | 52687k     | 31257k         | 37.00 | 57.00 | 80.00 | 97.00 | 134.00 | 220.00 | 426.00 |
| PM10 Air Quality Dataset | TDigest      | 0.093s | 31k        | 12k            | 37.00 | 56.75 | 80.03 | 96.69 | 133.81 | 221.42 | 420.84 |
| PM10 Air Quality Dataset | HDRHistogram | 0.038s | 14k        | 827            | 37.00 | 57.00 | 80.00 | 97.00 | 134.00 | 220.00 | 427.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.109s | 12k        | 1954           | 36.97 | 57.40 | 80.65 | 96.55 | 132.97 | 219.23 | 424.18 |
| PM10 Air Quality Dataset | DDSketch2    | 0.083s | 15k        | unavailable    | 37.14 | 57.46 | 80.37 | 97.94 | 134.56 | 220.84 | 424.37 |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.024s | 12098k     | 7812k          | 35.00 | 57.00 | 74.00 | 82.00 | 106.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | TDigest      | 0.026s | 9101k      | 10k            | 34.92 | 57.09 | 74.00 | 82.25 | 105.48 | 131.15 | 136.00 |
| PM10 Air Quality Dataset | HDRHistogram | 0.012s | 2205k      | 314            | 35.00 | 57.00 | 74.00 | 82.00 | 105.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.020s | 2363k      | 1251           | 34.82 | 57.40 | 74.45 | 82.28 | 104.60 | 125.22 | 135.65 |
| PM10 Air Quality Dataset | DDSketch2    | 0.024s | 2316k      | unavailable    | 35.00 | 57.46 | 74.28 | 81.96 | 103.99 | 124.38 | 137.26 |
