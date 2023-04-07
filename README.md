# Quantile Compare / Benchmark

Compare different quantile algorithms in rust in terms of performance, memory usage and accuracy.
The quantile algorithms run unbounded, no min, max information is passed.

Test with different distributions and a real world data set based on air quality.

#### Run Suite
`cargo run --release`

To observe memory consumption with multiple collectors collecting at the same time (without threading)
`cargo run --release --features parallel-collect`

#### Comments

- AllValues: Naive and Exact solution by storing all values in a sorted array.
- TDigest: Fork of https://github.com/MnO2/t-digest. Fixing the most severe performance issues, but there's still a lot of headroom.
- HDRHistogram: Supports only u64 values, and is not viable for some use cases.
- DDSketch: Fork of https://crates.io/crates/sketches-ddsketch. Added a simple serialization via serde.
- DDSketch2: https://crates.io/crates/sketches-rust. Pretty new crate, has a cubically interpolated variant, which is faster than `val.ln()` used by DDSketch.

#### Serialization
Only HDRHistogram has a specialized implementation. For the others simply `serde::to_json()` is used.

#### Counts
If there are multiple counts, that means they are collected and then merged.
Run with `cargo run --release --features parallel-collect`

## Contributing
To add a quantile algorithm, simply implement the `Aggregate` trait.

###$ TODO
- Display in a graph 

