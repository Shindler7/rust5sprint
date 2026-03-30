# Основные заметки по проекту.

## Python-скрипт (display_tests.py)

Для проекта создан [Python-скрипт](display_tests.py), который автоматически
запускает указанные ниже варианты тестирования (кроме `gdb/lldb`, а также
профилирования и бенчмарков).

Отчёты логируются в файлы (каталог `artifacts/logs`).

- 🔄 Изменено в версии 0.1.1: используется новый каталог сохранения логов,
  вместо предыдущего (`tests/reports`). Удалены лишние файлы логирования.

Для изменения набора тестов нужно отредактировать список `steps` в функции
`main`.

### Запуск

```shell
python3 display_tests.py
```

### История логирования

#### До оптимизации

[20260323_144110_tests.log](artifacts/logs/20260323_144110_tests.log)

#### После оптимизации

[20260329_134541_tests.log](artifacts/logs/20260329_134541_tests.log)

#### После первого ревью (и обновления кода)

- 🔄 добавлено в версии 0.1.1

[20260329_170241_tests.log](artifacts/logs/20260329_170241_tests.log)

#### После второго ревью

- 🔄 добавлено в версии 0.1.2

Обеспечено безошибочное исполнение всех тестов, включая `TSan`.

[20260330_192326_tests.log](artifacts/logs/20260330_192326_tests.log)

## Инфраструктура тестирования

### Отладчик gdb

GDB (GNU Debugger) — это классический отладчик для Linux и других Unix-систем.
Он работает из командной строки и предоставляет мощные возможности для отладки:
установку точек останова, пошаговое выполнение, просмотр памяти, изменение
переменных, анализ стека вызовов.

#### Установка

```shell
rustup component add rust-gdb
```

#### Использование

```shell
cargo build
gdb target/debug/demo
```

После запуска использовать команду `run` или `r` для старта исследуемого
приложения.

### Отладчик LLDB

LLDB (Low Level Debugger) — это современный отладчик, разработанный как часть
проекта `LLVM`. Он используется по умолчанию на `macOS` (вместе с `Xcode`)
и доступен на Linux. LLDB имеет более современный интерфейс, чем GDB,
и лучше интегрируется с Rust через rust-lldb.

#### Установка

```shell
rustup component add rust-lldb
```

#### Использование

```shell
cargo build
lldb target/debug/demo
```

После запуска использовать команду `run` или `r` для старта исследуемого
приложения.

### Miri

Miri — это интерпретатор промежуточного представления Rust (MIR), который
выполняет код и отслеживает каждую операцию с памятью, проверяя правила
безопасности Rust.

#### Установка

```shell
rustup toolchain install nightly
rustup component add miri --toolchain nightly
```

#### Использование

```shell
cargo +nightly miri test
```

#### Успешный результат

- Miri не сообщает об undefined behavior (UB) (неопределённое поведение)
- все тесты успешно проходят

### Valgrind

Valgrind — это инструмент для анализа работы с динамической памятью на уровне
операционной системы.

#### Установка

**В Linux**:

```shell
sudo apt install valgrind
```

Для MacOS:

```shell
brew install valgrind
```

#### Использование

Рекомендуемый запуск для Rust-программ:

```shell
valgrind -s --leak-check=full --show-leak-kinds=all target/debug/demo
```

Ключи:

- `--show-leak-kinds=all` — показывает все типы утечек
- `--leak-check=full` — включает полную проверку утечек памяти

#### Успешный результат

- нет ошибок чтения/записи, работы с памятью
- нет потерянных (косвенно потерянных) данных, указателей

### Sanitizers (nightly)

Инструменты динамического анализа кода (встроенные в компиляторе LLVM),
доступные только в ночных сборках (`nightly`), предназначенные для поиска
ошибок памяти (адресация, утечки) и гонок данных во время исполнения.

См. [Rust Unstable Book]
(https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html).

#### Установка

Рекомендуемый набор: `nightly toolchain` и компонент `rust-src`.

```shell
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

#### Использование

Без ключей `-Zbuild-std` и `--target` поведение может быть некорректным,
результаты невалидными.

**ASan**

Детектор ошибок работы с памятью.

Документация: https://clang.llvm.org/docs/AddressSanitizer.html

```shell
RUSTFLAGS="-Zsanitizer=address" \
cargo +nightly test -Zbuild-std=std --target x86_64-unknown-linux-gnu
```

#### Успешный результат

- AddressSanitizer не сообщает об ошибках памяти

**TSan**

Инструмент для обнаружения гонок данных.

> Рекомендуется тестирование проводить на Linux.

Документация: https://clang.llvm.org/docs/ThreadSanitizer.html

```shell
RUSTFLAGS="-Zsanitizer=thread" \
cargo +nightly test -Zbuild-std=std --target x86_64-unknown-linux-gnu
```

_Примечание: `--target` должен соответствовать целевой системе._

#### Успешный результат

- ThreadSanitizer не сообщает о data race

### Бенчмарки

Тесты определения производительности.

#### Использование

```shell
mkdir -p artifacts
cargo bench --bench baseline > artifacts/baseline_before.txt
```

#### Успешный результат

Пример запуска в проекте (до оптимизации):

```text
sum_even: 18.166µs
slow_fib: 10.651916ms
slow_dedup: 9.165709ms
sum_even: 6.875µs
slow_fib: 3.911584ms
slow_dedup: 7.671333ms
sum_even: 8.083µs
slow_fib: 4.104959ms
slow_dedup: 8.228292ms
```

## Профилирование

См. отдельный [Отчёт по профилированию](REPORT.md).
