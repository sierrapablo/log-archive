FROM rust:latest as builder

RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /usr/src/log-archive
COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y cron libssl1.1

RUN mkdir -p /archived_logs

COPY --from=builder /usr/src/log-archive/target/release/log-archive /usr/local/bin/log-archive

COPY crontab /etc/cron.d/log-archive-cron

RUN chmod 0644 /etc/cron.d/log-archive-cron

RUN touch /var/log/cron.log

VOLUME /archived_logs

CMD cron && tail -f /var/log/cron.log
