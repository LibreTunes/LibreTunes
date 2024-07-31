FROM registry.mregirouard.com/libretunes/ops/docker-leptos/musl:latest as builder

WORKDIR /app

# Install a few dependencies
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
		npm; \
	rm -rf /var/lib/apt/lists/*

RUN npm install tailwindcss@3.1.8 -g

# Copy project dependency manifests
COPY Cargo.toml Cargo.lock /app/

# Add the target to Cargo.toml so we can statically link:
RUN echo 'bin-target-triple = "x86_64-unknown-linux-musl"' >> Cargo.toml

# Create dummy files to force cargo to build the dependencies
RUN mkdir /app/src && mkdir /app/style && mkdir /app/assets && \
		echo "fn main() {}" | tee /app/src/build.rs > /app/src/main.rs && \
		touch /app/src/lib.rs && \
		touch /app/style/main.scss

# Prebuild dependencies
RUN cargo-leptos build --release --precompress

RUN rm -rf /app/src /app/style /app/assets
COPY style /app/style

# Minify CSS
RUN npx tailwindcss -i /app/style/main.scss -o /app/style/main.scss --minify

COPY ascii_art.txt /app/ascii_art.txt
COPY assets /app/assets
COPY src /app/src
COPY migrations /app/migrations

# Touch files to force rebuild
RUN touch /app/src/main.rs && touch /app/src/lib.rs && touch /app/src/build.rs

# Actually build the binary
RUN cargo-leptos build --release --precompress

# Build the final image
FROM scratch

LABEL license="MIT"
LABEL description="LibreTunes, an open-source browser audio player and \
library manager built for collaborative listening."

# Copy the binary and the compressed assets to the "site root"
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/libretunes /libretunes
COPY --from=builder /app/target/site /site

# Configure Leptos settings
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV LEPTOS_SITE_ROOT=/site
EXPOSE 3000

ENTRYPOINT [ "/libretunes" ]
