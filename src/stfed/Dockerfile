FROM rust:1.75.0 as build-env
RUN git clone https://github.com/desbma/stfed.git /app
WORKDIR /app
RUN cargo build --release
RUN ls /app/target/release

FROM gcr.io/distroless/cc-debian12
COPY --from=build-env /app/target/release/stfed /
CMD ["./stfed"]