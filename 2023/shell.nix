{ pkgs ? import <nixpkgs> { } }:
pkgs.mkShell rec {
  buildInputs = with pkgs; [
    rustup
    pkg-config
    # wl-clipboard
  ];
  LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
