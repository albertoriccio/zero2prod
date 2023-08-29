#Builder stage
# We use the latest Rust stable release as base image
FROM rust:1.70.0-slim as builder
# Let's switch our working directory to `app` (equivalent to `cd app`)
# The `app` folder will be created for us by Docker in case it does not
# exist already.
WORKDIR /app
# Install the required system dependencies for our linking configuration
RUN apt update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt install lld clang -y \
    && apt install pkg-config -y \
    && apt install libssl-dev -y

# Copy all files from our working environment to our Docker image
COPY . .
ENV SQLX_OFFLINE true
# Let's build our binary!
# We'll use the release profile to make it faaaast
RUN cargo build --release


#Runtime stage
FROM rust:1.70.0-slim as runtime
WORKDIR /app
# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/zero2prod zero2prod
# We need the configuration file at runtime!
COPY configuration configuration
ENV APP_ENVIRONMENT production
# When `docker run` is executed, launch the binary!
ENTRYPOINT ["./zero2prod"]