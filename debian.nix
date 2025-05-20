{ lib, stdenv, dpkg, makeWrapper, sysperf-svr }:

stdenv.mkDerivation {
  pname = "sysperf-svr-deb";
  version = "0.1.0";

  src = sysperf-svr;

  nativeBuildInputs = [ dpkg makeWrapper ];

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

  dpkg-deb --build $TMP/deb $out/sysperf-svr_0.1.0_amd64.deb
'';

  dontBuild = true;
}

