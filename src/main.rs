use std::io::{BufRead, BufReader};

use hdrhistogram::{
    serialization::{Serializer, V2Serializer},
    Histogram,
};
use itertools::Itertools;
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    row, Cell, Row, Table,
};
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
    //let exp = rand_distr::Exp::new(0.6).unwrap();
    // Simulate webserver response times
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
    //let mut rng5 = StdRng::from_seed([1u8; 32]);
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
        //("Exp Distribution", Box::new(move |_| exp.sample(&mut rng5))),
    ];
    distributions
}

#[allow(dead_code)]
fn test_counts() {
    // If there are multiple counts, the Algorithm has to support `merge`.
    let counts = vec![
        vec![1_000],
        //vec![1_000_000],
        vec![5_000_000],
        vec![1_000, 3_000_000, 1_000_000],
        (0..1000).map(|_| 1000).collect::<Vec<_>>(),
        //(0..10000).map(|_| 100).collect::<Vec<_>>(),
    ];
    //let ckms_error = 0.0001;
    //let gk_error = 0.001;
    //let zw_error = 0.001;
    let hdr_sigfig = 2;
    let tdigest_batch = 500;
    let tdigest_max_size = 300;
    let dd2_err = 0.01;

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
            let dd2 = || DDSketch2::unbounded(dd2_err);
            let quanto = || Quantogram::new();
            //let dd3 = || DDSketch2::logarithmic_low(dd2_err);
            //let dd4 = || DDSketch2::logarithmic_high(dd2_err);

            println!(
                "\nCOUNT={}, TDIGEST_BATCH={}, TDIGEST_MAX_SIZE={}, HDR_SIGFIG={}, DDSketch2Err={}",
                pretty_print_count(&count_group),
                tdigest_batch.separate_with_underscores(),
                tdigest_max_size,
                hdr_sigfig,
                dd2_err,
            );
            let mut table = get_markdown_table();
            table.set_titles(row![
                "Distribution",
                "Algorithm",
                "Time",
                "PeakMemory",
                "SerializedSize",
                "50.0",
                "75.0",
                "90.0",
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
            test(
                &count_group,
                quanto,
                distribution,
                table.add_row(row![distr]),
            );
            //test(&count_group, dd3, distribution, table.add_row(row![distr]));
            //test(&count_group, dd4, distribution, table.add_row(row![distr]));

            table.printstd();
        }
    }
}

fn pretty_print_count(count_group: &[usize]) -> String {
    let all_same = count_group.iter().tuple_windows().all(|(a, b)| a == b);
    if count_group.len() > 3 && all_same {
        return format!(
            "[{}, {}, ...x{}]",
            count_group[0],
            count_group[0],
            count_group.len()
        );
    }
    let yo = count_group
        .iter()
        .map(|el| el.separate_with_underscores())
        .collect::<Vec<_>>()
        .join(", ");

    format!("[{}]", yo)
}

