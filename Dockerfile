FROM rust:1.72.1
WORKDIR /hello_actix
COPY . /hello_actix/
RUN cargo build --release
EXPOSE $PORT
CMD ./target/release/hello_actix
