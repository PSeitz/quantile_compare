# Quantile Compare

Compare different quantile algorithms in rust in terms of performance, memory usage and accuracy.

Test with different distributions and a real world data set based on air quality.

### Run Suite
`cargo run --release`

### Comments

- AllValues: Naive and Exact solution by storing all values in a sorted array.
- TDigest: Fork of https://github.com/MnO2/t-digest. Fixing the most severe performance issues, but there's still a lot of headroom.
- HDRHistogram: Supports only u64 values, and is not viable for some use cases.
- DDSketch: Fork of https://crates.io/crates/sketches-ddsketch. Added a simple serialization via serde.
- DDSketch2: Fork of https://crates.io/crates/sketches-rust. Fixed some issues to make it usable. Pretty new crate, when testing. Some parts maybe not be finalized.

#### Serialization
Only HDRHistogram has a specialized implementation. There's a lot of headroom for the other crates.

#### Counts
If there are multiple counts, that means they are collected and then merged.

# Results


COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.000s | 12k        | 7k             | 0.51 | 0.83 | 0.98 | 1.08 | 1.08  |
| Normal Distribution | TDigest      | 0.000s | 13k        | 4k             | 0.50 | 0.83 | 0.97 | 1.14 | 1.17  |
| Normal Distribution | HDRHistogram | 0.000s | 16k        | 43             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.000s | 6k         | 5k             | 0.50 | 0.83 | 0.95 | 1.26 | 1.26  |
| Normal Distribution | DDSketch2    | 0.000s | 6k         | unavailable    | 0.50 | 0.81 | 0.95 | 1.09 | 1.09  |

