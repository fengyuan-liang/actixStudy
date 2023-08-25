# Build stage
FROM rust:latest AS build
# 复制项目根目录到容器中
COPY . /usr/src/app

# 设置工作目录
WORKDIR /usr/src/app

# Install musl-tools
RUN apt update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

# Add the target x86_64-unknown-linux-musl
RUN rustup target add x86_64-unknown-linux-musl

# Copy the Cargo.toml and Cargo.lock files
# COPY webservice/Cargo.toml ./

# Build the dependencies
RUN cargo build --release --target x86_64-unknown-linux-musl

# Final stage
FROM scratch
WORKDIR /app

# Copy the built binary from the build stage
COPY --from=build /usr/src/app/target/x86_64-unknown-linux-musl/release/teacher-service /app/

# Expose the necessary port
EXPOSE 3000

# Set the user for running the application
USER 1000

# Set the entrypoint command
ENTRYPOINT ["./teacher-service"]