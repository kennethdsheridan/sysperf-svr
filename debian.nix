{ lib, stdenv, dpkg, sysperf-svr }:

stdenv.mkDerivation {
  pname = "sysperf-svr";
  version = "0.1.0";
  src = sysperf-svr;

  nativeBuildInputs = [ dpkg ];

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

    dpkg-deb --build $TMP/deb ${placeholder "out"}
  '';

  dontUnpack = true;
  dontBuild = true;

  outputHashMode = "flat";
  outputHashAlgo = "sha256";
}

