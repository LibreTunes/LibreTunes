{
  description = "LibreTunes build and development environment";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
    cargo-leptos = {
      url = "github:leptos-rs/cargo-leptos?ref=v0.2.26";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, cargo-leptos, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Build a specific version of cargo-leptos
        cargo-leptos-build = pkgs.rustPlatform.buildRustPackage {
          name = "cargo-leptos";
          buildFeatures = ["no_downloads"];
          src = cargo-leptos; 
          cargoHash = "sha256-9Xvr3qbc7VdpQHie9vI7uwSLwwv6cvMvVmPDrnqPIpY=";

          nativeBuildInputs = with pkgs; [
            pkg-config
            openssl
          ];

          doCheck = false;
        };

        buildPkgs = with pkgs; [
          (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
          cargo-leptos-build
          clang
          openssl
          postgresql
          imagemagick
          pkg-config
          tailwindcss
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

          buildInputs = with pkgs; buildPkgs ++ [
            diesel-cli
          ];

          shellHook = ''
            set -a
            [[ -f .env ]] && source .env
            set +a
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "libretunes";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];

          nativeBuildInputs = with pkgs; buildPkgs ++ [
            makeWrapper
          ];

          buildInputs = with pkgs; [
            openssl
            imagemagick
          ];

          # TODO enable --release builds
          # Creates an issue with cargo-leptos trying to create cache directories
          # See https://github.com/leptos-rs/cargo-leptos/issues/79
          buildPhase = ''
            cargo-leptos build --precompress #--release
          '';

          installPhase = ''
            mkdir -p $out/bin
            install -t $out target/debug/libretunes
            cp -r target/site $out/site

            makeWrapper $out/libretunes $out/bin/libretunes \
              --set LEPTOS_SITE_ROOT $out/site
          '';

          doCheck = false;
        };
      }
    );
}
