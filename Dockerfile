FROM rustlang/rust:nightly-slim
ENV HASHSTORAGE_HOST=0.0.0.0
WORKDIR /app
EXPOSE 8080
COPY . .
RUN cargo build --release
CMD ./target/release/hashstorage
