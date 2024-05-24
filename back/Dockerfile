FROM rust:latest as builder

WORKDIR /usr/src/back

COPY migrations migrations
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
COPY diesel.toml .

#TODO Change to variable
ENV DATABASE_URL=postgres://postgres_username:postgres_password@postgres:5432/postgres_name

RUN apt-get update \
    && apt-get install -y libpq-dev \
    && cargo install diesel_cli --no-default-features --features postgres

ENV DATABASE_URL=postgres://postgres_username:postgres_password@postgres:5432/postgres_name
RUN cargo build

ENV DATABASE_URL=postgres://postgres_username:postgres_password@postgres:5432/postgres_name
RUN diesel migration run

ENV DATABASE_URL=postgres://postgres_username:postgres_password@postgres:5432/postgres_name
EXPOSE 8000

ENV DATABASE_URL=postgres://postgres_username:postgres_password@postgres:5432/postgres_name
# TODO: Change to release
CMD ["cargo", "run"]