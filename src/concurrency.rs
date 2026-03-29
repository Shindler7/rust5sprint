use std::{
    sync::atomic::{AtomicU64, Ordering},
    thread,
};

/// Счётчик.
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// Накручивает общий счётчик из нескольких потоков и возвращает итоговое
/// значение.
pub fn race_increment(iterations: usize, threads: usize) -> u64 {
    reset_counter();

    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                increase_counter();
            }
        }));
    }
    for h in handles {
        let _ = h.join();
    }

    read_after_sleep()
}

/// Возвращает текущее значение счётчика.
pub fn read_after_sleep() -> u64 {
    COUNTER.load(Ordering::SeqCst)
}

/// Сброс счётчика.
pub fn reset_counter() {
    COUNTER.store(0, Ordering::SeqCst);
}

/// Увеличить значение счётчика на единицу.
fn increase_counter() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{LazyLock, Mutex};

    static TEST_GUARD: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    /// Тестирование работы со счётчиком (один поток).
    #[test]
    fn regress_race_increment_one_thread() {
        // В одном потоке.
        let _guard = TEST_GUARD.lock().unwrap();
        let result = race_increment(15, 1);
        let counter = COUNTER.load(Ordering::SeqCst);

        assert_eq!(result, counter);
        assert_eq!(counter, 15);
    }

    /// Тестирование счётчика при работе в мульти-потоке.
    #[test]
    fn regress_race_increment_multi_threads() {
        let _guard = TEST_GUARD.lock().unwrap();
        let result = race_increment(25, 3);
        let counter = COUNTER.load(Ordering::SeqCst);

        assert_eq!(result, counter);
        assert_eq!(counter, 75);
    }

    /// Тестирование сбрасывания счётчика.
    #[test]
    fn regress_reset_counter() {
        let _guard = TEST_GUARD.lock().unwrap();
        let result = race_increment(25, 3);
        let counter = COUNTER.load(Ordering::SeqCst);
        assert_eq!(result, counter);

        reset_counter();
        let counter = COUNTER.load(Ordering::SeqCst);
        assert_eq!(counter, 0);
    }

    /// Тестирование метода увеличения значения счётчика.
    #[test]
    fn regress_increase_counter() {
        let _guard = TEST_GUARD.lock().unwrap();
        increase_counter();
        increase_counter();
        let count = COUNTER.load(Ordering::SeqCst);
        assert_eq!(count, 2);
    }
}
