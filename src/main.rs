use std::io::{BufRead, BufReader};

use hdrhistogram::{
    serialization::{Serializer, V2Serializer},
    Histogram,
};
use prettytable::{row, Cell, Row, Table};
use rand::{rngs::StdRng, SeedableRng};
use rand_distr::Distribution;
use zw_fast_quantile::UnboundEpsilonSummary;

use peakmem_alloc::PeakAlloc;

// INSTRUMENTED_SYSTEM is an instrumented instance of the system allocator
#[global_allocator]
static GLOBAL: &PeakAlloc<std::alloc::System> = &peakmem_alloc::INSTRUMENTED_SYSTEM;

fn main() {
    test_counts();
    //test_gk_and_cksm_params();
    //test_digest_params();
    //test_sketch_params();
}

fn get_distributions() -> Vec<(&'static str, Box<dyn FnMut(usize) -> f64>)> {
    let dn = rand_distr::Normal::new(0.5f64, 0.2f64).unwrap();
    let dp = rand_distr::Pareto::new(5f64, 10f64).unwrap();
    let lg_norm = rand_distr::LogNormal::new(2.996f64, 0.979f64).unwrap();
    let reader = BufReader::new(std::fs::File::open("PM10").unwrap());

    let mut pm10_data = Vec::new();
    for line in reader.lines() {
        if let Ok(val) = line.unwrap().parse::<f64>() {
            pm10_data.push(val);
        }
    }

    let mut rng1 = StdRng::from_seed([1u8; 32]);
    let mut rng2 = StdRng::from_seed([1u8; 32]);
    let mut rng3 = StdRng::from_seed([1u8; 32]);
    let mut rng4 = StdRng::from_seed([1u8; 32]);
    let distributions: Vec<(&str, Box<dyn FnMut(usize) -> f64>)> = vec![
        (
            "Normal Distribution",
            Box::new(move |_| dn.sample(&mut rng1)),
        ),
        (
            "Pareto Distribution",
            Box::new(move |_| dp.sample(&mut rng2)),
        ),
        (
            "LogNorm Distribution",
            Box::new(move |_| lg_norm.sample(&mut rng3)),
        ),
        (
            "LogNorm Distribution 1000x",
            Box::new(move |_| lg_norm.sample(&mut rng4) * 1000.0),
        ),
        (
            "PM10 Air Quality Dataset",
            Box::new(move |index| pm10_data[index % pm10_data.len()]),
        ),
    ];
    distributions
}

#[allow(dead_code)]
fn test_counts() {
    //let counts = vec![10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];
    let counts = vec![
        vec![1_000],
        vec![1_000_000],
        vec![5_000_000],
        vec![10, 100, 150, 1_000, 3_000_000, 1_000_000],
    ];
    //let ckms_error = 0.0001;
    let gk_error = 0.001;
    //let zw_error = 0.001;
    let hdr_sigfig = 3;
    let tdigest_batch = 1_000;
    let tdigest_max_size = 300;

    let mut distributions = get_distributions();

    for (distr, distribution) in &mut distributions {
        for count_group in counts.iter().cloned() {
            let all = || AllValues::new();
            //let mut aq = QuantilesCKMS::new(ckms_error);
            //let mut ag = QuantilesGK::new(gk_error);
            let td = || TDigest::new(tdigest_batch, tdigest_max_size);
            //let mut zw = ZWQuantile::new(zw_error);
            let hdr = || HDRHistogram::new(hdr_sigfig);
            let dd = || DDSketch::new();
            let dd2 = || DDSketch2::unbounded(0.01);

            println!(
                "\nCOUNT=[{}], GK_ERROR_CONFIG={}, TDIGEST_BATCH={}, TDIGEST_MAX_SIZE={}, HDR_SIGFIG={}",
                pretty_print_count(&count_group),
                gk_error,
                tdigest_batch.separate_with_underscores(),
                tdigest_max_size,
                hdr_sigfig
            );
            //println!("    NORMAL DISTRIBITION");
            let mut table = Table::new();
            table.set_titles(row![
                "Distribution",
                "Algorithm",
                "Time",
                "PeakMemory",
                "SerializedSize",
                "50.0",
                "95.0",
                "99.0",
                "99.9",
                "99.99"
            ]);
            test(&count_group, all, distribution, table.add_row(row![distr]));
            //test(count, &mut ag, distribution, table.add_row(row![distr]));
            //if count < 50_000_000 {
            //test(count, &mut aq, distribution, table.add_row(row![distr]));
            //}
            test(&count_group, td, distribution, table.add_row(row![distr]));
            //test(count, &mut zw, distribution, table.add_row(row![distr]));
            test(&count_group, hdr, distribution, table.add_row(row![distr]));

            test(&count_group, dd, distribution, table.add_row(row![distr]));

            test(&count_group, dd2, distribution, table.add_row(row![distr]));

            table.printstd();
        }
    }
}

