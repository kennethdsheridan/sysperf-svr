{
  description = "sysperf-svr: Native Rust service with .deb and .tar.gz packaging";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustPlatform = pkgs.rustPlatform;

        sysperf-svr = rustPlatform.buildRustPackage {
          pname = "sysperf-svr";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

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
        packages.default = sysperf-svr;
        packages.tarball = tarball;
        packages.deb = deb;

        devShells.default = pkgs.mkShell {
          packages = [ pkgs.rustc pkgs.cargo ];
        };
      });
}

