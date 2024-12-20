FROM rust:slim AS builder

WORKDIR /app

RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos@0.2.22

# Install a few dependencies
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
		pkg-config \
		clang \
		build-essential \
		libssl-dev \
		libpq-dev \
		wget; \
	rm -rf /var/lib/apt/lists/*

# Install ImageMagick
RUN cd / && \
	wget https://github.com/ImageMagick/ImageMagick/archive/refs/tags/7.1.1-38.tar.gz && \
	tar xf 7.1.1-38.tar.gz && \
	rm 7.1.1-38.tar.gz && \
	cd ImageMagick-7.1.1-38 && \
	./configure && \
	make install -j $(nproc) && \
	cd .. && \
	rm -rf ImageMagick-7.1.1-38

# Copy project dependency manifests
COPY Cargo.toml Cargo.lock /app/

# Create dummy files to force cargo to build the dependencies
RUN mkdir /app/src && mkdir /app/style && mkdir /app/assets && \
		echo "fn main() {}" | tee /app/src/build.rs > /app/src/main.rs && \
		touch /app/src/lib.rs && \
		touch /app/style/main.scss

# Prebuild dependencies
RUN cargo-leptos build --release --precompress

RUN rm -rf /app/src /app/style /app/assets

COPY ascii_art.txt /app/ascii_art.txt
COPY assets /app/assets
COPY src /app/src
COPY migrations /app/migrations
COPY style /app/style

# Touch files to force rebuild
RUN touch /app/src/main.rs && touch /app/src/lib.rs && touch /app/src/build.rs

# Actually build the binary
RUN cargo-leptos build --release --precompress

# Use ldd to list all dependencies of /app/target/release/libretunes, then copy them to /app/libs
# Setting LD_LIBRARY_PATH is necessary to find the ImageMagick libraries
RUN mkdir /app/libs && LD_LIBRARY_PATH=/usr/local/lib ldd /app/target/release/libretunes | grep "=> /" | \
	awk '{print $3}' | xargs -I '{}' cp '{}' /app/libs

# Build the final image
FROM scratch

LABEL license="MIT"
LABEL description="LibreTunes, an open-source browser audio player and \
library manager built for collaborative listening."

# Copy the binary and the compressed assets to the "site root"
COPY --from=builder /app/target/release/libretunes /libretunes
COPY --from=builder /app/target/site /site

# Copy libraries to /lib64
COPY --from=builder /app/libs /lib64
COPY --from=builder /lib/x86_64-linux-gnu/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2

ENV LD_LIBRARY_PATH=/lib64

# Configure Leptos settings
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_SITE_ROOT=/site
EXPOSE 3000

ENTRYPOINT [ "/libretunes" ]
