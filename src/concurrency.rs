use anyhow::{Context, Result as AnyhowResult, anyhow};
use std::sync::MutexGuard;
use std::{
    sync::{LazyLock, Mutex},
    thread,
};

/// Счётчик.
static COUNTER: LazyLock<Mutex<u64>> = LazyLock::new(|| Mutex::new(0_u64));

/// Небезопасный инкремент через несколько потоков.
/// Использует global static mut без синхронизации — data race.
///
/// ## Оптимизация
///
/// Код обновлён с учётом использования [`Mutex`]. Блоки unsafe не требуются,
/// и удалены.
pub fn race_increment(iterations: usize, threads: usize) -> AnyhowResult<u64> {
    reset_counter()?;

    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        handles.push(thread::spawn(move || -> AnyhowResult<()> {
            for _ in 0..iterations {
                let mut c = counter()?;
                *c += 1;
            }
            Ok(())
        }));
    }
    for h in handles {
        let worker_result = h.join().map_err(|_| anyhow!("воркер запаниковал"))?;
        worker_result.context("процесс вернул ошибку")?;
    }

    Ok(*counter()?)
}

/// Плохая «синхронизация» — просто sleep, возвращает потенциально устаревшее значение.
///
/// ## Оптимизация
///
/// Синхронизация значений обеспечивается силами [`Mutex`]. В связи с этим
/// блокировка через `sleep` не требуется.
pub fn read_after_sleep() -> AnyhowResult<u64> {
    let counter = counter()?;
    Ok(*counter)
}

/// Сброс счётчика (также небезопасен, без синхронизации).
///
/// ## Оптимизация
///
/// Сброс счётчика преобразован в безопасный.
pub fn reset_counter() -> AnyhowResult<()> {
    let mut counter = counter()?;
    *counter = 0_u64;

    Ok(())
}

/// Вернуть защищённый [`Mutex`] объект счётчика.
fn counter() -> AnyhowResult<MutexGuard<'static, u64>> {
    COUNTER.lock().map_err(|e| anyhow!(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Тестирование работы со счётчиком.
    #[test]
    fn test_race_increment() {
        {
            let result = race_increment(15, 1);
            assert!(result.is_ok());
            let counter = counter().unwrap();
            assert_eq!(*counter, 15);
        }

        {
            let result = race_increment(25, 3);
            assert!(result.is_ok());
            let counter = counter().unwrap();
            assert_eq!(*counter, 75);
        }
    }
}
