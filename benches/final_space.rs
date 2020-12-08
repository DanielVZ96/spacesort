use criterion::BenchmarkId;
use criterion::{criterion_group, criterion_main, Criterion, black_box, PlotConfiguration, AxisScale};
use spacesort::space_sort;
extern crate rand;
extern crate rand_pcg;
extern crate rdxsort;
extern crate quickersort;
use rand::{distributions::Uniform, SeedableRng, Rng};
use rand_pcg::Pcg64;
use rdxsort::*;

const MAX_SIZE : usize = 10_000_000;
const MIN : usize = 0;
const MAX_RANGE : usize = 100_000_000;
const BIG_RANGE : usize = 50_000_000;
const MED_RANGE : usize = 10_000_000;
const SMALL_RANGE : usize = 10_000;
const TINY_RANGE : usize = 1_000;


fn build_vec(max_val : usize) -> Vec<usize> {
    let mut r : Vec<usize> = vec![10, 50, 100, 1000, 10_000, 100_000, 1_000_000];
    let mut i : usize = 1_000_000;
    while i < max_val {
        i += 1_000_000;
        r.push(i);
    }
    r
}

pub fn cribench_sorts_max_range(c: &mut Criterion) {

    let mut group = c.benchmark_group("sorts_final_space_max_range");
    let rng = Pcg64::from_seed([2;32]);
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MIN+MAX_RANGE);
        let values: Vec<usize> = rng.clone().sample_iter(&range).take(*size).collect();   
        group.bench_with_input(BenchmarkId::new("space", size), size, |b, &size| {
            b.iter(|| black_box(space_sort(values.clone())));
        });
        group.bench_with_input(BenchmarkId::new("radix", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().rdxsort()));
        });
        group.bench_with_input(BenchmarkId::new("built-in", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().sort()));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort(&mut values.clone())));
        });
    }
}
pub fn cribench_sorts_big_range(c: &mut Criterion) {

    let mut group = c.benchmark_group("sorts_final_space_big_range");
    let rng = Pcg64::from_seed([2;32]);
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MIN+BIG_RANGE);
        let values: Vec<usize> = rng.clone().sample_iter(&range).take(*size).collect();   
        group.bench_with_input(BenchmarkId::new("space", size), size, |b, &size| {
            b.iter(|| black_box(space_sort(values.clone())));
        });
        group.bench_with_input(BenchmarkId::new("radix", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().rdxsort()));
        });
        group.bench_with_input(BenchmarkId::new("built-in", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().sort()));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort(&mut values.clone())));
        });
    }
}
pub fn cribench_sorts_med_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorts_final_space_med_range");
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    let rng = Pcg64::from_seed([2;32]);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MIN+MED_RANGE);
        let values: Vec<usize> = rng.clone().sample_iter(&range).take(*size).collect();   
        group.bench_with_input(BenchmarkId::new("space", size), size, |b, &size| {
            b.iter(|| black_box(space_sort(values.clone())));
        });
        group.bench_with_input(BenchmarkId::new("radix", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().rdxsort()));
        });
        group.bench_with_input(BenchmarkId::new("built-in", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().sort()));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort(&mut values.clone())));
        });
    }
}

pub fn cribench_sorts_small_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorts_final_space_small_range");
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    let rng = Pcg64::from_seed([2;32]);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MIN+SMALL_RANGE);
        let values: Vec<usize> = rng.clone().sample_iter(&range).take(*size).collect();   
        group.bench_with_input(BenchmarkId::new("space", size), size, |b, &size| {
            b.iter(|| black_box(space_sort(values.clone())));
        });
        group.bench_with_input(BenchmarkId::new("radix", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().rdxsort()));
        });
        group.bench_with_input(BenchmarkId::new("built-in", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().sort()));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort(&mut values.clone())));
        });
    }
}
pub fn cribench_sorts_tiny_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorts_final_space_tiny_range");
    let plot_config = PlotConfiguration::default()
        .summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);
    let rng = Pcg64::from_seed([2;32]);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MIN+TINY_RANGE);
        let values: Vec<usize> = rng.clone().sample_iter(&range).take(*size).collect();   
        group.bench_with_input(BenchmarkId::new("space", size), size, |b, &size| {
            b.iter(|| black_box(space_sort(values.clone())));
        });
        group.bench_with_input(BenchmarkId::new("radix", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().rdxsort()));
        });
        group.bench_with_input(BenchmarkId::new("built-in", size), size, |b, &size| {
            b.iter(|| black_box(values.clone().sort()));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort(&mut values.clone())));
        });
    }
}
criterion_group!(benches, cribench_sorts_max_range, cribench_sorts_big_range, cribench_sorts_med_range, cribench_sorts_small_range, cribench_sorts_tiny_range);
criterion_main!(benches);