#[allow(dead_code)]
fn test_sketch_params() {
    let counts = vec![
        vec![1_000],
        vec![1_000_000],
        vec![3_000_000],
        vec![1_000, 3_000_000, 1_000_000],
    ];

    //let mut results = HashMap::default();

    let mut distributions = get_distributions();

    for (distr, distribution) in &mut distributions {
        for count_group in counts.iter().cloned() {
            let mut table = get_markdown_table();
            let count_str = pretty_print_count(&count_group);

            println!("\nCOUNT={}", count_str);

            table.set_titles(row![
                "Distribution",
                "Algorithm",
                "Time",
                "PeakMemory",
                "SerializedSize",
                "50.0",
                "75.0",
                "90.0",
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

fn get_markdown_table() -> Table {
    let mut table = Table::new();

    let minus_pipe_sep: LineSeparator = LineSeparator::new('-', '|', '|', '|');
    let format_markdown = FormatBuilder::new()
        .padding(1, 1)
        .borders('|')
        .separator(LinePosition::Title, minus_pipe_sep)
        .column_separator('|')
        .build();
    table.set_format(format_markdown);

    table
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
            let mut table = get_markdown_table();
            let count_str = pretty_print_count(&count_group);

            println!("\nCOUNT={}", count_str);

            table.set_titles(row![
                "Distribution",
                "Algorithm",
                "Time",
                "PeakMemory",
                "SerializedSize",
                "50.0",
                "75.0",
                "90.0",
                "95.0",
                "99.0",
                "99.9",
                "99.99"
            ]);

            let all = || AllValues::new();

            test(&count_group, all, distribution, table.add_row(row![distr]));

            let batch_sizes = vec![100usize];
            let max_sizes = vec![100, 200, 300, 500, 1000, 2000];

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
    fn insert(&mut self, value: f64);

    fn serialize_size(&self) -> usize {
        0
    }

    // Default implementation which covers the nothing to merge case
    fn merge(mut other: Vec<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        if other.len() == 1 {
            return other.pop();
        }
        None
    }

    fn get_percentiles(&mut self) -> Percentiles {
        self.finalize();
        SELECTED_PERCENTILES
            .iter()
            .cloned()
            .map(|percentil| Percentile::from(self.get_quantil(percentil / 100.0)))
            .collect::<Vec<Percentile>>()
    }
}

const SELECTED_PERCENTILES: [f64; 7] = [50.0, 75.0, 90.0, 95.0, 99.0, 99.9, 99.99];

type Percentiles = Vec<Percentile>;
struct Percentile {
    value: f64,
}
impl From<f64> for Percentile {
    fn from(value: f64) -> Self {
        Self { value }
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct TestResult {
    pub name: String,
    pub run_time: f64,
    pub memory: usize,
    pub percentiles: Percentiles,
}

///
fn test<A: Aggregate, F: Fn() -> A>(
    count_group: &[usize],
    aggregate: F,
    sampler: &mut Box<dyn FnMut(usize) -> f64>,
    row: &mut Row,
) -> TestResult {
    let start = std::time::Instant::now();
    GLOBAL.reset_peak_memory();

    #[cfg(feature = "parallel-collect")]
    let aggregates = {
        let mut iters = count_group
            .iter()
            .map(|count| (0..*count, aggregate()))
            .collect::<Vec<_>>();
        let mut finished_aggregates = Vec::new();
        let mut iter_index = 0;
        loop {
            let iter = &mut iters[iter_index];

            if let Some(i) = iter.0.next() {
                let value = sampler(i);
                iter.1.insert(value);
            } else {
                let mut aggregate = iters.remove(iter_index).1;
                aggregate.finalize();
                finished_aggregates.push(aggregate);
                if iters.is_empty() {
                    break;
                }
            }

            iter_index += 1;
            iter_index %= iters.len();
        }
        finished_aggregates
    };
    #[cfg(not(feature = "parallel-collect"))]
    let aggregates = {
        count_group
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
            .collect::<Vec<_>>()
    };
    let name = aggregates[0].name().to_string();
    let mut aggregate = if let Some(aggregate) = A::merge(aggregates) {
        aggregate
    } else {
        // Unsupported
        // Fill cells
        row.add_cell(Cell::new(&name));
        for _ in 1..4 + SELECTED_PERCENTILES.len() {
            row.add_cell(Cell::new(&"NaN"));
        }

        return TestResult::default();
    };

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

    TestResult {
        name: aggregate.name().to_string(),
        run_time: elapsed,
        memory: peak_memory,
        percentiles: aggregate.get_percentiles(),
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
    fn finalize(&mut self) {
        self.values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);

        let index = (self.values.len() as f64 * q).ceil() as usize;
        let index = index.min(self.values.len() - 1);
        self.values[index] as f64
    }

    fn insert(&mut self, value: f64) {
        self.values.push(value);
    }

    fn serialize_size(&self) -> usize {
        self.values.len() * 8
    }

    fn merge(mut other: Vec<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        let mut first = other.pop().unwrap();
        for el in other {
            first.values.extend_from_slice(&el.values);
        }
        Some(first)
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
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.q.query(q).unwrap().1
    }
    fn insert(&mut self, value: f64) {
        self.q.insert(value);
    }
    fn merge(mut other: Vec<Self>) -> Option<Self> {
        let mut first = other.pop().unwrap();
        for el in other {
            first.q += el.q;
        }
        Some(first)
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
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        **self.q.quantile(q)
    }
    fn insert(&mut self, value: f64) {
        let value = unsafe { ordered_float::NotNan::new_unchecked(value) };
        self.q.insert(value);
    }
}

struct TDigest {
    batch: Vec<f64>,
    batch_size: usize,
    t: tdigest::TDigest,
}
impl TDigest {
    fn new(batch_size: usize, max_size: usize) -> Self {
        let batch = Vec::new();
        let t = tdigest::TDigest::new_with_size(max_size);
        TDigest {
            batch,
            t,
            batch_size,
        }
    }
    fn apply_batch(&mut self) {
        self.batch.sort_unstable_by(|a, b| a.total_cmp(b));

        self.t = self.t.merge_sorted(&mut self.batch);
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
        if self.batch.len() == self.batch_size {
            self.apply_batch();
        }
        self.batch.push(value);
    }
    fn serialize_size(&self) -> usize {
        //let encoded: Vec<u8> = bincode::serialize(&self.t).unwrap();
        //encoded.len()
        serde_json::to_string(&self.t).unwrap().len()
    }

    fn merge(other: Vec<Self>) -> Option<Self>
    where
        Self: Sized,
    {
        let batch_size = other[0].batch_size;
        let ts: Vec<tdigest::TDigest> = other.into_iter().map(|el| el.t).collect();
        let t = tdigest::TDigest::merge_digests(ts);
        Some(Self {
            batch: vec![],
            t,
            batch_size,
        })
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
    fn insert(&mut self, value: f64) {
        self.histogram.record(value as u64).unwrap()
    }

    fn serialize_size(&self) -> usize {
        let mut vec = Vec::new();
        V2Serializer::new()
            .serialize(&self.histogram, &mut vec)
            .unwrap();

        vec.len()
    }
    fn merge(mut other: Vec<Self>) -> Option<Self> {
        let mut first = other.pop().unwrap();
        for el in other {
            first.histogram += el.histogram;
        }
        Some(first)
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
    fn insert(&mut self, value: f64) {
        self.sketch.add(value)
    }

    fn serialize_size(&self) -> usize {
        serde_json::to_string(&self.sketch).unwrap().len()
    }
    fn merge(mut other: Vec<Self>) -> Option<Self> {
        let mut first = other.pop().unwrap();
        for el in other {
            first.sketch.merge(&el.sketch).unwrap();
        }
        Some(first)
    }
}

use sketches_rust::store::UnboundedSizeDenseStore;
use sketches_rust::store::{CollapsingHighestDenseStore, Store};
use sketches_rust::{
    index_mapping::{CubicallyInterpolatedMapping, IndexMapping, LogarithmicMapping},
    store::CollapsingLowestDenseStore,
};
struct DDSketch2<I: IndexMapping, T: Store> {
    sketch: sketches_rust::DDSketch<I, T>,
}

impl DDSketch2<CubicallyInterpolatedMapping, UnboundedSizeDenseStore> {
    fn unbounded(error: f64) -> Self {
        Self {
            sketch: sketches_rust::DDSketch::unbounded_dense(error).unwrap(),
        }
    }
}

impl DDSketch2<LogarithmicMapping, CollapsingLowestDenseStore> {
    #[allow(unused)]
    fn logarithmic_low(error: f64) -> Self {
        Self {
            sketch: sketches_rust::DDSketch::logarithmic_collapsing_lowest_dense(error, 2000)
                .unwrap(),
        }
    }
}

impl DDSketch2<LogarithmicMapping, CollapsingHighestDenseStore> {
    #[allow(unused)]
    fn logarithmic_high(error: f64) -> Self {
        Self {
            sketch: sketches_rust::DDSketch::logarithmic_collapsing_highest_dense(error, 2000)
                .unwrap(),
        }
    }
}

impl<I: IndexMapping, T: Store> Aggregate for DDSketch2<I, T> {
    fn name(&self) -> &str {
        "DDSketch2"
    }
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.sketch.get_value_at_quantile(q).unwrap()
    }
    fn insert(&mut self, value: f64) {
        self.sketch.accept(value)
    }

    fn serialize_size(&self) -> usize {
        0
    }
    fn merge(mut other: Vec<Self>) -> Option<Self> {
        let mut first = other.pop().unwrap();
        for el in other.iter_mut() {
            first.sketch.merge_with(&mut el.sketch).unwrap();
        }
        Some(first)
    }
}

struct Quantogram {
    quantogram: quantogram::Quantogram,
}

impl Quantogram {
    fn new() -> Self {
        Self {
            quantogram: quantogram::Quantogram::new(),
        }
    }
}
impl Aggregate for Quantogram {
    fn name(&self) -> &str {
        "Quantogram"
    }
    fn get_quantil(&mut self, q: f64) -> f64 {
        assert!(q >= 0f64 && q <= 1f64);
        self.quantogram.quantile(q).unwrap()
    }
    fn insert(&mut self, value: f64) {
        self.quantogram.add(value)
    }

    fn serialize_size(&self) -> usize {
        0
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
