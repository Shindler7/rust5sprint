pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
///
/// ## Примечание
///
/// Код изменен: убран неоправданный `unsafe` и метод `get_unchecked()`,
/// который приводил к выходу за пределы массива. Добавлены тесты.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|&v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
pub fn leak_buffer(input: &[u8]) -> usize {
    let boxed = input.to_vec().into_boxed_slice();
    let len = input.len();
    let raw = Box::into_raw(boxed) as *mut u8;

    let mut count = 0;
    unsafe {
        for i in 0..len {
            if *raw.add(i) != 0_u8 {
                count += 1;
            }
        }
        // утечка: не вызываем Box::from_raw(raw);
    }
    count
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
///
/// ## Примечание
///
/// Код исправлен: теперь высчитывается среднее значение положительных чисел
/// в массиве. Реализуется за один проход и без копирования массива. Добавлены
/// тесты.
pub fn average_positive(values: &[i64]) -> f64 {
    let mut count = 0_usize;
    let mut sum = 0_i64;
    for &v in values {
        if v > 0 {
            count += 1;
            sum += v;
        }
    }

    if count == 0 {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);   // Выделили память в куче.
    let raw = Box::into_raw(b);     // Забрали сырой указатель и владение.
    let val = unsafe {*raw};
    drop(Box::from_raw(raw));
    val + *raw
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Пустой слайд не должен вызывать ошибку.
    #[test]
    fn regress_sum_even_empty() {
        assert_eq!(sum_even(&[]), 0);
    }

    /// Проверим, что у нас итерация не вызывает дополнительного шага.
    #[test]
    fn regress_sum_even_last() {
        assert_eq!(sum_even(&[4]), 4);
    }

    /// Проверка корректности работы функции.
    #[test]
    fn regress_sum_even() {
        assert_eq!(sum_even(&[1, 2, 3, 5, 6, 8, 19, 21]), 16);
    }

    /// Проверяет, что отрицательные числа не учитываются в расчёте.
    #[test]
    fn regress_average_not_positive_ignore() {
        assert_eq!(average_positive(&[-1, -2, -3, -4, -5]), 0.0);
    }

    /// Проверяет корректность выборки для подсчёта среднего.
    #[test]
    fn regress_average_positive() {
        assert_eq!(average_positive(&[2, -2, -5, 2, 2]), 2.0);
    }
}
