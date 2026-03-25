/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            // лишняя копия, хотя можно было пушить значение напрямую
            out.push(*v);
            out.sort_unstable(); // бесполезная сортировка на каждой вставке
        }
    }
    out
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
/// Использованная формула даёт неточности при больших `n`.
pub fn slow_fib(n: u64) -> u64 {
    let sqrt_five = 5.0_f64.sqrt();

    let f = ((1_f64 + sqrt_five) / 2_f64).powf(n as f64);
    let w = ((1_f64 - sqrt_five) / 2_f64).powf(n as f64);

    ((f - w) / sqrt_five).round() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slow_dedup() {
        let n = 20;
        assert_eq!(slow_fib(n), 6765);
    }
}
