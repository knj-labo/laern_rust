FROM rust:1.61.0 AS builder

RUN useradd -ms /bin/bash pokemon
USER pokemon
ENV USER=pokemon

RUN mkdir /home/pokemon/server
WORKDIR /home/pokemon/server

RUN cargo init --vcs none
COPY server/Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
RUN rm target/release/deps/pokemon*

COPY server/src ./src
RUN cargo build --release

EXPOSE 4000