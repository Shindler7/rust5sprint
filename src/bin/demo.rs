use broken_app::{
    algo::{slow_dedup, slow_fib},
    leak_buffer, sum_even,
};
use rand::{
    RngExt, SeedableRng,
    distr::uniform::{SampleRange, SampleUniform},
    rngs::SmallRng,
};

#[cfg(not(feature = "benchmark"))]
use broken_app::normalize;

#[cfg(feature = "benchmark")]
use std::{
    hint::black_box,
    time::{Duration, Instant},
};

/// Константа длины массивов.
const SEQ_LEN: usize = 100_000_000;

#[cfg(not(feature = "benchmark"))]
fn main() {
    let nums = random_sequence(0..=100, SEQ_LEN);
    println!("sum_even: {}", sum_even(&nums));

    let data = random_sequence(0..=255, SEQ_LEN);
    println!("non-zero bytes: {}", leak_buffer(&data));

    let text = " Hello World ";
    println!("normalize: {}", normalize(text));

    let slow_num = 40u64;
    let fib = slow_fib(slow_num);
    println!("fib({}): {}", slow_num, fib);

    let nums = random_sequence(0..=100, SEQ_LEN);
    let uniq = slow_dedup(&nums);
    println!("dedup: {:?}", uniq);
}

/// Реализация `main` для проведения профилирования.
///
/// ## Компиляция
///
/// ```bash
/// cargo build --release --features benchmark
/// ```
///
/// ## Профилирование через flamegraph
///
/// Провести измерение одновременно с компиляцией можно в одну команду.
///
/// ```bash
/// cargo flamegraph --release --features benchmark --bin demo
/// ```
#[cfg(feature = "benchmark")]
fn main() {
    println!("benchmark");

    let even_data = random_sequence(0..=100, SEQ_LEN);
    let leak_data = random_sequence(0..=255, SEQ_LEN);
    let dedup_data = random_sequence(0..=100, SEQ_LEN);

    let runs = 10usize;
    let mut timings: Vec<Duration> = Vec::with_capacity(runs);

    for _ in 0..runs {
        let start = Instant::now();

        black_box(sum_even(black_box(&even_data)));
        black_box(leak_buffer(black_box(&leak_data)));
        black_box(slow_fib(black_box(40)));
        black_box(slow_dedup(black_box(&dedup_data)));

        timings.push(start.elapsed());
    }

    let total: Duration = timings.iter().copied().sum();
    let min = timings.iter().min().copied().unwrap();
    let max = timings.iter().max().copied().unwrap();
    let avg = total / runs as u32;

    println!("\n=== Final benchmark ===");
    println!(
        "min: {:.3} s, max: {:.3} s, avg: {:.3} s, total: {:.3} s",
        min.as_secs_f64(),
        max.as_secs_f64(),
        avg.as_secs_f64(),
        total.as_secs_f64()
    );
}

/// Генератор последовательности заданной длины.
///
/// Возвращает вектор с заданным заказчиком типом.
///
/// ## Args
///
/// - `range` — диапазон выбора случайного числа
/// - `len` — длина финальной последовательности
fn random_sequence<T, R>(range: R, len: usize) -> Vec<T>
where
    T: SampleUniform,
    R: SampleRange<T> + Clone,
{
    let mut rng = SmallRng::from_rng(&mut rand::rng());
    (0..len).map(|_| rng.random_range(range.clone())).collect()
}
