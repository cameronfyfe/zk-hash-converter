{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { self, ... }:
    (inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let

        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };

        rust-config = {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override rust-config;

        shellPkgs = [
          rust
        ] ++ (with pkgs; [
          bc
          ets
          just
          nixpkgs-fmt
          openssl
          pkg-config
          present-cli
          rustup
        ]);

      in
      rec {

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = shellPkgs;
          };
        };

      }));
}
