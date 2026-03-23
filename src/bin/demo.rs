use rand::distr::uniform::SampleUniform;
use rand::{random_iter, RngExt, SeedableRng};
use rand::distr::{Distribution, StandardUniform};
use rand::rngs::SmallRng;
use broken_app::{algo, leak_buffer, normalize, sum_even};

fn main() {
    const MAX_LEN: usize = 50_000;
    
    let nums_vec: Vec<i64> = random_sequence(MAX_LEN);
    // let nums = [1, 2, 3, 4];
    println!("sum_even: {}", sum_even(&nums_vec));

    let data_vec: Vec<u8> = random_sequence(MAX_LEN);
    // let data = [1_u8, 0, 2, 3];
    println!("non-zero bytes: {}", leak_buffer(&data_vec));

    let text = " Hello World ";
    println!("normalize: {}", normalize(text));

    for i in MAX_LEN {
        let fib = algo::slow_fib(20);
        // println!("fib(20): {}", fib);        
    }


    let uniq = algo::slow_dedup(&[1, 2, 2, 3, 1, 4, 4]);
    println!("dedup: {:?}", uniq);
}


/// Возвращает случайное число в заданном диапазоне.
fn random<T>(min: T, max: T) -> T
where 
    T: SampleUniform + PartialOrd
{
    let mut rng = rand::rng();
    rng.random_range(min..=max)
}


/// Генератор последовательности заданной длины.
/// 
/// Возвращает вектор с заданным заказчиком типом.
fn random_sequence<T>(max: usize) -> Vec<T>
where
    T: Sized,
    StandardUniform: Distribution<T>
{
    let rng = SmallRng::seed_from_u64(0);
    rng.random_iter().take(max).collect::<Vec<T>>()
}
