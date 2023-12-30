FROM rust:slim AS builder

WORKDIR /app

# Select the nightly toolchain and add needed targets
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-unknown-linux-musl

# Install a few dependencies
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
		libssl-dev \
		pkg-config \
		npm \
		musl-tools; \
	rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-leptos
RUN npm install tailwindcss -g

# Copy project dependency manifests
COPY Cargo.toml Cargo.lock /app/

# Add the target to Cargo.toml so we can statically link:
RUN echo 'bin-target-triple = "x86_64-unknown-linux-musl"' >> Cargo.toml

# Create dummy files to force cargo to build the dependencies
RUN mkdir /app/src && mkdir /app/style && mkdir /app/assets && \
		echo "fn main() {}" > /app/src/main.rs && \
		touch /app/src/lib.rs && \
		touch /app/style/main.scss

# Prebuild dependencies
RUN cargo-leptos build --release --precompress

RUN rm -rf /app/src /app/style /app/assets
COPY style /app/style

# Minify CSS
RUN npx tailwindcss -i /app/style/main.scss -o /app/style/main.scss --minify

COPY assets /app/assets
COPY src /app/src

# Touch files to force rebuild
RUN touch /app/src/main.rs && touch /app/src/lib.rs

# Actually build the binary
RUN cargo-leptos build --release --precompress

# Build the final image
FROM scratch

LABEL license="MIT"
LABEL description="LibreTunes, an open-source browser audio player and \
library manager built for collaborative listening."

# Copy the binary and the compressed assets to the "site root"
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/libretunes /libretunes
COPY --from=builder /app/target/site /target/site

# Configure Leptos settings
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_SITE_ROOT=/site
EXPOSE 3000

ENTRYPOINT [ "/libretunes" ]
