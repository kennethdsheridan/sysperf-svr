{ lib, stdenv, dpkg, makeWrapper, sysperf-svr }:

stdenv.mkDerivation {
  pname = "sysperf-svr-deb";
  version = "0.1.0";

  src = sysperf-svr;

  nativeBuildInputs = [ dpkg makeWrapper ];

  installPhase = ''
    mkdir -p $out/DEBIAN
    mkdir -p $out/usr/bin

    cp ${sysperf-svr}/bin/sysperf-svr $out/usr/bin/sysperf-svr

    cat > $out/DEBIAN/control <<EOF
Package: sysperf-svr
Version: 0.1.0
Architecture: amd64
Maintainer: kennethdsheridan@gmail.com
Description: Static sysperf service for Linux.
EOF
  '';

  dontBuild = true;
}

