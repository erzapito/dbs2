FROM rust:1.75 as  build

RUN USER=root cargo new --bin dbs2
WORKDIR /dbs2
COPY ./backend/Cargo.lock ./Cargo.lock
COPY ./backend/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm target/release/dbs2*
RUN rm src/*.rs
COPY ./backend/src ./src
RUN cargo build --release

FROM debian:stable-slim
RUN apt update
RUN apt upgrade -y
RUN apt install -y libsqlite3-0 libmariadb3
COPY --from=build /dbs2/target/release/dbs2 .
CMD ["/dbs2"]
