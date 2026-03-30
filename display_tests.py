"""
Локальная утилита для более элегантного вывода.

Команда вдохновения (Linux, Mac):

```
cargo test -- --nocapture &> tests/reports/20260323_123601.log
```
"""

from __future__ import annotations

import os
import platform
import shutil
import subprocess
from dataclasses import dataclass
from datetime import datetime, timezone
from enum import StrEnum
from io import TextIOWrapper
from pathlib import Path

LOG_DIR = Path('artifacts/logs')
LOG_FILENAME = '{timestamp}_tests.log'


class OS(StrEnum):
    """Доступные ОС для идентификации."""
    WINDOWS = 'windows'
    LINUX = 'linux'
    MACOS = 'darwin'
    UNKNOW = 'unknow'

    @classmethod
    def detect_os(cls) -> OS:
        """Определить тип системы и вернуть элемент перечисления."""

        os_sys = platform.system().lower()
        for key in vars(cls):
            if key.islower() or key == 'UNKNOW': continue

            os_attr: OS = getattr(OS, key)
            if os_attr.value == os_sys:
                return os_attr

        return OS.UNKNOW

    def get_rust_target(self) -> str | None:
        """Определить target на основе ОС."""
        match self:
            case self.LINUX:
                return 'x86_64-unknown-linux-gnu'
            case self.WINDOWS:
                return 'x86_64-pc-windows-msvc'
            case self.MACOS:
                if platform.machine().lower() == 'arm64':
                    return 'aarch64-apple-darwin'
                else:
                    return 'x86_64-apple-darwin'
            case _:
                return None


"""Идентифицированный тип системы."""
OS_NAME: OS = OS.UNKNOW


@dataclass(frozen=True)
class Step:
    """Шаги тестирования."""
    name: str
    cmd: list[str]
    env_overrides: dict[str, str] | None = None
    linux_only: bool = False
    ignore: bool = False

    def run(self, log_file: Path) -> int | None:
        """Выполнить тест комплексно (с логированием)."""

        try:
            self._self_validate()
        except RuntimeError as err:
            self.print_skip(str(err))
            return None

        self.print_run()

        with log_file.open('a', encoding='utf-8') as log:
            self.info_to_log(log)
            log.flush()
            code = self.exec_me(log)
            log.flush()

            return code

    def exec_me(self,
                log: TextIOWrapper | None = None
                ) -> int:
        """Выполнить запрос в командной строке."""

        env = os.environ.copy()
        if self.env_overrides:
            env.update(self.env_overrides)

        result = subprocess.run(
            self.cmd,
            stdout=log,
            stderr=subprocess.STDOUT,
            text=True,
            env=env,
        )

        result_code = result.returncode
        if log:
            log.write(f'\n[EXIT CODE] ({result_code})\n')
        return result_code

    def _self_validate(self):
        """Провести самопроверку."""

        if self.linux_only and OS_NAME != OS.LINUX:
            raise RuntimeError('только для Linux')
        if not tool_exists(self.cmd[0]):
            raise RuntimeError(f'не найден инструмент: {self.cmd[0]}')

    def info_to_log(self, log: TextIOWrapper):
        """Внести запись-шапку в открытый log-файл."""

        def stars_line(br: bool = True):
            return f'{"=" * 79}{"\n" if br else ""}'

        log.write(f'\n{stars_line()}')
        log.write(f'STEP: {self.name}\n')
        log.write(f'CMD: {" ".join(self.cmd)}\n')
        if self.env_overrides:
            log.write(f'ENV: {self.env_overrides}\n')
        log.write(f'{stars_line()}')

    def print_run(self, message: str | None = None):
        """Напечатать информацию о запуске теста."""
        StatusTest.RUN.print_status(self.name, message)

    def print_skip(self, message: str | None = None):
        """Напечатать информацию о пропуске теста."""
        StatusTest.SKIP.print_status(self.name, message)


class StatusTest(StrEnum):
    """Статусы при запуске тестов."""
    RUN = 'run'
    SKIP = 'skip'

    def print_status(self, step_name: str, message: str | None = None):
        msg = f'[{self.upper()} {step_name}]'
        if message:
            msg = msg + f': {message}'
        print(msg)


def timestamp() -> str:
    """Создать временную метку."""
    return datetime.now(timezone.utc).strftime('%Y%m%d_%H%M%S')


def make_log_file() -> Path:
    """Собрать имя для log-файла."""

    log_dir = Path.cwd() / LOG_DIR
    log_dir.mkdir(parents=True, exist_ok=True)
    return log_dir / LOG_FILENAME.format(timestamp=timestamp())


def tool_exists(tool: str | os.PathLike[str]) -> bool:
    """Проверить существование вызываемого инструмента."""
    tool_str = os.fspath(tool)
    return shutil.which(tool_str) is not None  # noqa


def detect_target() -> str:
    """Определить `target` для текущей ОС."""

    target = OS_NAME.get_rust_target()
    if target is None:
        raise RuntimeError('Неподдерживаемая платформа')

    return target


def main() -> None:
    log_file = make_log_file()
    print(f'Запуск тестов. Результаты будут записаны в файл:\n {log_file}')

    # Обеспечить правильную работу ASan и TSan.
    key_test_args = ['--target', detect_target(), '--tests']
    cmd = ['cargo', '+nightly', 'test', '-Zbuild-std=std', *key_test_args]

    steps: list[Step] = [
        Step(
            name='cargo test -- --nocapture',
            cmd=['cargo', 'test', '--', '--nocapture'],
            ignore=False
        ),
        Step(
            name='cargo +nightly miri test',
            cmd=['cargo', '+nightly', 'miri', 'test'],
            ignore=False,
        ),
        Step(
            name='valgrind --leak-check=full cargo test --tests',
            cmd=['valgrind', '--leak-check=full', 'cargo', 'test', '--tests'],
            linux_only=True,
            ignore=False,
        ),
        Step(
            name='valgrid (...) target/debug/demo',
            cmd=['valgrind', '-s', '--leak-check=full',
                 '--show-leak-kinds=all', 'target/debug/demo'],
            ignore=False
        ),
        Step(
            name='ASan (nightly)',
            cmd=cmd,
            env_overrides={
                'RUSTFLAGS': '-Zsanitizer=address',
                'CARGO_TARGET_DIR': 'target/asan'
            },
            ignore=False
        ),
        Step(
            name='TSan (nightly)',
            cmd=cmd + ['--', '--no-capture'],
            env_overrides={
                'RUSTFLAGS': '-Zsanitizer=thread',
                'CARGO_TARGET_DIR': 'target/tsan'
            },
            ignore=False
        ),
    ]

    results: list[tuple[str, str]] = []
    has_fail = False

    for step in steps:
        if step.ignore:
            results.append((step.name, 'IGNORE'))
            continue

        code = step.run(log_file)
        if code is None:
            results.append((step.name, 'SKIP'))
            continue
        if code == 0:
            results.append((step.name, 'OK'))
        else:
            results.append((step.name, f'FAIL ({code})'))
            has_fail = True

    with log_file.open('a', encoding='utf-8') as log:
        log.flush()
        print('\nИтог:')
        log.write('\n\nИтог:\n')
        for name, status in results:
            line = f'- {status:10} {name}'
            print(line)
            log.write(f'{line}\n')

    raise SystemExit(1 if has_fail else 0)


if __name__ == '__main__':
    OS_NAME = OS.detect_os()
    main()
