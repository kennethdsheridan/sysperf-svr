{
  description = "sysperf-svr: Rust system performance service with .deb and .tar.gz outputs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlay ];
        };

        rust = pkgs.rust-bin.stable.latest.default;

        sysperf-svr = pkgs.rustPlatform.buildRustPackage {
          pname = "sysperf-svr";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          doCheck = false;

          # Static build with musl
          RUSTFLAGS = "-C target-feature=+crt-static";
          buildPhase = ''
            cargo build --release --target x86_64-unknown-linux-musl
          '';

          installPhase = ''
            mkdir -p $out/bin
            cp target/x86_64-unknown-linux-musl/release/sysperf-svr $out/bin/
          '';
        };

        tarball = pkgs.runCommand "sysperf-svr-tarball" {
          nativeBuildInputs = [ pkgs.gzip ];
        } ''
          mkdir -p $out
          mkdir -p build
          cp ${sysperf-svr}/bin/sysperf-svr build/
          tar -czf $out/sysperf-svr.tar.gz -C build sysperf-svr
        '';

        deb = pkgs.callPackage ./debian.nix { inherit sysperf-svr; };
      in {
        packages.default = deb;
        packages.tarball = tarball;
        packages.deb = deb;
      });
}

