{ stdenv, dpkg, sysperf-svr }:

stdenv.mkDerivation {
  pname = "sysperf-svr";
  version = "0.1.0";
  src = sysperf-svr;

  buildInputs = [ dpkg ];

  unpackPhase = "true"; # skip unpack

  installPhase = ''
    mkdir -p $out/DEBIAN
    mkdir -p $out/usr/local/bin

    cp -v ${sysperf-svr}/bin/sysperf-svr $out/usr/local/bin/

    cat > $out/DEBIAN/control <<EOF
Package: sysperf-svr
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Kenneth Sheridan <you@example.com>
Description: System performance benchmarking and metrics server built in Rust
EOF
  '';

  buildPhase = ''
    dpkg-deb --build $out $out/sysperf-svr.deb
  '';

  installCheckPhase = ''
    echo "Built .deb package:"
    ls -lh $out/sysperf-svr.deb
  '';

  dontFixup = true;
}

