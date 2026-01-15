# Vigil

**Simple async website availability checker in Rust**

Vigil позволяет проверять доступность сайтов параллельно с использованием асинхронного Rust (`Tokio` + `Reqwest`). Программа поддерживает таймауты, цветной вывод статусов и простую настройку через YAML.

---

## Features

- Асинхронная проверка доступности сайтов
- Опциональный таймаут для каждой проверки
- Цветной терминальный вывод статусов
- Настройка через простой YAML-файл
- Лёгкая интеграция и запуск через CLI

---

## Installation

Склонируйте репозиторий и соберите проект:

```bash
git clone https://github.com/rusyaz/vigil.git
cd vigil
cargo build --release
```

## Usage 

cargo run --config.yaml


---

## Output example 

[ ✅ AVAILABLE ] https://google.com
    The site is available and everything looks good.

[ ⏱ TIMEOUT ] https://example.com
    The request timed out.

[ 💥 SERVER ERROR ] https://brokenserver.com
    Oops! Server error detected.