COUNT=[1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.098s | 12099k     | 7812k          | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.036s | 13k        | 4k             | 0.50 | 0.83 | 0.96 | 1.12 | 1.24  |
| Normal Distribution | HDRHistogram | 0.016s | 16k        | 45             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.028s | 16k        | 10k            | 0.50 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.020s | 14k        | unavailable    | 0.50 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.489s | 85068k     | 39062k         | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.168s | 13k        | 4k             | 0.50 | 0.83 | 0.97 | 1.11 | 1.24  |
| Normal Distribution | HDRHistogram | 0.080s | 16k        | 47             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.138s | 16k        | 12k            | 0.50 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.101s | 13k        | unavailable    | 0.50 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.391s | 64025k     | 31257k         | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.135s | 30k        | 4k             | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | HDRHistogram | 0.064s | 48k        | 47             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.110s | 32k        | 11k            | 0.50 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.082s | 43k        | unavailable    | 0.50 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.000s | 12k        | 7k             | 5.34 | 6.77 | 7.98 | 17.43 | 17.43 |
| Pareto Distribution | TDigest      | 0.000s | 13k        | 4k             | 5.36 | 6.88 | 8.32 | 9.45  | 9.75  |
| Pareto Distribution | HDRHistogram | 0.000s | 16k        | 51             | 5.00 | 6.00 | 7.00 | 13.00 | 13.00 |
| Pareto Distribution | DDSketch     | 0.000s | 1k         | 1162           | 5.31 | 6.62 | 7.92 | 9.30  | 9.30  |
| Pareto Distribution | DDSketch2    | 0.000s | 0k         | unavailable    | 5.33 | 6.63 | 8.08 | 9.85  | 9.85  |

COUNT=[1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.095s | 12099k     | 7812k          | 5.36 | 6.75 | 7.92 | 9.99  | 12.66 |
| Pareto Distribution | TDigest      | 0.042s | 13k        | 4k             | 5.36 | 6.75 | 7.92 | 9.93  | 12.46 |
| Pareto Distribution | HDRHistogram | 0.027s | 16k        | 67             | 5.00 | 6.00 | 7.00 | 10.00 | 12.00 |
| Pareto Distribution | DDSketch     | 0.039s | 1k         | 1162           | 5.31 | 6.75 | 7.92 | 10.07 | 12.55 |
| Pareto Distribution | DDSketch2    | 0.033s | 1k         | unavailable    | 5.33 | 6.76 | 7.93 | 10.04 | 12.74 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.532s | 85068k     | 39062k         | 5.36 | 6.75 | 7.92 | 9.97  | 12.55 |
| Pareto Distribution | TDigest      | 0.213s | 13k        | 4k             | 5.36 | 6.75 | 7.92 | 10.04 | 12.61 |
| Pareto Distribution | HDRHistogram | 0.134s | 16k        | 77             | 5.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.198s | 1k         | 1162           | 5.31 | 6.75 | 7.92 | 9.88  | 12.55 |
| Pareto Distribution | DDSketch2    | 0.173s | 1k         | unavailable    | 5.33 | 6.76 | 7.93 | 10.04 | 12.49 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.422s | 64025k     | 31257k         | 5.36 | 6.75 | 7.92 | 9.98  | 12.62 |
| Pareto Distribution | TDigest      | 0.171s | 30k        | 4k             | 5.36 | 6.75 | 7.91 | 9.98  | 12.29 |
| Pareto Distribution | HDRHistogram | 0.107s | 48k        | 75             | 5.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.158s | 3k         | 1162           | 5.31 | 6.75 | 7.92 | 9.88  | 12.55 |
| Pareto Distribution | DDSketch2    | 0.115s | 4k         | unavailable    | 5.33 | 6.76 | 7.93 | 10.04 | 12.74 |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.000s | 12k        | 7k             | 20.59 | 102.24 | 213.66 | 341.40 | 341.40 |
| LogNorm Distribution | TDigest      | 0.000s | 13k        | 4k             | 19.89 | 101.11 | 203.38 | 458.16 | 527.81 |
| LogNorm Distribution | HDRHistogram | 0.000s | 16k        | 226            | 19.00 | 110.00 | 212.00 | 595.00 | 595.00 |
| LogNorm Distribution | DDSketch     | 0.000s | 4k         | 3k             | 19.89 | 100.49 | 186.82 | 820.71 | 820.71 |
| LogNorm Distribution | DDSketch2    | 0.000s | 4k         | unavailable    | 19.70 | 92.28  | 180.94 | 361.88 | 361.88 |

COUNT=[1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.091s | 12099k     | 7812k          | 19.99 | 100.23 | 195.43 | 406.74 | 756.09 |
| LogNorm Distribution | TDigest      | 0.038s | 13k        | 4k             | 20.02 | 100.18 | 193.34 | 413.25 | 744.75 |
| LogNorm Distribution | HDRHistogram | 0.026s | 32k        | 1232           | 20.00 | 99.00  | 194.00 | 415.00 | 759.00 |
| LogNorm Distribution | DDSketch     | 0.032s | 4k         | 4k             | 19.89 | 100.49 | 194.44 | 415.78 | 727.90 |
| LogNorm Distribution | DDSketch2    | 0.023s | 8k         | unavailable    | 20.09 | 99.92  | 195.89 | 407.74 | 768.17 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.509s | 85068k     | 39062k         | 20.03 | 100.18 | 195.39 | 411.56 | 760.84 |
| LogNorm Distribution | TDigest      | 0.190s | 13k        | 4k             | 20.00 | 100.15 | 194.97 | 407.02 | 749.15 |
| LogNorm Distribution | HDRHistogram | 0.108s | 32k        | 1736           | 20.00 | 100.00 | 194.00 | 410.00 | 757.00 |
| LogNorm Distribution | DDSketch     | 0.150s | 4k         | 4k             | 19.89 | 100.49 | 194.44 | 415.78 | 772.92 |
| LogNorm Distribution | DDSketch2    | 0.112s | 4k         | unavailable    | 20.09 | 99.92  | 195.89 | 415.97 | 768.17 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.402s | 64025k     | 31257k         | 20.00 | 100.12 | 195.54 | 413.57 | 761.05 |
| LogNorm Distribution | TDigest      | 0.152s | 30k        | 4k             | 20.02 | 100.08 | 195.91 | 413.89 | 743.61 |
| LogNorm Distribution | HDRHistogram | 0.081s | 80k        | 1663           | 19.00 | 100.00 | 194.00 | 411.00 | 771.00 |
| LogNorm Distribution | DDSketch     | 0.122s | 12k        | 4k             | 19.89 | 100.49 | 194.44 | 407.54 | 757.61 |
| LogNorm Distribution | DDSketch2    | 0.090s | 20k        | unavailable    | 20.09 | 99.92  | 195.89 | 415.97 | 783.61 |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.000s | 12k        | 7k             | 20589.08 | 102240.64 | 213661.03 | 341397.11 | 341397.11 |
| LogNorm Distribution 1000x | TDigest      | 0.000s | 13k        | 4k             | 19885.93 | 101113.31 | 203383.83 | 458157.44 | 527813.52 |
| LogNorm Distribution 1000x | HDRHistogram | 0.000s | 160k       | 1705           | 19343.00 | 110207.00 | 212735.00 | 595455.00 | 595455.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.000s | 4k         | 3k             | 20136.32 | 101756.13 | 185415.47 | 814560.26 | 814560.26 |
| LogNorm Distribution 1000x | DDSketch2    | 0.000s | 4k         | unavailable    | 19782.72 | 92659.16  | 181692.15 | 363392.69 | 363392.69 |

COUNT=[1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.092s | 12099k     | 7812k          | 19987.59 | 100233.58 | 195429.27 | 406742.90 | 756094.39 |
| LogNorm Distribution 1000x | TDigest      | 0.039s | 13k        | 4k             | 20015.22 | 100182.47 | 193339.03 | 413251.05 | 744747.28 |
| LogNorm Distribution 1000x | HDRHistogram | 0.026s | 192k       | 14k            | 20015.00 | 100031.00 | 194047.00 | 415487.00 | 760319.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.030s | 4k         | 4k             | 20136.32 | 99741.16  | 192982.67 | 412660.72 | 722447.25 |
| LogNorm Distribution 1000x | DDSketch2    | 0.022s | 8k         | unavailable    | 20176.12 | 100318.96 | 196688.92 | 409372.62 | 756158.34 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.511s | 85068k     | 39062k         | 20026.04 | 100180.66 | 195393.01 | 411557.29 | 760835.86 |
| LogNorm Distribution 1000x | TDigest      | 0.193s | 13k        | 4k             | 20002.65 | 100148.87 | 194974.49 | 407016.99 | 749146.21 |
| LogNorm Distribution 1000x | HDRHistogram | 0.125s | 128k       | 17k            | 20015.00 | 100095.00 | 194943.00 | 410111.00 | 757759.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.152s | 4k         | 4k             | 20136.32 | 99741.16  | 196881.31 | 420997.30 | 767122.43 |
| LogNorm Distribution 1000x | DDSketch2    | 0.112s | 4k         | unavailable    | 20176.12 | 100318.96 | 196688.92 | 417624.12 | 756158.34 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.403s | 64025k     | 31257k         | 20003.12 | 100121.88 | 195539.50 | 413570.56 | 761053.07 |
| LogNorm Distribution 1000x | TDigest      | 0.154s | 30k        | 4k             | 20020.97 | 100077.87 | 195910.09 | 413889.83 | 743611.91 |
| LogNorm Distribution 1000x | HDRHistogram | 0.077s | 368k       | 17k            | 19999.00 | 100095.00 | 195071.00 | 411647.00 | 771583.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.124s | 12k        | 4k             | 20136.32 | 99741.16  | 196881.31 | 412660.72 | 751931.89 |
| LogNorm Distribution 1000x | DDSketch2    | 0.091s | 20k        | unavailable    | 20176.12 | 100318.96 | 196688.92 | 409372.62 | 771311.53 |

COUNT=[1_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.000s | 12k        | 7k             | 35.00 | 82.00 | 106.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | TDigest      | 0.000s | 13k        | 4k             | 34.84 | 82.17 | 105.50 | 130.00 | 136.00 |
| PM10 Air Quality Dataset | HDRHistogram | 0.000s | 16k        | 157            | 35.00 | 82.00 | 105.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.000s | 2k         | 2k             | 34.82 | 82.28 | 104.60 | 125.22 | 125.22 |
| PM10 Air Quality Dataset | DDSketch2    | 0.000s | 2k         | unavailable    | 35.00 | 81.96 | 103.99 | 124.38 | 124.38 |

COUNT=[1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.046s | 12099k     | 7812k          | 40.00 | 101.00 | 139.00 | 227.00 | 446.00 |
| PM10 Air Quality Dataset | TDigest      | 0.024s | 13k        | 4k             | 39.93 | 100.93 | 139.04 | 231.91 | 450.48 |
| PM10 Air Quality Dataset | HDRHistogram | 0.008s | 16k        | 837            | 40.00 | 101.00 | 139.00 | 227.00 | 446.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.023s | 4k         | 3k             | 40.05 | 100.49 | 138.40 | 228.18 | 450.41 |
| PM10 Air Quality Dataset | DDSketch2    | 0.012s | 4k         | unavailable    | 40.18 | 101.93 | 140.01 | 225.29 | 441.69 |

COUNT=[5_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.273s | 85068k     | 39062k         | 36.00 | 92.00 | 129.00 | 213.00 | 410.00 |
| PM10 Air Quality Dataset | TDigest      | 0.115s | 13k        | 4k             | 35.91 | 92.40 | 129.35 | 214.33 | 378.86 |
| PM10 Air Quality Dataset | HDRHistogram | 0.041s | 16k        | 1003           | 36.00 | 92.00 | 129.00 | 213.00 | 410.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.116s | 5k         | 4k             | 36.24 | 92.77 | 127.75 | 214.89 | 407.54 |
| PM10 Air Quality Dataset | DDSketch2    | 0.058s | 4k         | unavailable    | 35.70 | 92.28 | 129.30 | 212.18 | 407.74 |

COUNT=[1_000, 3_000_000, 1_000_000], TDIGEST_BATCH=500, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3, DDSketch2Err=0.01
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.224s | 64025k     | 31257k         | 37.00 | 97.00 | 134.00 | 220.00 | 426.00 |
| PM10 Air Quality Dataset | TDigest      | 0.094s | 30k        | 4k             | 37.00 | 96.69 | 133.81 | 221.42 | 420.84 |
| PM10 Air Quality Dataset | HDRHistogram | 0.033s | 48k        | 996            | 37.00 | 97.00 | 134.00 | 220.00 | 426.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.092s | 11k        | 4k             | 36.97 | 96.55 | 132.97 | 219.23 | 424.18 |
| PM10 Air Quality Dataset | DDSketch2    | 0.047s | 15k        | unavailable    | 37.14 | 97.94 | 134.56 | 220.84 | 424.37 |
