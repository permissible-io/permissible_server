FROM rust:1.36 as build

RUN cargo install cargo-build-deps
WORKDIR /opt/permissible-server
COPY Cargo* ./
COPY src/lib.rs ./src/lib.rs
RUN cargo build-deps --release
COPY src ./src
RUN cargo build --release

FROM rust:1.36
WORKDIR /opt/permissible-server
COPY --from=build /opt/permissible-server/target/release/permissible_server permissible_server
CMD ./permissible_server
