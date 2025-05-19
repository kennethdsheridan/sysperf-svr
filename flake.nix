 {
  description = "sysperf-svr: Static musl Rust service with .deb and .tar.gz packaging";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        rustPlatform = pkgs.makeRustPlatform {
          cargo = pkgs.cargo;
          rustc = pkgs.rustc;
        };

        sysperf-svr = rustPlatform.buildRustPackage {
          pname = "sysperf-svr";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [
            pkgs.pkg-config
            pkgs.musl
          ];

          cargoBuildFlags = [
            "--target=x86_64-unknown-linux-musl"
          ];

          RUSTFLAGS = "-C target-feature=+crt-static";

          installPhase = ''
            mkdir -p $out/bin
            cp target/x86_64-unknown-linux-musl/release/sysperf-svr $out/bin/
          '';

          doCheck = false;
        };

        tarball = pkgs.runCommand "sysperf-svr-tarball" {
          nativeBuildInputs = [ pkgs.gzip ];
        } ''
          mkdir -p $out
          cp ${sysperf-svr}/bin/sysperf-svr ./sysperf-svr
          tar -czf $out/sysperf-svr.tar.gz sysperf-svr
        '';

        deb = pkgs.callPackage ./debian.nix { inherit sysperf-svr; };

      in {
        packages.default = deb;
        packages.tarball = tarball;
        packages.deb = deb;
      });
}

