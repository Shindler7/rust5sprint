"""
Локальная утилита для более элегантного вывода.

Команда вдохновения (Linux, Mac):

```
cargo test -- --nocapture &> tests/reports/20260323_123601.log
```
"""

import subprocess
from datetime import datetime, timezone
from pathlib import Path

LOG_DIR = Path("tests/reports")


def make_log_file() -> Path:
    """Собрать имя для log-файла."""

    log_dir = Path.cwd() / LOG_DIR
    log_dir.mkdir(parents=True, exist_ok=True)
    ts = datetime.now(timezone.utc).strftime("%Y%m%d_%H%M%S")
    return log_dir / f"{ts}.log"


def run_tests(log_file: Path) -> int:
    """Обеспечить запуск и логирование тестов."""

    cmd = ["cargo", "test", "--", "--nocapture"]
    print("Выполняется команда:", " ".join(cmd))
    print("Лог:", log_file)

    with log_file.open("w", encoding="utf-8") as f:
        result = subprocess.run(cmd,
                                stdout=f,
                                stderr=subprocess.STDOUT,
                                text=True)

    return result.returncode


def main() -> None:
    log_file = make_log_file()
    code = run_tests(log_file)

    if code == 0:
        print("Тесты завершились успешно")
    else:
        print(f"Тесты завершились с кодом {code}")

    raise SystemExit(code)


if __name__ == "__main__":
    main()
