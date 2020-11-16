use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use space_sort::*;
extern crate quickersort;
extern crate rand;
extern crate rand_pcg;
extern crate rdxsort;
use rand::{distributions::Uniform, Rng, SeedableRng};
use rand_pcg::Pcg64;
use rdxsort::*;
use std::time::Duration;

const MAX_SIZE: usize = 10_000_000;
const MIN: u8 = 0;
const MAX_RANGE: u8 = 255;
const MED_RANGE: u8 = 100;
const SMALL_RANGE: u8 = 10;

fn build_vec(max_val: usize) -> Vec<usize> {
    let mut r: Vec<usize> = vec![10, 100, 1000, 10_000, 100_000, 1_000_000];
    let mut i: usize = 1_000_000;
    while i < max_val {
        i += 1_000_000;
        r.push(i);
    }
    r
}

pub fn cribench_sorts_max_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorts_max_range");
    let rng = Pcg64::from_seed([2; 32]);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MAX_RANGE);
        let values: Vec<char> = rng
            .clone()
            .sample_iter(&range)
            .take(*size)
            .map(char::from)
            .collect();
        group.bench_with_input(BenchmarkId::new("spacesort", size), size, |b, &size| {
            b.iter(|| black_box(space_sort_by(values.clone(), |x| x as u32)));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort_by_key(&mut values.clone(), |x| *x as u32)));
        });
    }
}

pub fn cribench_sorts_med_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorts_med_range");
    let rng = Pcg64::from_seed([2; 32]);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MIN + MED_RANGE);
        let values: Vec<char> = rng
            .clone()
            .sample_iter(&range)
            .take(*size)
            .map(char::from)
            .collect();
        group.bench_with_input(BenchmarkId::new("spacesort", size), size, |b, &size| {
            b.iter(|| black_box(space_sort_by(values.clone(), |x| x as u32)));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort_by_key(&mut values.clone(), |x| *x as u32)));
        });
    }
}

pub fn cribench_sorts_small_range(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorts_small_range");
    let rng = Pcg64::from_seed([2; 32]);
    for size in build_vec(MAX_SIZE).iter() {
        let range = Uniform::from(MIN..MIN + SMALL_RANGE);
        let values: Vec<char> = rng
            .clone()
            .sample_iter(&range)
            .take(*size)
            .map(char::from)
            .collect();
        group.bench_with_input(BenchmarkId::new("spacesort", size), size, |b, &size| {
            b.iter(|| black_box(space_sort_by(values.clone(), |x| x as u32)));
        });
        group.bench_with_input(BenchmarkId::new("quickersort", size), size, |b, &size| {
            b.iter(|| black_box(quickersort::sort_by_key(&mut values.clone(), |x| *x as u32)));
        });
    }
}
criterion_group!(
    benches,
    cribench_sorts_max_range,
    cribench_sorts_med_range,
    cribench_sorts_small_range
);
criterion_main!(benches);
