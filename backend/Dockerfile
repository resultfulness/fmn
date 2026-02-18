FROM rust:1.93-alpine

RUN cargo install watchexec-cli

WORKDIR /app

CMD [ "watchexec", "-e", "rs,toml", "-r", "--", "cargo", "run" ]
