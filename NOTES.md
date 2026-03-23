# Основные заметки по проекту.

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
cargo component add miri
```

#### Использование

**Запуск программы и выполнение через интерпретатор**

```shell
cargo miri run
```

**Запуск тестов**

```shell
cargo +nightly miri test
```

Для "ночных сборок":

```shell
cargo +nightly miri test
```

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
valgrind --leak-check=full --show-leak-kinds=all target/debug/demo
```

Здесь:

- `--show-leak-kinds=all` — показывает все типы утечек
- `--leak-check=full` — включает полную проверку утечек памяти

### Sanitizers (nightly)

Инструменты динамического анализа кода (встроенные в компиляторе LLVM),
доступные только в ночных сборках (`nightly`), предназначенные для поиска
ошибок памяти (адресация, утечки) и гонок данных во время исполнения.

См. [Rust Unstable Book]
(https://doc.rust-lang.org/beta/unstable-book/compiler-flags/sanitizer.html).

#### Установка

Рекомендуемый набор: `nightly toolchain` и компонент `rust-src`.

**nightly toolchain**

```shell
rustup toolchain install nightly
```

**rust-src**

```shell
rustup component add rust-src --toolchain nightly
```

#### Использование

Без ключей `-Zbuild-std` и `--target` поведение может быть некорректным,
результаты невалидными.

**ASan**

Детектор ошибок работы с памятью.

Документация: https://clang.llvm.org/docs/AddressSanitizer.html

```shell
RUSTFLAGS="-Zsanitizer=thread" \
cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

**TSan**

Инструмент для обнаружения гонок данных.

Документация: https://clang.llvm.org/docs/ThreadSanitizer.html

```shell
RUSTFLAGS="-Zsanitizer=thread" \
cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu
```

## display_tests.py

Для проекта создан собственный Python-скрипт `display_tests.py`, который
автоматически запускает все варианты тестирования из списка выше (кроме
`gdb/lldb`), а отчёты логирует в файлы (каталог `tests/reports`).

Для изменения настроек можно внести коррективы в набор шагов (`Step`)
в функции `main()`.

### Запуск

```shell
python3 display_tests.py
```