fn pretty_print_count(count_group: &[usize]) -> String {
    count_group
        .iter()
        .map(|el| el.separate_with_underscores())
        .collect::<Vec<_>>()
        .join(", ")
}

#[allow(dead_code)]
fn test_sketch_params() {
    //let counts = vec![10_000, 100_000, 1_000_000, 10_000_000, 100_000_000];
    let counts = vec![
        vec![1_000],
        vec![1_000_000],
        vec![3_000_000],
        vec![10, 100, 150, 1_000, 3_000_000, 1_000_000],
    ];

    let mut distributions = get_distributions();

    for (distr, distribution) in &mut distributions {
        for count_group in counts.iter().cloned() {
            let mut table = Table::new();
            let count_str = pretty_print_count(&count_group);

            println!("\nCOUNT=[{}]", count_str);

            table.set_titles(row![
                "Distribution",
                "Algorithm",
                "Time",
                "PeakMemory",
                "SerializedSize",
                "50.0",
                "95.0",
                "99.0",
                "99.9",
                "99.99"
            ]);

            let all = || AllValues::new();

            test(&count_group, all, distribution, table.add_row(row![distr]));

            let err = vec![0.01, 0.02, 0.03, 0.05, 0.10, 0.2];

            for err_val in err.iter() {
                let dd2 = || DDSketch2::unbounded(*err_val);

                test(
                    &count_group,
                    dd2,
                    distribution,
                    table.add_row(row![format!("ErrRate {}", err_val)]),
                );
            }

            //use itertools::Itertools;
            //for (err_val, num_bucket_val) in err.iter().cartesian_product(num_buckets.iter()) {
            //let mut dd2 = DDSketch2::new(*err_val, *num_bucket_val);

            //test(
            //&count_group,
            //&mut dd2,
            //distribution,
            //table.add_row(row![format!("DD {}:{}", err_val, num_bucket_val)]),
            //);
            //}
            table.printstd();
        }
    }
}

#[allow(dead_code)]
fn test_digest_params() {
    let counts = vec![
        vec![1_000],
        vec![1_000_000],
        vec![3_000_000],
        vec![10, 100, 150, 1_000, 3_000_000, 1_000_000],
    ];

    let mut distributions = get_distributions();

    for (distr, distribution) in &mut distributions {
        for count_group in counts.iter().cloned() {
            let mut table = Table::new();
            let count_str = pretty_print_count(&count_group);

            println!("\nCOUNT=[{}]", count_str);

            table.set_titles(row![
                "Distribution",
                "Algorithm",
                "Time",
                "PeakMemory",
                "SerializedSize",
                "50.0",
                "95.0",
                "99.0",
                "99.9",
                "99.99"
            ]);

            let all = || AllValues::new();

            test(&count_group, all, distribution, table.add_row(row![distr]));

            let batch_sizes = vec![100usize];
            let max_sizes = vec![100, 200, 300, 500, 1000, 2000];

            use itertools::Itertools;
            for (batch, max_size) in batch_sizes.iter().cartesian_product(max_sizes.iter()) {
                test(
                    &count_group,
                    || TDigest::new(*batch, *max_size),
                    distribution,
                    table.add_row(row![format!("Batch:T-Size {}:{}", batch, max_size)]),
                );
            }
            table.printstd();
        }
    }
}

trait Aggregate {
    fn name(&self) -> &str;
    fn finalize(&mut self) {}
    fn get_quantil(&mut self, q: f64) -> f64;

    fn serialize_size(&self) -> usize {
        0
    }
    fn insert(&mut self, value: f64);
    fn merge(other: Vec<Self>) -> Self
    where
        Self: Sized;

    fn get_percentiles(&mut self) -> Percentiles {
        self.finalize();
        SELECTED_PERCENTILES
            .iter()
            .cloned()
            .map(|percentil| Percentile::from(self.get_quantil(percentil / 100.0)))
            .collect::<Vec<Percentile>>()
    }
}

const SELECTED_PERCENTILES: [f64; 5] = [50.0, 95.0, 99.0, 99.9, 99.99];

