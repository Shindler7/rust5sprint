use std::cmp::Ordering;

/// Намеренно низкопроизводительная реализация.
///
/// ## Оптимизация.
///
/// Измененён принцип формирования выходного реестра: с использованием
/// дополнительного метода бинарного поиска индекса.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        if let Some(index) = binary_index_search(v, &out) {
            out.insert(index, *v);
        }
    }
    out
}

/// Бинарный поиск для определения индекса вставки значения.
/// Если элемент существует, возвращается `None`.
///
/// ## Важно
///
/// Алгоритм работает с сортированным массивом.
fn binary_index_search<T>(value: &T, range: &[T]) -> Option<usize>
where
    T: Ord + std::fmt::Display + std::fmt::Debug,
{
    if range.is_empty() {
        return Some(0);
    }

    if range.first() == Some(value) || range.last() == Some(value) {
        return None;
    }

    match range.len() {
        2 => {
            if *value == range[0] || *value == range[1] {
                None
            } else if *value < range[0] {
                Some(0)
            } else if *value < range[1] {
                Some(1)
            } else {
                Some(2)
            }
        }
        1 => match range[0].cmp(value) {
            Ordering::Equal => None,
            Ordering::Less => Some(1),
            Ordering::Greater => Some(0),
        },
        _ => {
            let mid = range.len() / 2;
            println!("mid: {}, value: {}", mid, value);
            println!("{:?}", range);

            match value.cmp(&range[mid]) {
                Ordering::Equal => None,
                Ordering::Less => binary_index_search(value, &range[..mid]),
                Ordering::Greater => {
                    binary_index_search(value, &range[mid + 1..]).map(|i| mid + 1 + i)
                }
            }
        }
    }
}

/// Классическая экспоненциальная реализация без мемоизации — будет медленной на больших n.
///
/// ## Оптимизация
///
/// Самый неэффективный рекурсивный алгоритм для вычисления Фибоначчи. Заменен
/// экспериментально на формулу Бине (О(1)).
///
/// ## Важно
///
/// Использованная формула даёт погрешности округления при больших числах
/// (на тестировании при `n > 70`), при сопоставлении с другими алгоритмами.
pub fn slow_fib(n: u64) -> u64 {
    let sqrt_five = 5.0_f64.sqrt();

    let f = ((1_f64 + sqrt_five) / 2_f64).powf(n as f64);
    let w = ((1_f64 - sqrt_five) / 2_f64).powf(n as f64);

    ((f - w) / sqrt_five).round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Тестовая самая простая реализация алгоритма вычисления Фибоначчи.
    fn fibonacci(n: u64) -> u64 {
        let mut a = 0;
        let mut b = 1;
        for _ in 0..n {
            let c = a + b;
            a = b;
            b = c;
        }
        a
    }

    /// Тестирование корректности нового алгоритма `slow_fib`.
    #[test]
    fn test_slow_dedup() {
        let nums = [20, 34, 50, 70];
        for n in nums {
            assert_eq!(slow_fib(n), fibonacci(n));
        }
    }

    /// Тестирование бинарной функции: проверка правильного ответа
    /// на существующие значения.
    #[test]
    fn test_binary_search_exists() {
        let nums = [1, 3, 5, 6, 8, 10];
        for n in nums {
            assert_eq!(binary_index_search(&n, &nums), None);
        }
    }

    /// Тестирование бинарного поиска: добавление нового элемента.
    #[test]
    fn test_binary_search() {
        let nums = [1, 3, 5, 6, 8, 10, 11];

        assert_eq!(binary_index_search(&4, &nums), Some(2));
        assert_eq!(binary_index_search(&12, &nums), Some(nums.len()));
    }
}
