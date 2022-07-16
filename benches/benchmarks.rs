use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quick_cache::sync::Cache;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rand_distr::Zipf;

pub fn criterion_benchmark(c: &mut Criterion) {
    for population in [10_000, 1_000_000] {
        for s in [0.5, 0.75] {
            let mut g = c.benchmark_group(format!("Zipf N={} S={}", population, s));
            g.throughput(criterion::Throughput::Elements(population));
            for capacity_ratio in [0.05, 0.1, 0.15] {
                let capacity = (population as f64 * capacity_ratio) as usize;
                g.bench_function(format!("qc {}", capacity), |b| {
                    let mut _hits = 0u64;
                    let mut _misses = 0u64;
                    b.iter_batched_ref(
                        || {
                            let mut rng = SmallRng::seed_from_u64(1);
                            let mut dist = Zipf::new(population, s).unwrap();
                            let mut cache = Cache::new(capacity, capacity);
                            for i in 0..population * 10 {
                                let sample = dist.sample(&mut rng) as usize;
                                cache.insert(sample, sample);
                            }
                            (rng, dist, cache)
                        },
                        |(rng, dist, cache)| {
                            for _ in 0..population {
                                let sample = dist.sample(rng) as usize;
                                if cache.get(&sample).is_some() {
                                    _hits += 1;
                                } else {
                                    cache.insert(sample, sample);
                                    _misses += 1;
                                }
                            }
                        },
                        criterion::BatchSize::LargeInput,
                    );
                    // eprintln!("Hit rate {:?}", _hits as f64 / (_hits + _misses) as f64);
                });
            }
        }
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
