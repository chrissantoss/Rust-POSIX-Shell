FROM rust:1.74-bullseye
WORKDIR /usr/src/shell
COPY . .
RUN rm -f Cargo.lock && cargo build
CMD ["./target/debug/shell"] 