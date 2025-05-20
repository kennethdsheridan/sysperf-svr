{
  description = "sysperf-svr: musl-static Rust binary + .deb packaging";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # Use musl64 cross compilation environment
        pkgs = import nixpkgs {
          inherit system;
        };

        muslPkgs = pkgs.pkgsCross.musl64;

        sysperf-svr = muslPkgs.rustPlatform.buildRustPackage {
          pname = "sysperf-svr";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

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

        devShells.default = muslPkgs.mkShell {
          packages = [
            muslPkgs.rustc
            muslPkgs.cargo
          ];
        };
      });
}

