# ------------- 빌드 ------------
FROM rust:1.98 AS builder
WORKDIR /app

# 의존성만 먼저 빌드해서 레이어 캐시
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo 'fn main() {}' > src/main.rs \
    && cargo build --release \
    && rm -rf src

# 진짜 소스 복사 후 빌드
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ------------- 실행 ------------
FROM debian:trixie-slim
RUN apt-get update \ 
    && apt-get install -y --no-install-recommends ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/sambook /app/sambook

CMD ["/app/sambook"]