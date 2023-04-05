# Quantile Compare

Compare different quantile algorithms in rust in terms of performance, memory usage and accuracy.

Test with different distributions and a real world data set based on air quality.

### Run Suite
`cargo run --release`

### Comments

- AllValues: Naive and Exact solution by storing all values in a sorted array.
- TDigest: Fork of `https://github.com/MnO2/t-digest`. Fixing the most severe performance issues, but there's still a lot of headroom.
- HDRHistogram: Supports only u64 values.
- DDSketch: Fork of https://crates.io/crates/sketches-ddsketch. Added a simple serialization via serde.
- DDSketch2: Fork of https://crates.io/crates/sketches-rust. Fixed some issues to make it usable. Pretty new crate, when testing. Some parts maybe not be finalized.

#### Serialization
Only HDRHistogram has a specialized implementation. There's a lot of headroom for the other crates.

# Results

COUNT=[1_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.000s | 12k        | 7k             | 0.51 | 0.83 | 0.98 | 1.08 | 1.08  |
| Normal Distribution | TDigest      | 0.000s | 12k        | 4k             | 0.50 | 0.83 | 0.97 | 1.14 | 1.17  |
| Normal Distribution | HDRHistogram | 0.000s | 16k        | 43             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.000s | 6k         | 5k             | 0.50 | 0.83 | 0.95 | 1.26 | 1.26  |
| Normal Distribution | DDSketch2    | 0.000s | 6k         | unavailable    | 0.50 | 0.81 | 0.95 | 1.09 | 1.09  |

COUNT=[1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.092s | 12099k     | 7812k          | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.033s | 17k        | 4k             | 0.50 | 0.83 | 0.96 | 1.12 | 1.24  |
| Normal Distribution | HDRHistogram | 0.017s | 16k        | 45             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.028s | 16k        | 10k            | 0.50 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.020s | 14k        | unavailable    | 0.50 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[5_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.514s | 85068k     | 39062k         | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.167s | 17k        | 4k             | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | HDRHistogram | 0.085s | 16k        | 47             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.142s | 16k        | 12k            | 0.50 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.106s | 13k        | unavailable    | 0.50 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[10, 100, 150, 1_000, 3_000_000, 1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9 | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|------|-------|
| Normal Distribution | AllValues    | 0.417s | 64027k     | 31259k         | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | TDigest      | 0.133s | 70k        | 4k             | 0.50 | 0.83 | 0.97 | 1.12 | 1.24  |
| Normal Distribution | HDRHistogram | 0.069s | 96k        | 47             | 0.00 | 0.00 | 0.00 | 1.00 | 1.00  |
| Normal Distribution | DDSketch     | 0.119s | 41k        | 11k            | 0.50 | 0.83 | 0.97 | 1.12 | 1.23  |
| Normal Distribution | DDSketch2    | 0.086s | 51k        | unavailable    | 0.50 | 0.83 | 0.97 | 1.12 | 1.26  |

COUNT=[1_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.000s | 12k        | 7k             | 5.34 | 6.77 | 7.98 | 17.43 | 17.43 |
| Pareto Distribution | TDigest      | 0.000s | 12k        | 4k             | 5.36 | 6.88 | 8.32 | 9.45  | 9.75  |
| Pareto Distribution | HDRHistogram | 0.000s | 16k        | 51             | 5.00 | 6.00 | 7.00 | 13.00 | 13.00 |
| Pareto Distribution | DDSketch     | 0.000s | 1k         | 1162           | 5.31 | 6.62 | 7.92 | 9.30  | 9.30  |
| Pareto Distribution | DDSketch2    | 0.000s | 0k         | unavailable    | 5.33 | 6.63 | 8.08 | 9.85  | 9.85  |

COUNT=[1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.109s | 12099k     | 7812k          | 5.36 | 6.75 | 7.92 | 9.99  | 12.66 |
| Pareto Distribution | TDigest      | 0.050s | 17k        | 4k             | 5.36 | 6.75 | 7.93 | 10.00 | 12.35 |
| Pareto Distribution | HDRHistogram | 0.030s | 16k        | 67             | 5.00 | 6.00 | 7.00 | 10.00 | 12.00 |
| Pareto Distribution | DDSketch     | 0.043s | 1k         | 1162           | 5.31 | 6.75 | 7.92 | 10.07 | 12.55 |
| Pareto Distribution | DDSketch2    | 0.030s | 1k         | unavailable    | 5.33 | 6.76 | 7.93 | 10.04 | 12.74 |

COUNT=[5_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.600s | 85068k     | 39062k         | 5.36 | 6.75 | 7.92 | 9.97  | 12.55 |
| Pareto Distribution | TDigest      | 0.209s | 17k        | 4k             | 5.36 | 6.75 | 7.92 | 10.04 | 12.58 |
| Pareto Distribution | HDRHistogram | 0.137s | 16k        | 77             | 5.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.208s | 1k         | 1162           | 5.31 | 6.75 | 7.92 | 9.88  | 12.55 |
| Pareto Distribution | DDSketch2    | 0.152s | 1k         | unavailable    | 5.33 | 6.76 | 7.93 | 10.04 | 12.49 |

COUNT=[10, 100, 150, 1_000, 3_000_000, 1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution        | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0 | 95.0 | 99.0 | 99.9  | 99.99 |
|---------------------|--------------|--------|------------|----------------|------|------|------|-------|-------|
| Pareto Distribution | AllValues    | 0.435s | 64027k     | 31259k         | 5.36 | 6.75 | 7.92 | 9.98  | 12.62 |
| Pareto Distribution | TDigest      | 0.163s | 70k        | 4k             | 5.36 | 6.74 | 7.91 | 10.08 | 12.52 |
| Pareto Distribution | HDRHistogram | 0.109s | 96k        | 75             | 5.00 | 6.00 | 7.00 | 9.00  | 12.00 |
| Pareto Distribution | DDSketch     | 0.161s | 7k         | 1162           | 5.31 | 6.75 | 7.92 | 9.88  | 12.55 |
| Pareto Distribution | DDSketch2    | 0.113s | 6k         | unavailable    | 5.33 | 6.76 | 7.93 | 10.04 | 12.74 |

COUNT=[1_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.000s | 12k        | 7k             | 20.59 | 102.24 | 213.66 | 341.40 | 341.40 |
| LogNorm Distribution | TDigest      | 0.000s | 12k        | 4k             | 19.86 | 101.11 | 203.38 | 458.16 | 527.81 |
| LogNorm Distribution | HDRHistogram | 0.000s | 16k        | 226            | 19.00 | 110.00 | 212.00 | 595.00 | 595.00 |
| LogNorm Distribution | DDSketch     | 0.000s | 4k         | 3k             | 19.89 | 100.49 | 186.82 | 820.71 | 820.71 |
| LogNorm Distribution | DDSketch2    | 0.000s | 4k         | unavailable    | 19.70 | 92.28  | 180.94 | 361.88 | 361.88 |

COUNT=[1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.093s | 12099k     | 7812k          | 19.99 | 100.23 | 195.43 | 406.74 | 756.09 |
| LogNorm Distribution | TDigest      | 0.036s | 17k        | 4k             | 20.01 | 99.90  | 193.37 | 411.98 | 769.91 |
| LogNorm Distribution | HDRHistogram | 0.021s | 32k        | 1232           | 20.00 | 99.00  | 194.00 | 415.00 | 759.00 |
| LogNorm Distribution | DDSketch     | 0.032s | 4k         | 4k             | 19.89 | 100.49 | 194.44 | 415.78 | 727.90 |
| LogNorm Distribution | DDSketch2    | 0.024s | 8k         | unavailable    | 20.09 | 99.92  | 195.89 | 407.74 | 768.17 |

COUNT=[5_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.522s | 85068k     | 39062k         | 20.03 | 100.18 | 195.39 | 411.56 | 760.84 |
| LogNorm Distribution | TDigest      | 0.188s | 17k        | 4k             | 19.97 | 100.06 | 195.42 | 410.33 | 731.52 |
| LogNorm Distribution | HDRHistogram | 0.103s | 32k        | 1736           | 20.00 | 100.00 | 194.00 | 410.00 | 757.00 |
| LogNorm Distribution | DDSketch     | 0.158s | 4k         | 4k             | 19.89 | 100.49 | 194.44 | 415.78 | 772.92 |
| LogNorm Distribution | DDSketch2    | 0.119s | 4k         | unavailable    | 20.09 | 99.92  | 195.89 | 415.97 | 768.17 |

COUNT=[10, 100, 150, 1_000, 3_000_000, 1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution         | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|----------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| LogNorm Distribution | AllValues    | 0.423s | 64027k     | 31259k         | 20.00 | 100.12 | 195.54 | 413.54 | 761.05 |
| LogNorm Distribution | TDigest      | 0.143s | 70k        | 4k             | 20.02 | 100.75 | 195.48 | 409.68 | 740.67 |
| LogNorm Distribution | HDRHistogram | 0.079s | 128k       | 1663           | 19.00 | 100.00 | 194.00 | 411.00 | 771.00 |
| LogNorm Distribution | DDSketch     | 0.128s | 20k        | 4k             | 19.89 | 100.49 | 194.44 | 407.54 | 757.61 |
| LogNorm Distribution | DDSketch2    | 0.099s | 26k        | unavailable    | 20.09 | 99.92  | 195.89 | 415.97 | 783.61 |

COUNT=[1_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.000s | 12k        | 7k             | 20589.08 | 102240.64 | 213661.03 | 341397.11 | 341397.11 |
| LogNorm Distribution 1000x | TDigest      | 0.000s | 12k        | 4k             | 19859.99 | 101113.31 | 203383.83 | 458157.44 | 527813.52 |
| LogNorm Distribution 1000x | HDRHistogram | 0.000s | 160k       | 1705           | 19343.00 | 110207.00 | 212735.00 | 595455.00 | 595455.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.000s | 4k         | 3k             | 20136.32 | 101756.13 | 185415.47 | 814560.26 | 814560.26 |
| LogNorm Distribution 1000x | DDSketch2    | 0.000s | 4k         | unavailable    | 19782.72 | 92659.16  | 181692.15 | 363392.69 | 363392.69 |

COUNT=[1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.099s | 12099k     | 7812k          | 19987.59 | 100233.58 | 195429.27 | 406742.90 | 756094.39 |
| LogNorm Distribution 1000x | TDigest      | 0.038s | 17k        | 4k             | 20008.39 | 99898.85  | 193370.20 | 411978.49 | 769912.91 |
| LogNorm Distribution 1000x | HDRHistogram | 0.022s | 192k       | 14k            | 20015.00 | 100031.00 | 194047.00 | 415487.00 | 760319.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.032s | 4k         | 4k             | 20136.32 | 99741.16  | 192982.67 | 412660.72 | 722447.25 |
| LogNorm Distribution 1000x | DDSketch2    | 0.024s | 8k         | unavailable    | 20176.12 | 100318.96 | 196688.92 | 409372.62 | 756158.34 |

COUNT=[5_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.532s | 85068k     | 39062k         | 20026.04 | 100180.66 | 195393.01 | 411557.29 | 760835.86 |
| LogNorm Distribution 1000x | TDigest      | 0.179s | 17k        | 4k             | 19966.21 | 100064.94 | 195416.79 | 410333.57 | 731517.97 |
| LogNorm Distribution 1000x | HDRHistogram | 0.119s | 128k       | 17k            | 20015.00 | 100095.00 | 194943.00 | 410111.00 | 757759.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.159s | 4k         | 4k             | 20136.32 | 99741.16  | 196881.31 | 420997.30 | 767122.43 |
| LogNorm Distribution 1000x | DDSketch2    | 0.119s | 4k         | unavailable    | 20176.12 | 100318.96 | 196688.92 | 417624.12 | 756158.34 |

COUNT=[10, 100, 150, 1_000, 3_000_000, 1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution               | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0     | 95.0      | 99.0      | 99.9      | 99.99     |
|----------------------------|--------------|--------|------------|----------------|----------|-----------|-----------|-----------|-----------|
| LogNorm Distribution 1000x | AllValues    | 0.411s | 64027k     | 31259k         | 20003.07 | 100119.73 | 195537.86 | 413539.30 | 761053.07 |
| LogNorm Distribution 1000x | TDigest      | 0.145s | 70k        | 4k             | 20016.07 | 100752.35 | 195478.39 | 409675.16 | 740674.08 |
| LogNorm Distribution 1000x | HDRHistogram | 0.080s | 688k       | 17k            | 19999.00 | 100095.00 | 195071.00 | 411647.00 | 771583.00 |
| LogNorm Distribution 1000x | DDSketch     | 0.124s | 20k        | 4k             | 20136.32 | 99741.16  | 196881.31 | 412660.72 | 751931.89 |
| LogNorm Distribution 1000x | DDSketch2    | 0.089s | 26k        | unavailable    | 20176.12 | 100318.96 | 196688.92 | 409372.62 | 771311.53 |

COUNT=[1_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.000s | 12k        | 7k             | 35.00 | 82.00 | 106.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | TDigest      | 0.000s | 12k        | 4k             | 34.82 | 82.17 | 105.50 | 130.00 | 136.00 |
| PM10 Air Quality Dataset | HDRHistogram | 0.000s | 16k        | 157            | 35.00 | 82.00 | 105.00 | 136.00 | 136.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.000s | 2k         | 2k             | 34.82 | 82.28 | 104.60 | 125.22 | 125.22 |
| PM10 Air Quality Dataset | DDSketch2    | 0.000s | 2k         | unavailable    | 35.00 | 81.96 | 103.99 | 124.38 | 124.38 |

COUNT=[1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0   | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|--------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.047s | 12099k     | 7812k          | 40.00 | 101.00 | 139.00 | 227.00 | 446.00 |
| PM10 Air Quality Dataset | TDigest      | 0.020s | 17k        | 4k             | 39.92 | 100.87 | 138.86 | 229.33 | 441.45 |
| PM10 Air Quality Dataset | HDRHistogram | 0.008s | 16k        | 837            | 40.00 | 101.00 | 139.00 | 227.00 | 446.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.023s | 4k         | 3k             | 40.05 | 100.49 | 138.40 | 228.18 | 450.41 |
| PM10 Air Quality Dataset | DDSketch2    | 0.013s | 4k         | unavailable    | 40.18 | 101.93 | 140.01 | 225.29 | 441.69 |

COUNT=[5_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.276s | 85068k     | 39062k         | 36.00 | 92.00 | 129.00 | 213.00 | 410.00 |
| PM10 Air Quality Dataset | TDigest      | 0.096s | 17k        | 4k             | 35.91 | 92.43 | 129.28 | 213.35 | 399.02 |
| PM10 Air Quality Dataset | HDRHistogram | 0.041s | 16k        | 1003           | 36.00 | 92.00 | 129.00 | 213.00 | 410.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.116s | 5k         | 4k             | 36.24 | 92.77 | 127.75 | 214.89 | 407.54 |
| PM10 Air Quality Dataset | DDSketch2    | 0.062s | 4k         | unavailable    | 35.70 | 92.28 | 129.30 | 212.18 | 407.74 |

COUNT=[10, 100, 150, 1_000, 3_000_000, 1_000_000], GK_ERROR_CONFIG=0.001, TDIGEST_BATCH=1_000, TDIGEST_MAX_SIZE=300, HDR_SIGFIG=3
| Distribution             | Algorithm    | Time   | PeakMemory | SerializedSize | 50.0  | 95.0  | 99.0   | 99.9   | 99.99  |
|--------------------------|--------------|--------|------------|----------------|-------|-------|--------|--------|--------|
| PM10 Air Quality Dataset | AllValues    | 0.223s | 64027k     | 31259k         | 37.00 | 97.00 | 134.00 | 220.00 | 426.00 |
| PM10 Air Quality Dataset | TDigest      | 0.079s | 70k        | 4k             | 37.00 | 96.73 | 134.59 | 220.73 | 433.94 |
| PM10 Air Quality Dataset | HDRHistogram | 0.033s | 96k        | 996            | 37.00 | 97.00 | 134.00 | 220.00 | 426.00 |
| PM10 Air Quality Dataset | DDSketch     | 0.092s | 17k        | 4k             | 36.97 | 96.55 | 132.97 | 219.23 | 424.18 |
| PM10 Air Quality Dataset | DDSketch2    | 0.046s | 20k        | unavailable    | 37.14 | 97.94 | 134.56 | 220.84 | 424.37 |
