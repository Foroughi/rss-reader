{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer   
    pkg-config
    openssl
    sqlite
  ];

  shellHook = ''
    export RUST_BACKTRACE=1
    echo "Rust dev shell ready 🚀"
  '';

}
