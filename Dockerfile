# Многоступенчатый Dockerfile для OVIS Sandbox
# Этап 1: Сборка Rust-приложения
FROM rust:1.75-slim AS builder

WORKDIR /usr/src/ovis-sandbox

# Установка системных зависимостей (pkg-config, OpenSSL)
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Копируем манифесты для кэширования зависимостей
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch --locked

# Копируем исходный код и собираем релизный бинарник
COPY src ./src
RUN cargo build --release

# Этап 2: Финальный легковесный образ
FROM debian:bookworm-slim

WORKDIR /app

# Устанавливаем минимальные runtime-зависимости (OpenSSL, CA-сертификаты)
RUN apt-get update && \
    apt-get install -y ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Копируем скомпилированный бинарник из builder-образа
COPY --from=builder /usr/src/ovis-sandbox/target/release/ovis-sandbox .

# Команда по умолчанию
CMD ["./ovis-sandbox"]