type Percentiles = Vec<Percentile>;
struct Percentile {
    value: f64,
}
impl From<f64> for Percentile {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

///
fn test<A: Aggregate, F: Fn() -> A>(
    count_group: &[usize],
    aggregate: F,
    sampler: &mut Box<dyn FnMut(usize) -> f64>,
    row: &mut Row,
) {
    let start = std::time::Instant::now();
    GLOBAL.reset_peak_memory();

    let aggregates = count_group
        .iter()
        .map(|count| {
            let mut aggregate = aggregate();
            for i in 0..*count {
                let value = sampler(i);
                aggregate.insert(value);
            }
            aggregate.finalize();
            aggregate
        })
        .collect();
    let mut aggregate = A::merge(aggregates);
    let percentiles = aggregate.get_percentiles();
    let elapsed = start.elapsed().as_secs_f64();
    let peak_memory = GLOBAL.get_peak_memory();

    for entry in [
        aggregate.name().to_owned(),
        format!("{:.3}s", elapsed),
        format!("{}k ", peak_memory / 1024),
        pretty_print_ser_size(aggregate.serialize_size()),
    ] {
        row.add_cell(Cell::new(&entry));
    }
    for percentile in percentiles {
        row.add_cell(Cell::new(&format!("{:.2}", percentile.value)));
    }
}

fn pretty_print_ser_size(size: usize) -> String {
    match size {
        0 => "unavailable".to_string(),
        1..=2000 => format!("{}", size),
        _ => format!("{}k", size / 1024),
    }
}

struct AllValues {
    values: Vec<f64>,
}
impl AllValues {
    fn new() -> Self {
        let values = Vec::new();
        AllValues { values }
    }
}
impl Aggregate for AllValues {
    fn name(&self) -> &str {
        "AllValues"
    }
    fn insert(&mut self, value: f64) {
        self.values.push(value);
    }

    fn serialize_size(&self) -> usize {
        self.values.len() * 8
    }

    fn merge(mut other: Vec<Self>) -> Self
    where
        Self: Sized,
    {
        let mut first = other.pop().unwrap();
        for el in other {
            first.values.extend_from_slice(&el.values);
        }
        first
    }

    fn finalize(&mut self) {
        self.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);

        let index = (self.values.len() as f64 * q).ceil() as usize;
        let index = index.min(self.values.len() - 1);
        self.values[index] as f64
    }
}

struct QuantilesCKMS {
    q: quantiles::ckms::CKMS<f64>,
}
impl QuantilesCKMS {
    #[allow(dead_code)]
    fn new(error: f64) -> Self {
        let q = quantiles::ckms::CKMS::new(error);
        QuantilesCKMS { q }
    }
}
impl Aggregate for QuantilesCKMS {
    fn name(&self) -> &str {
        "QuantilesCKMS"
    }
    fn insert(&mut self, value: f64) {
        self.q.insert(value);
    }
    fn merge(mut other: Vec<Self>) -> Self {
        let mut first = other.pop().unwrap();
        for el in other {
            first.q += el.q;
        }
        first
    }

    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.q.query(q).unwrap().1
    }
}

struct QuantilesGK {
    q: quantiles::greenwald_khanna::Stream<ordered_float::NotNan<f64>>,
}
impl QuantilesGK {
    #[allow(dead_code)]
    fn new(error: f64) -> Self {
        let q = quantiles::greenwald_khanna::Stream::new(error);
        QuantilesGK { q }
    }
}
impl Aggregate for QuantilesGK {
    fn name(&self) -> &str {
        "QuantilesGK"
    }
    fn insert(&mut self, value: f64) {
        let value = unsafe { ordered_float::NotNan::new_unchecked(value) };
        self.q.insert(value);
    }
    fn merge(mut other: Vec<Self>) -> Self {
        if other.len() == 1 {
            return other.pop().unwrap();
        }
        unimplemented!()
    }

    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        **self.q.quantile(q)
    }
}

struct TDigest {
    batch: Vec<f64>,
    t: tdigest::TDigest,
}
impl TDigest {
    fn new(batch: usize, max_size: usize) -> Self {
        let batch = Vec::with_capacity(batch);
        let t = tdigest::TDigest::new_with_size(max_size);
        TDigest { batch, t }
    }
    fn apply_batch(&mut self) {
        self.batch.sort_unstable_by(|a, b| a.total_cmp(b));

        self.t = self.t.merge_sorted(&self.batch);
        self.batch.clear();
    }
}
impl Aggregate for TDigest {
    fn name(&self) -> &str {
        "TDigest"
    }
    fn finalize(&mut self) {
        self.apply_batch();
    }
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.t.estimate_quantile(q)
    }
    fn insert(&mut self, value: f64) {
        if self.batch.len() == self.batch.capacity() {
            self.apply_batch();
        }
        self.batch.push(value);
    }
    fn serialize_size(&self) -> usize {
        let encoded: Vec<u8> = bincode::serialize(&self.t).unwrap();
        encoded.len()
    }

    fn merge(other: Vec<Self>) -> Self
    where
        Self: Sized,
    {
        let ts: Vec<tdigest::TDigest> = other.into_iter().map(|el| el.t).collect();
        let t = tdigest::TDigest::merge_digests(ts);
        Self { batch: vec![], t }
    }
}

