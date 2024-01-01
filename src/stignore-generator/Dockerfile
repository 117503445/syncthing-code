FROM rust:1.75.0 as build-env
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo fetch
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=build-env /app/target/release/stignore-generator .
ENTRYPOINT [ "./stignore-generator" ]