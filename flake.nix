{
  description = "sysperf-svr: statically linked musl Rust binary packaged as .deb";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Cross compile for musl
        pkgs = import nixpkgs {
          inherit system;
          crossSystem = {
            config = "x86_64-unknown-linux-musl";
          };
        };

        rustPlatform = pkgs.buildPackages.rustPlatform;

        sysperf-svr = rustPlatform.buildRustPackage {
          pname = "sysperf-svr";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # Enable static build
          cargoBuildFlags = [ "--release" ];
          RUSTFLAGS = "-C target-feature=+crt-static";
          doCheck = false;
        };

        deb = pkgs.stdenv.mkDerivation {
          pname = "sysperf-svr-deb";
          version = "0.1.0";
          src = sysperf-svr;

          nativeBuildInputs = [ pkgs.dpkg ];

          installPhase = ''
            mkdir -p $TMP/deb/usr/bin
            mkdir -p $TMP/deb/DEBIAN

            cp ${sysperf-svr}/bin/sysperf-svr $TMP/deb/usr/bin/

            cat > $TMP/deb/DEBIAN/control <<EOF
Package: sysperf-svr
Version: 0.1.0
Architecture: amd64
Maintainer: kennethdsheridan@gmail.com
Description: Statically linked sysperf-svr service.
EOF

            dpkg-deb --build $TMP/deb $out
          '';

          dontUnpack = true;
          dontBuild = true;
        };

      in {
        packages.default = sysperf-svr;
        packages.deb = deb;

        devShells.default = pkgs.buildPackages.mkShell {
          packages = [
            pkgs.buildPackages.rustc
            pkgs.buildPackages.cargo
          ];
        };
      });
}