struct ZWQuantile {
    sum: UnboundEpsilonSummary<ordered_float::NotNan<f64>>,
}
impl ZWQuantile {
    #[allow(dead_code)]
    fn new(epsilon: f64) -> Self {
        ZWQuantile {
            sum: UnboundEpsilonSummary::new(epsilon),
        }
    }
}
impl Aggregate for ZWQuantile {
    fn name(&self) -> &str {
        "ZWQuantile"
    }
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.sum.query(q).into_inner()
    }
    fn insert(&mut self, value: f64) {
        let value = unsafe { ordered_float::NotNan::new_unchecked(value) };
        self.sum.update(value)
    }

    fn merge(mut other: Vec<Self>) -> Self
    where
        Self: Sized,
    {
        if other.len() == 1 {
            return other.pop().unwrap();
        }
        todo!()
    }
}

struct HDRHistogram {
    histogram: Histogram<u64>,
}
impl HDRHistogram {
    fn new(sigfig: u8) -> Self {
        Self {
            histogram: Histogram::new(sigfig).unwrap(),
        }
    }
}
impl Aggregate for HDRHistogram {
    fn name(&self) -> &str {
        "HDRHistogram"
    }
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.histogram.value_at_quantile(q) as f64
    }
    fn serialize_size(&self) -> usize {
        let mut vec = Vec::new();
        V2Serializer::new()
            .serialize(&self.histogram, &mut vec)
            .unwrap();

        vec.len()
    }

    fn insert(&mut self, value: f64) {
        self.histogram.record(value as u64).unwrap()
    }
    fn merge(mut other: Vec<Self>) -> Self {
        let mut first = other.pop().unwrap();
        for el in other {
            first.histogram += el.histogram;
        }
        first
    }
}

struct DDSketch {
    sketch: sketches_ddsketch::DDSketch,
}

impl DDSketch {
    fn new() -> Self {
        let c = sketches_ddsketch::Config::defaults();
        let sketch = sketches_ddsketch::DDSketch::new(c);
        Self { sketch }
    }
}
impl Aggregate for DDSketch {
    fn name(&self) -> &str {
        "DDSketch"
    }
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.sketch.quantile(q).unwrap().unwrap()
    }
    fn serialize_size(&self) -> usize {
        let encoded: Vec<u8> = bincode::serialize(&self.sketch).unwrap();
        encoded.len()
    }

    fn insert(&mut self, value: f64) {
        self.sketch.add(value)
    }
    fn merge(mut other: Vec<Self>) -> Self {
        let mut first = other.pop().unwrap();
        for el in other {
            first.sketch.merge(&el.sketch).unwrap();
        }
        first
    }
}

use sketches_rust::index_mapping::CubicallyInterpolatedMapping;
use sketches_rust::store::Store;
use sketches_rust::store::UnboundedSizeDenseStore;
struct DDSketch2<T: Store> {
    sketch: sketches_rust::DDSketch<CubicallyInterpolatedMapping, T>,
}

impl DDSketch2<UnboundedSizeDenseStore> {
    fn unbounded(error: f64) -> Self {
        Self {
            sketch: sketches_rust::DDSketch::unbounded_dense(error).unwrap(),
        }
    }
}
impl<T: Store> Aggregate for DDSketch2<T> {
    fn name(&self) -> &str {
        "DDSketch2"
    }
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.sketch.get_value_at_quantile(q).unwrap()
    }
    fn serialize_size(&self) -> usize {
        0
    }

    fn insert(&mut self, value: f64) {
        self.sketch.accept(value)
    }
    fn merge(mut other: Vec<Self>) -> Self {
        let mut first = other.pop().unwrap();
        for el in other.iter_mut() {
            first.sketch.merge_with(&mut el.sketch).unwrap();
        }
        first
    }
}

trait DisplayWithUnderscores {
    fn separate_with_underscores(&self) -> String;
}

impl DisplayWithUnderscores for usize {
    fn separate_with_underscores(&self) -> String {
        self.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
            .join("_")
    }
}
