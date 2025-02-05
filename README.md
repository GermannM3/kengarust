# 1. Установить зависимости
sudo apt install libssl-dev protobuf-compiler

# 2. Собрать проект
cargo build --features "autonomous networking"

# 3. Запустить с автономным режимом
AUTONOMOUS_MODE=true NETWORK_KEY=secret RUST_LOG=info cargo run -- --autonomous

# 4. Открыть интерфейсы:
# - Веб-интерфейс: http://localhost:8080
# - Графана: http://localhost:3000


# KengaRust Autonomic AI

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Автономная нейросетевая система с функциями:
- 🤖 Telegram-интерфейс
- 🌐 Веб-панель управления
- 🧠 Самообучение
- 🌍 Сбор данных из открытых источников

## Запуск
```bash
docker-compose up --build


Доступы:

    Веб-интерфейс: http://localhost:8080

    Графана: http://localhost:3000



### 7. rust-toolchain.toml
```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
# kengarust
