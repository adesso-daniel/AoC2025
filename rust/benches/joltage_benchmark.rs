use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::prelude::*;

const NUM_ROWS: usize = 200;
const LENGTHS: [usize; 3] = [20, 50, 100];

fn do_benchmark_old(input: &String) {
    let input_rows = input.split("\r\n");
    for row in input_rows {
        aoc2025::day_three::find_joltage_rating(&row.to_string());
    }
}

fn do_benchmark_new(input: &String, length: usize) {
    let input_rows = input.split("\r\n");
    for row in input_rows {
        aoc2025::day_three::find_joltage_rating_override(&row.to_string(), length);
    }
}

fn generate_inputs() -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut rng = rand::rng();

    for length in LENGTHS {
        let mut row_collection: String = "".to_string();
        for i in 0..NUM_ROWS {
            let row: String = ::std::iter::repeat(rng.random_range(1..=9).to_string())
                .take(length)
                .collect();

            row_collection += &row;
            if i + 1 < NUM_ROWS {
                row_collection += &"\r\n";
            }
        }
        result.push(row_collection);
    }
    result
}

fn criterion_benchmark(c: &mut Criterion) {
    let inputs = generate_inputs();
    let mut group = c.benchmark_group("joltage_rating");

    let mut i = 0;
    for length in LENGTHS {
        let input = &inputs[i];
        group.throughput(criterion::Throughput::Elements(input.len() as u64));
        group.bench_with_input(BenchmarkId::new("old", length), input, |b, input| {
            b.iter(|| do_benchmark_old(&input))
        });
        group.bench_with_input(BenchmarkId::new("new len 2", length), input, |b, input| {
            b.iter(|| do_benchmark_new(&input, 2))
        });
        group.bench_with_input(BenchmarkId::new("new len 12", length), input, |b, input| {
            b.iter(|| do_benchmark_new(&input, 12))
        });
        i += 1;
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