## Results

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.000s | 12k        | 7k             | 0.51 | 0.64 | 0.75 | 0.83 | 0.98 | 1.08 | 1.08  |
| Normal Distribution | TDigest      | 0.000s | 13k        | 11k            | 0.50 | 0.63 | 0.75 | 0.83 | 0.97 | 1.14 | 1.17  |
| Normal Distribution | HDRHistogram | 0.000s | 2k         | 43             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.000s | 6k         | 1765           | 0.50 | 0.64 | 0.75 | 0.83 | 0.95 | 1.26 | 1.26  |
| Normal Distribution | DDSketch2    | 0.000s | 6k         | unavailable    | 0.50 | 0.63 | 0.75 | 0.81 | 0.95 | 1.09 | 1.09  |
| Normal Distribution | Quantogram   | 0.000s | 20k        | unavailable    | 0.51 | 0.64 | 0.77 | 0.85 | 0.93 | 1.16 | 1.16  |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.531s | 85068k     | 39062k         | 0.50 | 0.63 | 0.76 | 0.83 | 0.96 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.189s | 13k        | 13k            | 0.50 | 0.63 | 0.76 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | HDRHistogram | 0.075s | 2k         | 47             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.147s | 16k        | 4k             | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.104s | 16k        | unavailable    | 0.50 | 0.64 | 0.75 | 0.83 | 0.97 | 1.12 | 1.26  |
| Normal Distribution | Quantogram   | 0.496s | 127k       | unavailable    | 0.51 | 0.64 | 0.75 | 0.83 | 0.97 | 1.12 | 1.23  |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.428s | 52687k     | 31257k         | 0.50 | 0.63 | 0.76 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.155s | 31k        | 13k            | 0.50 | 0.64 | 0.76 | 0.83 | 0.96 | 1.12 | 1.23  |
| Normal Distribution | HDRHistogram | 0.060s | 6k         | 47             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.111s | 36k        | 4k             | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.083s | 46k        | unavailable    | 0.50 | 0.64 | 0.75 | 0.83 | 0.97 | 1.12 | 1.26  |
| Normal Distribution | Quantogram   | NaN    | NaN        | NaN            | NaN  | NaN  | NaN  | NaN  | NaN  | NaN  | NaN   |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.098s | 12098k     | 7812k          | 0.50 | 0.63 | 0.76 | 0.83 | 0.96 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.065s | 9101k      | 13k            | 0.50 | 0.63 | 0.76 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | HDRHistogram | 0.018s | 2205k      | 45             | 0.00 | 0.00 | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.032s | 4915k      | 3k             | 0.50 | 0.64 | 0.76 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.026s | 5054k      | unavailable    | 0.50 | 0.64 | 0.75 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | Quantogram   | NaN    | NaN        | NaN            | NaN  | NaN  | NaN  | NaN  | NaN  | NaN  | NaN   |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.000s | 12k        | 7k             | 5.34 | 5.72 | 6.30 | 6.77 | 7.98 | 17.43 | 17.43 |
| Pareto Distribution | TDigest      | 0.000s | 13k        | 11k            | 5.36 | 5.73 | 6.35 | 6.88 | 8.32 | 9.45  | 9.75  |
| Pareto Distribution | HDRHistogram | 0.000s | 2k         | 51             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 13.00 | 13.00 |
| Pareto Distribution | DDSketch     | 0.000s | 1k         | 722            | 5.31 | 5.75 | 6.23 | 6.62 | 7.92 | 9.30  | 9.30  |
| Pareto Distribution | DDSketch2    | 0.000s | 1k         | unavailable    | 5.33 | 5.77 | 6.37 | 6.63 | 8.08 | 9.85  | 9.85  |
| Pareto Distribution | Quantogram   | 0.000s | 7k         | unavailable    | 5.33 | 5.77 | 6.25 | 6.63 | 7.37 | 10.06 | 10.06 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.586s | 85068k     | 39062k         | 5.36 | 5.74 | 6.29 | 6.75 | 7.92 | 9.99  | 12.59 |
| Pareto Distribution | TDigest      | 0.223s | 13k        | 12k            | 5.36 | 5.74 | 6.29 | 6.74 | 7.92 | 10.01 | 12.64 |
| Pareto Distribution | HDRHistogram | 0.139s | 2k         | 76             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.183s | 1k         | 882            | 5.31 | 5.75 | 6.23 | 6.75 | 7.92 | 10.07 | 12.55 |
| Pareto Distribution | DDSketch2    | 0.144s | 1k         | unavailable    | 5.33 | 5.77 | 6.24 | 6.76 | 7.93 | 10.04 | 12.49 |
| Pareto Distribution | Quantogram   | 0.422s | 13k        | unavailable    | 5.33 | 5.77 | 6.25 | 6.76 | 7.92 | 10.05 | 12.49 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.442s | 52687k     | 31257k         | 5.36 | 5.74 | 6.29 | 6.75 | 7.92 | 9.98  | 12.62 |
| Pareto Distribution | TDigest      | 0.179s | 31k        | 12k            | 5.36 | 5.74 | 6.29 | 6.75 | 7.93 | 10.00 | 12.62 |
| Pareto Distribution | HDRHistogram | 0.111s | 6k         | 75             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.146s | 4k         | 877            | 5.31 | 5.75 | 6.23 | 6.75 | 7.92 | 9.88  | 12.55 |
| Pareto Distribution | DDSketch2    | 0.115s | 5k         | unavailable    | 5.33 | 5.77 | 6.24 | 6.76 | 7.93 | 10.04 | 12.74 |
| Pareto Distribution | Quantogram   | NaN    | NaN        | NaN            | NaN  | NaN  | NaN  | NaN  | NaN  | NaN   | NaN   |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 75.0 | 90.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.102s | 12098k     | 7812k          | 5.36 | 5.74 | 6.30 | 6.75 | 7.92 | 9.93  | 12.53 |
| Pareto Distribution | TDigest      | 0.068s | 9101k      | 12k            | 5.36 | 5.74 | 6.29 | 6.74 | 7.92 | 10.02 | 12.54 |
| Pareto Distribution | HDRHistogram | 0.028s | 2205k      | 66             | 5.00 | 5.00 | 6.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.040s | 1363k      | 841            | 5.31 | 5.75 | 6.23 | 6.75 | 7.92 | 10.07 | 12.81 |
| Pareto Distribution | DDSketch2    | 0.032s | 823k       | unavailable    | 5.33 | 5.77 | 6.24 | 6.76 | 7.93 | 9.85  | 12.74 |
| Pareto Distribution | Quantogram   | NaN    | NaN        | NaN            | NaN  | NaN  | NaN  | NaN  | NaN  | NaN   | NaN   |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.000s | 12k        | 7k             | 20.59 | 39.14 | 69.15 | 102.24 | 213.66 | 341.40 | 341.40 |
| LogNorm Distribution | TDigest      | 0.000s | 13k        | 11k            | 19.89 | 37.96 | 69.41 | 101.11 | 203.38 | 458.16 | 527.81 |
| LogNorm Distribution | HDRHistogram | 0.000s | 4k         | 226            | 19.00 | 39.00 | 73.00 | 110.00 | 212.00 | 595.00 | 595.00 |
| LogNorm Distribution | DDSketch     | 0.000s | 4k         | 1235           | 19.89 | 39.25 | 68.72 | 100.49 | 186.82 | 820.71 | 820.71 |
| LogNorm Distribution | DDSketch2    | 0.000s | 4k         | unavailable    | 19.70 | 37.88 | 67.28 | 92.28  | 180.94 | 361.88 | 361.88 |
| LogNorm Distribution | Quantogram   | 0.000s | 31k        | unavailable    | 20.90 | 40.19 | 72.80 | 108.17 | 165.18 | 515.02 | 515.02 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.531s | 85068k     | 39062k         | 20.01 | 38.72 | 70.11 | 100.12 | 194.34 | 411.43 | 756.07 |
| LogNorm Distribution | TDigest      | 0.198s | 13k        | 12k            | 20.02 | 38.73 | 70.25 | 100.05 | 195.52 | 407.69 | 778.08 |
| LogNorm Distribution | HDRHistogram | 0.116s | 8k         | 1080           | 20.00 | 38.00 | 70.00 | 100.00 | 195.00 | 411.00 | 763.00 |
| LogNorm Distribution | DDSketch     | 0.159s | 8k         | 2k             | 19.89 | 38.48 | 70.11 | 100.49 | 194.44 | 407.54 | 757.61 |
| LogNorm Distribution | DDSketch2    | 0.131s | 4k         | unavailable    | 20.09 | 38.63 | 70.00 | 99.92  | 195.89 | 415.97 | 768.17 |
| LogNorm Distribution | Quantogram   | 0.535s | 63k        | unavailable    | 20.09 | 38.63 | 69.97 | 99.93  | 195.95 | 415.88 | 768.41 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.415s | 52687k     | 31257k         | 20.00 | 38.72 | 70.12 | 100.12 | 195.54 | 413.57 | 761.05 |
| LogNorm Distribution | TDigest      | 0.161s | 31k        | 13k            | 20.01 | 38.78 | 70.30 | 100.11 | 194.70 | 414.01 | 739.14 |
| LogNorm Distribution | HDRHistogram | 0.096s | 20k        | 1062           | 19.00 | 38.00 | 70.00 | 100.00 | 194.00 | 411.00 | 771.00 |
| LogNorm Distribution | DDSketch     | 0.132s | 13k        | 2k             | 19.89 | 38.48 | 70.11 | 100.49 | 194.44 | 407.54 | 757.61 |
| LogNorm Distribution | DDSketch2    | 0.105s | 20k        | unavailable    | 20.09 | 38.63 | 70.00 | 99.92  | 195.89 | 415.97 | 783.61 |
| LogNorm Distribution | Quantogram   | NaN    | NaN        | NaN            | NaN   | NaN   | NaN   | NaN    | NaN    | NaN    | NaN    |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|-------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.097s | 12098k     | 7812k          | 19.98 | 38.71 | 70.14 | 100.06 | 194.56 | 406.10 | 732.98 |
| LogNorm Distribution | TDigest      | 0.063s | 9101k      | 12k            | 19.99 | 38.74 | 70.20 | 100.18 | 195.58 | 410.43 | 755.12 |
| LogNorm Distribution | HDRHistogram | 0.027s | 4281k      | 878            | 20.00 | 38.00 | 70.00 | 100.00 | 195.00 | 413.00 | 763.00 |
| LogNorm Distribution | DDSketch     | 0.038s | 4360k      | 2k             | 19.89 | 38.48 | 70.11 | 100.49 | 194.44 | 415.78 | 757.61 |
| LogNorm Distribution | DDSketch2    | 0.033s | 4178k      | unavailable    | 20.09 | 38.63 | 70.00 | 99.92  | 195.89 | 407.74 | 753.06 |
| LogNorm Distribution | Quantogram   | NaN    | NaN        | NaN            | NaN   | NaN   | NaN   | NaN    | NaN    | NaN    | NaN    |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.000s | 12k        | 7k             | 20589.08 | 39142.38 | 69147.17 | 102240.64 | 213661.03 | 341397.11 | 341397.11 |
| LogNorm Distribution 1000x | TDigest      | 0.000s | 13k        | 11k            | 19885.93 | 37963.12 | 69405.38 | 101113.31 | 203383.83 | 458157.44 | 527813.52 |
| LogNorm Distribution 1000x | HDRHistogram | 0.000s | 16k        | 750            | 19455.00 | 39423.00 | 73727.00 | 110591.00 | 212991.00 | 598015.00 | 598015.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.000s | 4k         | 1236           | 20136.32 | 38960.45 | 68208.26 | 101756.13 | 185415.47 | 814560.26 | 814560.26 |
| LogNorm Distribution 1000x | DDSketch2    | 0.000s | 4k         | unavailable    | 19782.72 | 38037.52 | 67551.62 | 92659.16  | 181692.15 | 363392.69 | 363392.69 |
| LogNorm Distribution 1000x | Quantogram   | 0.000s | 30k        | unavailable    | 20986.66 | 40343.45 | 73080.61 | 106464.65 | 165184.61 | 515019.42 | 515019.42 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.535s | 85068k     | 39062k         | 20007.30 | 38721.35 | 70110.67 | 100124.28 | 194341.83 | 411425.94 | 756069.38 |
| LogNorm Distribution 1000x | TDigest      | 0.201s | 13k        | 12k            | 20016.12 | 38729.37 | 70249.25 | 100049.83 | 195518.88 | 407688.32 | 778079.90 |
| LogNorm Distribution 1000x | HDRHistogram | 0.116s | 28k        | 3k             | 20095.00 | 38911.00 | 70655.00 | 100351.00 | 195583.00 | 411647.00 | 765951.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.161s | 8k         | 2k             | 20136.32 | 38960.45 | 69586.21 | 99741.16  | 192982.67 | 412660.72 | 751931.89 |
| LogNorm Distribution 1000x | DDSketch2    | 0.127s | 4k         | unavailable    | 20176.12 | 38794.63 | 70292.52 | 100318.96 | 196688.92 | 417624.12 | 771311.53 |
| LogNorm Distribution 1000x | Quantogram   | 0.515s | 63k        | unavailable    | 20171.72 | 38776.86 | 70242.80 | 100324.02 | 196713.76 | 417508.43 | 756300.49 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.421s | 52687k     | 31257k         | 20002.92 | 38722.46 | 70118.07 | 100119.19 | 195540.07 | 413570.56 | 761053.07 |
| LogNorm Distribution 1000x | TDigest      | 0.161s | 31k        | 13k            | 20006.77 | 38775.06 | 70301.59 | 100110.42 | 194699.45 | 414006.05 | 739143.37 |
| LogNorm Distribution 1000x | HDRHistogram | 0.092s | 50k        | 2k             | 20095.00 | 38911.00 | 70655.00 | 100351.00 | 195583.00 | 411647.00 | 774143.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.128s | 13k        | 2k             | 20136.32 | 38960.45 | 69586.21 | 99741.16  | 196881.31 | 412660.72 | 751931.89 |
| LogNorm Distribution 1000x | DDSketch2    | 0.101s | 20k        | unavailable    | 20176.12 | 38794.63 | 70292.52 | 100318.96 | 196688.92 | 409372.62 | 771311.53 |
| LogNorm Distribution 1000x | Quantogram   | NaN    | NaN        | NaN            | NaN      | NaN      | NaN      | NaN       | NaN       | NaN       | NaN       |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 75.0     | 90.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|----------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.098s | 12098k     | 7812k          | 19978.14 | 38709.89 | 70137.27 | 100064.82 | 194562.76 | 406101.55 | 732980.18 |
| LogNorm Distribution 1000x | TDigest      | 0.063s | 9101k      | 12k            | 19987.39 | 38738.29 | 70204.43 | 100182.69 | 195580.09 | 410434.28 | 755119.63 |
| LogNorm Distribution 1000x | HDRHistogram | 0.038s | 18396k     | 2k             | 20095.00 | 38911.00 | 70655.00 | 100351.00 | 195583.00 | 413695.00 | 761855.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.037s | 4360k      | 2k             | 19737.58 | 38960.45 | 69586.21 | 99741.16  | 192982.67 | 412660.72 | 751931.89 |
| LogNorm Distribution 1000x | DDSketch2    | 0.033s | 4178k      | unavailable    | 20176.12 | 38794.63 | 70292.52 | 100318.96 | 196688.92 | 409372.62 | 741324.66 |
| LogNorm Distribution 1000x | Quantogram   | NaN    | NaN        | NaN            | NaN      | NaN      | NaN      | NaN       | NaN       | NaN       | NaN       |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.000s | 12k        | 7k             | 35.00 | 57.00 | 74.00 | 82.00 | 106.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | TDigest      | 0.000s | 13k        | 8k             | 34.84 | 57.00 | 74.00 | 82.17 | 105.50 | 130.00 | 136.00 |
| PM10 Air Quality Dataset | HDRHistogram | 0.000s | 2k         | 157            | 35.00 | 57.00 | 74.00 | 82.00 | 105.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.000s | 2k         | 969            | 34.82 | 57.40 | 74.45 | 82.28 | 104.60 | 125.22 | 125.22 |
| PM10 Air Quality Dataset | DDSketch2    | 0.000s | 2k         | unavailable    | 35.00 | 57.46 | 74.28 | 81.96 | 103.99 | 124.38 | 124.38 |
| PM10 Air Quality Dataset | Quantogram   | 0.000s | 16k        | unavailable    | 35.00 | 57.00 | 74.00 | 82.00 | 106.00 | 136.00 | 136.00 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.291s | 85068k     | 39062k         | 36.00 | 54.00 | 76.00 | 92.00 | 129.00 | 213.00 | 410.00 |
| PM10 Air Quality Dataset | TDigest      | 0.125s | 13k        | 10k            | 35.91 | 54.00 | 76.41 | 92.40 | 129.35 | 214.33 | 378.86 |
| PM10 Air Quality Dataset | HDRHistogram | 0.050s | 8k         | 835            | 36.00 | 54.00 | 76.00 | 92.00 | 129.00 | 213.00 | 411.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.108s | 5k         | 1957           | 36.24 | 54.06 | 75.95 | 92.77 | 127.75 | 214.89 | 407.54 |
| PM10 Air Quality Dataset | DDSketch2    | 0.062s | 5k         | unavailable    | 35.70 | 54.11 | 75.76 | 92.28 | 129.30 | 212.18 | 407.74 |
| PM10 Air Quality Dataset | Quantogram   | 0.373s | 31k        | unavailable    | 36.00 | 54.00 | 75.74 | 92.32 | 129.28 | 212.10 | 407.72 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.245s | 52687k     | 31257k         | 37.00 | 57.00 | 80.00 | 97.00 | 134.00 | 220.00 | 426.00 |
| PM10 Air Quality Dataset | TDigest      | 0.094s | 31k        | 12k            | 37.00 | 56.75 | 80.03 | 96.69 | 133.81 | 221.42 | 420.84 |
| PM10 Air Quality Dataset | HDRHistogram | 0.040s | 14k        | 827            | 37.00 | 57.00 | 80.00 | 97.00 | 134.00 | 220.00 | 427.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.089s | 12k        | 1954           | 36.97 | 57.40 | 80.65 | 96.55 | 132.97 | 219.23 | 424.18 |
| PM10 Air Quality Dataset | DDSketch2    | 0.051s | 15k        | unavailable    | 37.14 | 57.46 | 80.37 | 97.94 | 134.56 | 220.84 | 424.37 |
| PM10 Air Quality Dataset | Quantogram   | NaN    | NaN        | NaN            | NaN   | NaN   | NaN   | NaN   | NaN    | NaN    | NaN    |

COUNT=[1000, 1000, ...x1000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=2, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 75.0  | 90.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.029s | 12098k     | 7812k          | 35.00 | 57.00 | 74.00 | 82.00 | 106.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | TDigest      | 0.030s | 9101k      | 10k            | 34.92 | 57.09 | 74.00 | 82.25 | 105.48 | 131.15 | 136.00 |
| PM10 Air Quality Dataset | HDRHistogram | 0.015s | 2205k      | 314            | 35.00 | 57.00 | 74.00 | 82.00 | 105.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.021s | 2363k      | 1251           | 34.82 | 57.40 | 74.45 | 82.28 | 104.60 | 125.22 | 135.65 |
| PM10 Air Quality Dataset | DDSketch2    | 0.015s | 2316k      | unavailable    | 35.00 | 57.46 | 74.28 | 81.96 | 103.99 | 124.38 | 137.26 |
| PM10 Air Quality Dataset | Quantogram   | NaN    | NaN        | NaN            | NaN   | NaN   | NaN   | NaN   | NaN    | NaN    | NaN    |
