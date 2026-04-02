{
  description = "RSS hub dev environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "rss-reader";
        version = "0.1.0";

        src = ./.;

        cargoLock = {
          lockFile = ./Cargo.lock;
        };

        nativeBuildInputs = [ pkgs.pkg-config ];

        buildInputs = [ pkgs.openssl pkgs.sqlite ];
      };

      apps.default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/rss-reader";
      };

      devShells.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          rust-analyzer
          cargo
          pkg-config
          openssl
          sqlite
        ];
      };
    }
  );
}
