{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.libiconv
    pkgs.openssl
    pkgs.pkgconfig
  ];
}