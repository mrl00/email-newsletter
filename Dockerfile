#chef
FROM  lukemathwalker/cargo-chef:latest as chef

WORKDIR /app

RUN apt update && apt install lld clang -y

#planner
FROM chef as planner 

COPY . .

RUN cargo chef prepare --recipe-path recipe.json


#builder
FROM chef as builder

COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release --bin email_newsletter


#runtime
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y lld clang libc6 \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/email_newsletter email_newsletter

COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./email_newsletter"]
