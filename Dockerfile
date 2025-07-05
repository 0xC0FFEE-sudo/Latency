# --- Build Stage ---
FROM rust:1.77 as builder

# Create a new empty shell project
WORKDIR /usr/src/app
COPY . .

# Build the binaries
RUN cd latency-x-core && cargo build --release

# --- Final Stage ---
FROM debian:bullseye-slim

# Copy the binaries from the builder stage
COPY --from=builder /usr/src/app/latency-x-core/target/release/latency-x-core /usr/local/bin/latency-x-core
COPY --from=builder /usr/src/app/latency-x-core/target/release/pump-trader /usr/local/bin/pump-trader
COPY --from=builder /usr/src/app/latency-x-core/target/release/latency-x-backtester /usr/local/bin/latency-x-backtester

# Copy the configuration file
COPY latency-x-core/Config.toml /etc/latency-x/Config.toml

# Set the working directory
WORKDIR /etc/latency-x

# The entrypoint is just the main trading bot.
# Users can override this to run pump-trader or the backtester.
ENTRYPOINT ["latency-x-core"] 